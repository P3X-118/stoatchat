# Authentik OIDC integration (SGC / Cooey Club)

Status: **design + scaffold** on branch `feat/authentik-oidc`. Implementation is
deploy-gated — the production host (`yo.yeet.fm`) and the mesh-internal IdP
(`auth.cooey.club`) are not reachable from the dev environment, so the code is
written and `cargo check`'d locally but live-tested later.

## Goal

Authenticate **normal chat users via Authentik** (the estate IdP already serving
`auth.cooey.club`) instead of the built-in email/password system. Keep exactly
**one local break-glass super-admin**: `oneill@sgc.ai` (the built-in
email/password path stays mounted, used only by this account).

Decision (confirmed): **native OIDC client built into `delta`** that mints an
`authifier` session on callback. Rejected alternatives: Traefik forward-auth
(can't satisfy Revolt's per-user session + WebSocket token model) and a
standalone bridge (would duplicate authifier internals).

## Reuse: the radio.cooey.club / Owncast precedent

`radio.cooey.club` (Owncast SGC fork, `legitservices/okast`) already does exactly
this against the same IdP. Mirror it:

- Authentik Application slug + `CLIENT_ID` = `stoked`
- Issuer: `https://auth.cooey.club/application/o/stoked/`
- `CLIENT_SECRET` derived from `sgc_pgsk`:
  `password_hash('sha512', 'stoked.oidc', rounds=655555) | to_uuid`
  (Owncast uses the `owncast.oidc` salt; we use `stoked.oidc`.)
- Scopes: `openid profile email`
- Mesh reachability: the public `auth.cooey.club` IP is NOT reachable from the
  app host, so the backend container must pin the IdP to the mesh edge:
  `--add-host=auth.cooey.club:169.254.0.127` (same as Owncast). The user's
  *browser* still uses public DNS for the authorize redirect; only the
  server-side discovery + token exchange use the mesh, while still validating
  the public Let's Encrypt cert for `auth.cooey.club`.

Authentik provisioning is the established `ak shell` Django-ORM pattern on
`pds-authentik-server` (bskypds.pro): copy `scripts/sgc/provision-radio-chat.py`
→ `provision-stoked.py` (OAuth2Provider + Application slug `stoked`, the redirect
URI below, confidential client, the derived secret).

## Backend (delta)

### Config
Add an `[api.oidc]` section, mirroring the captcha pattern in
`crates/core/config/src/lib.rs` (~`ApiSecurityCaptcha`, L191-195):

```toml
[api.oidc]
enabled      = true
issuer       = "https://auth.cooey.club/application/o/stoked/"
client_id    = "stoked"
client_secret = "<derived>"
scopes       = "openid profile email"
redirect_uri = "https://chat.cooey.club/api/auth/oidc/callback"
```

`enabled` false (or empty `issuer`) ⇒ OIDC off (compiled default), exactly like
the SMTP host master-switch.

### Routes
New `crates/delta/src/routes/auth/oidc.rs`, mounted next to the authifier mounts
in `crates/delta/src/routes/mod.rs` (~L36-38, alongside `/auth/account`,
`/auth/session`, `/auth/mfa`):

- `GET /auth/oidc/login` → build the authorization URL (state + PKCE/nonce in a
  short-lived cookie) and 302 to Authentik.
- `GET /auth/oidc/callback` → validate `state`, exchange `code` for tokens
  (server-side, uses `client_secret`), validate the ID token, read the verified
  `email` (+ `preferred_username`/`name`) claims, then:
  1. find-or-create the authifier `Account` by normalised email, and
  2. mint a session, and
  3. 302 back to the SPA with a **one-time exchange code** (NOT the session token
     in the URL); the SPA redeems it for `{ _id, user_id, token }`.

Crate deps (delta): `openidconnect` (preferred — does discovery + ID-token
validation) or `oauth2` + manual JWKS; `reqwest` is already a dependency.

### The authifier seam (no trait changes needed)
All in `rust-authifier`, already implemented on the mongo backend:

- `Account::create_session(&self, authifier, name) -> Session`
  (`crates/authifier/src/impl/account.rs` ~L84-109): generates the token,
  `user_id = account.id`, persists, emits `CreateSession`. **No password check** —
  this is the mint point.
- `Account::new(authifier, email, plaintext_password, verify_email) -> Account`
  (`account.rs` ~L21-81): find-or-create by normalised email; `verify_email=false`
  marks it `Verified`. Password is mandatory in the model, so SSO-only accounts
  get a random throwaway (`hash_password(uuid)`) that is never used.
- Trait methods used: `find_account_by_normalised_email`, `save_account`,
  `save_session` (`crates/authifier/src/database/definition.rs`) — all exist on
  mongo + dummy.
- Get the `Authifier` in delta via the existing `Database::to_authifier()`
  (`crates/core/database/src/drivers/mod.rs:118`).

### Onboarding is unchanged
First login creates the chat `User` lazily through the existing
`/onboard/complete` (`crates/delta/src/routes/onboard/complete.rs` ~L31-54),
which keys off `session.user_id` (= authifier Account id). An OIDC-minted session
flows through it identically to a password user.

## Frontend (for-web, branch `feat/authentik-oidc`)

- `packages/client/components/auth/src/flows/FlowLogin.tsx`: add a
  "Sign in with Cooey" button → `window.location = <API>/auth/oidc/login`.
- New `FlowOIDCCallback` component + route in `packages/client/src/index.tsx`
  (~L143-154): redeem the one-time code, then `state.auth.setSession({_id, token,
  userId})` (`components/client/Controller.ts`) and run the normal post-login
  lifecycle. The SDK/API already attach `X-Session-Token`, so no `stoat.js` /
  `stoat-api` change is needed.
- Optionally hide the email/password form for everyone except a break-glass
  escape hatch (e.g. `/login?local=1`).

## Break-glass (oneill@sgc.ai)

- Platform admin = the `User.privileged` bool
  (`crates/core/database/src/models/users/model.rs:52`, default false L180); no
  programmatic setter — set once in Mongo:
  `db.users.updateOne({_id:"<id>"},{$set:{privileged:true}})`.
- Keep authifier's password login route mounted (break-glass uses it).
- After SSO is verified working, set `[api.registration].invite_only = true`
  (already wired via `stoked_registration_invite_only`) so no NEW local
  email/password accounts can be created; existing break-glass account still
  logs in. Verify break-glass login BEFORE locking down.

## SGC role / host_vars wiring (stoked-ar)

- New `stoked_oidc_*` role vars → render `[api.oidc]` in `revolt-toml.j2`.
- `stoked_api_container_extra_arguments`: `--add-host=auth.cooey.club:169.254.0.127`.
- host_vars `yo.yeet.fm`: enable + the derived secret (reproducible from `sgc_pgsk`,
  not stored).
- A new `legitservices/stoked-*` image built from this branch is required (the
  OIDC routes are compiled in). Bump the role image tag + cut a role release once
  the images are built and pushed.

## Deploy + test sequence (when mesh/host are reachable)

1. `provision-stoked.py` on pds-authentik-server (creates the `stoked` app +
   secret + redirect URI).
2. Build + push the OIDC-enabled `stoked-*` images from this branch.
3. Wire role vars + host_vars; `just setup-service stoked`.
4. Test: login → Authentik → callback → chat session; onboarding for a new SSO
   user; returning user; then set `privileged` on oneill@sgc.ai and verify
   break-glass; finally flip `invite_only=true`.

## Risks / open items

- **Email collision**: an OIDC email that matches an existing password account
  links to it (find-or-create by email). Intended for the migration, but confirm
  the break-glass account's email is not one a normal Authentik user could claim.
- **Token transport**: use a one-time server-issued exchange code, never the
  session token in a redirect URL.
- **Redirect URI** must be registered EXACTLY in Authentik (backend path under
  `/api`), and added to the provider's allowed redirect URIs.
- Live testing requires the SGC mesh (IdP) + the production host — both
  unreachable from the dev environment at authoring time.
