//! Native Authentik (OpenID Connect) login for the chat backend.
//!
//! This implements the OIDC **authorization-code flow** directly in `delta` and
//! mints an `authifier` session on a successful callback, so normal chat users
//! can authenticate against the estate IdP (`auth.cooey.club`) while the
//! built-in email/password path stays mounted for the break-glass super-admin.
//! See `docs/authentik-oidc-integration.md` for the full design.
//!
//! ## Routes (mounted at `/auth/oidc`)
//!
//! * `GET /auth/oidc/login`    – build the authorization URL, drop a short-lived
//!   `state`/`nonce` cookie, and `302` the browser to Authentik.
//! * `GET /auth/oidc/callback` – validate `state`, exchange `code` for tokens
//!   server-side (using the client secret), validate the ID token, read the
//!   verified `email` (+ `preferred_username`/`name`) claims, find-or-create the
//!   authifier `Account`, mint a `Session`, and `302` back to the SPA.
//!
//! ## Token-transport contract (the frontend MUST implement this)
//!
//! On success the callback issues a single `302 Found` redirect to:
//!
//! ```text
//! {config.hosts.app}/login/oidc#token=<session_token>&user_id=<account_id>&session_id=<session_id>
//! ```
//!
//! * The session material is carried in the URL **fragment** (`#...`), never the
//!   query string, so it is not sent to the server, not logged by proxies, and
//!   not stored in browser history the way query params are.
//! * `token`      – the authifier session bearer token (the value sent as
//!   `X-Session-Token` on every subsequent request).
//! * `user_id`    – the authifier account id (== chat `User` id used by
//!   `/onboard/complete` and the WebSocket auth).
//! * `session_id` – the authifier session id (`_id`), for session management.
//!
//! The SPA route at `/login/oidc` parses the fragment, calls
//! `state.auth.setSession({ _id: session_id, token, userId: user_id })`, clears
//! the fragment from the URL, and runs the normal post-login lifecycle (which
//! lazily creates the chat `User` through `/onboard/complete` for first-time SSO
//! users). No `stoat.js` / `stoat-api` change is needed — they already attach
//! `X-Session-Token`.
//!
//! On any failure the callback redirects to
//! `{config.hosts.app}/login/oidc#error=<machine_readable_reason>` so the SPA can
//! surface a friendly message instead of leaking a backend error page.
//!
//! ## Mesh reachability
//!
//! The user's *browser* reaches `auth.cooey.club` over public DNS for the
//! authorize redirect. The *server-side* discovery + token exchange go over the
//! SGC mesh; the container pins the IdP host with
//! `--add-host=auth.cooey.club:169.254.0.127` (see the role), so the issuer host
//! is used verbatim here — IPs are never hardcoded in this module.

use authifier::models::{Account, Session};
use authifier::Authifier;
use revolt_config::config;
use revolt_database::Database;
use revolt_result::{create_error, Error, Result};
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::Redirect;
use rocket::time::Duration;
use rocket::State;
use serde::Deserialize;

/// Name of the short-lived cookie that ties the `/login` request to its
/// `/callback`. Holds the CSRF `state` and the ID-token `nonce`.
const STATE_COOKIE: &str = "revolt_oidc_state";

/// How long the user has to complete the IdP round-trip.
const STATE_COOKIE_TTL_MINUTES: i64 = 10;

/// Opaque per-request anti-forgery material, stored in [`STATE_COOKIE`] and
/// echoed back via the authorization request.
#[derive(serde::Serialize, Deserialize)]
struct OidcState {
    /// CSRF token matched against the `state` query parameter on callback.
    state: String,
    /// Nonce bound into the ID token and re-checked on callback.
    nonce: String,
}

/// Token endpoint response (subset of RFC 6749 / OIDC core we care about).
#[derive(Deserialize)]
struct TokenResponse {
    #[allow(dead_code)]
    access_token: Option<String>,
    id_token: Option<String>,
    #[allow(dead_code)]
    token_type: Option<String>,
    #[allow(dead_code)]
    expires_in: Option<i64>,
}

/// Claims we read out of the verified ID token.
#[derive(Deserialize)]
struct IdTokenClaims {
    /// Issuer — must equal the configured issuer.
    iss: String,
    /// Audience — must contain our `client_id`. Authentik emits a single string
    /// but the spec also allows an array, so accept both.
    #[serde(default)]
    aud: Audience,
    /// Expiry (seconds since epoch). Validated by `jsonwebtoken` itself, so it
    /// is not read directly below.
    #[allow(dead_code)]
    exp: i64,
    /// Nonce — must equal the one we generated for this flow.
    #[serde(default)]
    nonce: Option<String>,
    /// Verified email address. Authentik always sets this with the `email` scope.
    #[serde(default)]
    email: Option<String>,
    /// Whether the IdP considers the email verified.
    #[serde(default)]
    email_verified: Option<bool>,
    /// Short login name, preferred for the chat username seed.
    #[serde(default)]
    preferred_username: Option<String>,
    /// Display name fallback.
    #[serde(default)]
    name: Option<String>,
}

/// `aud` may be a single string or a list of strings.
#[derive(Deserialize, Default)]
#[serde(untagged)]
enum Audience {
    #[default]
    None,
    One(String),
    Many(Vec<String>),
}

impl Audience {
    fn contains(&self, client_id: &str) -> bool {
        match self {
            Audience::None => false,
            Audience::One(value) => value == client_id,
            Audience::Many(values) => values.iter().any(|value| value == client_id),
        }
    }
}

/// OIDC provider metadata we consume from discovery
/// (`<issuer>/.well-known/openid-configuration`). Authentik's authorize/token
/// endpoints are SHARED (`/application/o/authorize/`, `/application/o/token/`) —
/// they are NOT `<issuer>/authorize` — so we must discover them, never derive by
/// convention. (`jwks_uri` IS per-application.)
#[derive(Deserialize)]
struct OidcDiscovery {
    authorization_endpoint: String,
    token_endpoint: String,
    jwks_uri: String,
}

/// Fetch the OIDC discovery document for the issuer. The issuer host is used
/// verbatim (resolved over the SGC mesh by the container's `--add-host`).
async fn discover(issuer: &str) -> std::result::Result<OidcDiscovery, &'static str> {
    let url = format!(
        "{}.well-known/openid-configuration",
        ensure_trailing_slash(issuer)
    );
    reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .map_err(|_| "discovery_fetch_failed")?
        .json::<OidcDiscovery>()
        .await
        .map_err(|_| "discovery_decode_failed")
}

fn ensure_trailing_slash(value: &str) -> String {
    if value.ends_with('/') {
        value.to_owned()
    } else {
        format!("{value}/")
    }
}

/// # Begin OIDC login
///
/// Builds the Authentik authorization URL (carrying `state` + `nonce`), stores
/// the matching anti-forgery material in a short-lived cookie, and redirects the
/// browser to the IdP.
#[get("/login")]
pub async fn login(jar: &CookieJar<'_>) -> Result<Redirect> {
    let config = config().await;
    let oidc = config.api.oidc.clone();

    if !oidc.enabled || oidc.issuer.is_empty() {
        return Err(create_error!(NotFound));
    }

    // Generate anti-forgery material for this flow.
    let state = nanoid::nanoid!(32);
    let nonce = nanoid::nanoid!(32);

    let payload = OidcState {
        state: state.clone(),
        nonce: nonce.clone(),
    };

    let serialised = serde_json::to_string(&payload).map_err(|_| create_error!(InternalError))?;

    // Short-lived, http-only state cookie. `SameSite::Lax` is required so the
    // cookie survives the top-level GET redirect back from the IdP.
    //
    // NOTE: a plain (not `add_private`) cookie is used deliberately — delta does
    // not enable Rocket's `secrets` feature, so private/encrypted cookies are not
    // available. This is safe here: the cookie only holds single-use, random
    // anti-forgery values (`state`/`nonce`) that are validated server-side, so
    // there is nothing confidential to protect and tampering is caught by the
    // state/nonce mismatch checks on callback.
    let mut cookie = Cookie::new(STATE_COOKIE, serialised);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_path("/");
    cookie.set_max_age(Duration::minutes(STATE_COOKIE_TTL_MINUTES));
    // TODO: gate `cookie.set_secure(true)` on the deployment being https (it is
    // in production: chat.cooey.club). Left unset so local http dev still works.
    jar.add(cookie);

    // Discover the IdP endpoints (Authentik's authorize endpoint is a shared
    // path, not <issuer>/authorize), then build the authorization request.
    let discovery = discover(&oidc.issuer)
        .await
        .map_err(|_| create_error!(InternalError))?;
    let mut url = url::Url::parse(&discovery.authorization_endpoint)
        .map_err(|_| create_error!(InternalError))?;
    url.query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("client_id", &oidc.client_id)
        .append_pair("redirect_uri", &oidc.redirect_uri)
        .append_pair("scope", &oidc.scopes)
        .append_pair("state", &state)
        .append_pair("nonce", &nonce);

    Ok(Redirect::to(url.to_string()))
}

/// # Complete OIDC login (IdP callback)
///
/// Validates `state`, exchanges `code` for tokens server-side, validates the ID
/// token, resolves the verified email to an authifier account (find-or-create),
/// mints a session, and redirects the browser back to the SPA carrying the
/// session in the URL fragment (see the module doc for the exact contract).
#[get("/callback?<code>&<state>&<error>")]
pub async fn callback(
    db: &State<Database>,
    jar: &CookieJar<'_>,
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
) -> Redirect {
    let config = config().await;
    let app = config.hosts.app.clone();
    let oidc = config.api.oidc.clone();

    // Always consume the state cookie so it cannot be replayed. The removal
    // cookie carries the same path the login handler set, so the browser clears
    // the right one.
    let stored = jar.get(STATE_COOKIE).cloned();
    if stored.is_some() {
        let mut removal = Cookie::new(STATE_COOKIE, "");
        removal.set_path("/");
        jar.remove(removal);
    }

    // Run the fallible flow, mapping any failure to a single SPA error redirect.
    match callback_inner(db.inner(), &oidc, &app, stored, code, state, error).await {
        Ok(redirect) => redirect,
        Err(reason) => Redirect::to(format!("{}/login/oidc#error={}", trim_trailing_slash(&app), reason)),
    }
}

/// Trim a single trailing slash so fragment URLs are well-formed.
fn trim_trailing_slash(value: &str) -> String {
    value.strip_suffix('/').unwrap_or(value).to_owned()
}

/// The fallible body of [`callback`]. Returns either the success redirect or a
/// short machine-readable error reason (used as `#error=<reason>`).
#[allow(clippy::too_many_arguments)]
async fn callback_inner(
    db: &Database,
    oidc: &revolt_config::ApiOidc,
    app: &str,
    stored: Option<Cookie<'static>>,
    code: Option<String>,
    state: Option<String>,
    idp_error: Option<String>,
) -> std::result::Result<Redirect, &'static str> {
    if !oidc.enabled || oidc.issuer.is_empty() {
        return Err("oidc_disabled");
    }

    // The IdP can bounce the user back with an error (e.g. access_denied).
    if idp_error.is_some() {
        return Err("idp_error");
    }

    let code = code.ok_or("missing_code")?;
    let state = state.ok_or("missing_state")?;

    // Validate CSRF state against the cookie.
    let stored = stored.ok_or("missing_state_cookie")?;
    let expected: OidcState = serde_json::from_str(stored.value()).map_err(|_| "bad_state_cookie")?;
    if expected.state != state {
        return Err("state_mismatch");
    }

    // Discover the IdP endpoints (shared authorize/token paths in Authentik).
    let discovery = discover(&oidc.issuer).await?;

    // Exchange the authorization code for tokens (server-side, confidential).
    let id_token = exchange_code(oidc, &discovery.token_endpoint, &code).await?;

    // Validate the ID token (signature via the discovered JWKS) + read claims.
    let claims =
        validate_id_token(oidc, &discovery.jwks_uri, &id_token, &expected.nonce).await?;

    let email = claims.email.ok_or("missing_email")?;
    // Reject only an explicit `email_verified: false`. Authentik always sends
    // `true` for confirmed accounts; an absent flag is tolerated because the
    // email was delivered inside a token minted by the trusted IdP.
    if claims.email_verified == Some(false) {
        return Err("email_unverified");
    }

    let display_name = claims
        .preferred_username
        .or(claims.name)
        .unwrap_or_else(|| email.clone());

    // --- authifier seam: find-or-create account, mint session -----------------
    let authifier = db.clone().to_authifier().await;
    let (_account, session) = find_or_create_session(&authifier, &email, display_name)
        .await
        .map_err(|_| "session_failed")?;

    // Success: hand the session back to the SPA via the URL fragment.
    Ok(Redirect::to(format!(
        "{}/login/oidc#token={}&user_id={}&session_id={}",
        trim_trailing_slash(app),
        urlencode(&session.token),
        urlencode(&session.user_id),
        urlencode(&session.id),
    )))
}

/// POST the authorization code to the token endpoint and return the raw ID
/// token JWT. Uses the already-present `reqwest` client; the issuer host is used
/// verbatim (resolved over the mesh by the container's `--add-host`).
async fn exchange_code(
    oidc: &revolt_config::ApiOidc,
    token_endpoint: &str,
    code: &str,
) -> std::result::Result<String, &'static str> {
    let params = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", oidc.redirect_uri.as_str()),
        ("client_id", oidc.client_id.as_str()),
        ("client_secret", oidc.client_secret.as_str()),
    ];

    let response = reqwest::Client::new()
        .post(token_endpoint)
        .form(&params)
        .send()
        .await
        .map_err(|_| "token_request_failed")?;

    if !response.status().is_success() {
        return Err("token_exchange_rejected");
    }

    let body: TokenResponse = response.json().await.map_err(|_| "token_decode_failed")?;
    body.id_token.ok_or("missing_id_token")
}

/// Validate the ID token and return its claims.
///
/// We verify the RS256 **signature** (key fetched from the IdP JWKS and matched
/// by the token header `kid`), plus **issuer**, **audience** (must contain our
/// client_id), **expiry** and **nonce**.
async fn validate_id_token(
    oidc: &revolt_config::ApiOidc,
    jwks_uri: &str,
    id_token: &str,
    expected_nonce: &str,
) -> std::result::Result<IdTokenClaims, &'static str> {
    use jsonwebtoken::{decode, decode_header, jwk::JwkSet, Algorithm, DecodingKey, Validation};

    // Identify the signing key by the token header `kid`.
    let header = decode_header(id_token).map_err(|_| "id_token_bad_header")?;
    let kid = header.kid.ok_or("id_token_no_kid")?;

    // Fetch the IdP JWKS (the discovered jwks_uri, per-application in Authentik).
    // TODO(perf): cache the JWKS per-issuer with a short TTL instead of fetching
    // on every callback (logins are infrequent, so per-call is acceptable here).
    let jwks: JwkSet = reqwest::Client::new()
        .get(jwks_uri)
        .send()
        .await
        .map_err(|_| "jwks_fetch_failed")?
        .json()
        .await
        .map_err(|_| "jwks_decode_failed")?;
    let jwk = jwks.find(&kid).ok_or("jwks_kid_not_found")?;
    let key = DecodingKey::from_jwk(jwk).map_err(|_| "jwks_key_invalid")?;

    // Real RS256 signature verification + standard claim checks (Authentik signs
    // ID tokens with RS256 by default).
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[oidc.issuer.as_str()]);
    validation.set_audience(&[oidc.client_id.as_str()]);
    validation.validate_exp = true;

    let token =
        decode::<IdTokenClaims>(id_token, &key, &validation).map_err(|_| "id_token_invalid")?;
    let claims = token.claims;

    // jsonwebtoken already enforced issuer / audience / expiry via `validation`;
    // re-check audience defensively for the array-vs-string Authentik case and
    // enforce the nonce binding (jsonwebtoken does not understand nonce).
    if claims.iss != oidc.issuer {
        return Err("issuer_mismatch");
    }
    if !claims.aud.contains(&oidc.client_id) {
        return Err("audience_mismatch");
    }
    match &claims.nonce {
        Some(nonce) if nonce == expected_nonce => {}
        _ => return Err("nonce_mismatch"),
    }

    Ok(claims)
}

/// Resolve a verified email to an authifier account (creating one for first-time
/// SSO users) and mint a session.
///
/// Uses the authifier seam exactly as the integration spec prescribes:
///
/// * [`Account::new`] performs find-or-create by *normalised* email (so it reuses
///   authifier's own normalisation and will link an OIDC login to a pre-existing
///   password account with the same email — see the collision note in the
///   report). `verify_email = false` marks the account `Verified` without sending
///   a confirmation email. The password is mandatory in authifier's model, so an
///   SSO-only account is given a random throwaway secret that is never used for
///   login.
/// * [`Account::create_session`] is the password-free mint point — it generates
///   the token, sets `user_id = account.id`, persists the session, and emits the
///   `CreateSession` event consumed by the bonfire WebSocket layer.
async fn find_or_create_session(
    authifier: &Authifier,
    email: &str,
    name: String,
) -> std::result::Result<(Account, Session), Error> {
    // Random throwaway password — never used (the account is OIDC-only unless the
    // user later runs a password reset). `Account::new` hashes this internally.
    let throwaway_password = ulid::Ulid::new().to_string() + &nanoid::nanoid!(32);

    // find-or-create by normalised email; verify_email = false => account Verified.
    let account = Account::new(authifier, email.to_owned(), throwaway_password, false)
        .await
        .map_err(|_| create_error!(InternalError))?;

    // Mint a session (no password check — this is the SSO mint point).
    let session = account
        .create_session(authifier, name)
        .await
        .map_err(|_| create_error!(InternalError))?;

    Ok((account, session))
}

/// Percent-encode a value for safe inclusion in the redirect fragment.
fn urlencode(value: &str) -> String {
    url::form_urlencoded::byte_serialize(value.as_bytes()).collect()
}
