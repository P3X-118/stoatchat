//! In-chat Bluesky session broker (server-to-server proxy to pds-pro).
//!
//! `GET /auth/atproto/session` authenticates the chat user with the authifier
//! `Account` guard (which yields their verified email), then calls the internal
//! pds-pro broker over the **shared host docker network** (bearer `broker_secret`,
//! never browser-facing) to mint an atproto session for that user's OWN linked
//! `<handle>.cooey.club` account. The returned session lets the chat client read
//! the user's Bluesky timeline directly against their PDS — no second login.
//!
//! ## Why a server-side proxy (not a direct browser → pds-pro call)
//!
//! The browser only holds a chat session; it has no pds-pro credential. delta is
//! the trusted party that can authenticate the chat user and vouch for their
//! email. The broker secret is "log in as this user" — it must stay internal, so
//! it lives only in delta's config and travels delta → pds-pro on the host's
//! docker bridge, never out to the public internet. pds-pro resolves the handle
//! from the Authentik record (never from caller input), so the response only ever
//! contains the caller's own account.
//!
//! Mounted at `/auth/atproto`; JSON, kept out of the okapi document (called via a
//! raw fetch from the SPA, like the OIDC session helpers).

use authifier::models::Account;
use revolt_config::config;
use revolt_result::{create_error, Result};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

/// atproto session material returned to the chat client (camelCase on the wire,
/// matching atproto's own `accessJwt`/`refreshJwt` naming).
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AtprotoSession {
    pub access_jwt: String,
    pub refresh_jwt: String,
    pub did: String,
    pub handle: String,
    /// PDS host the session is valid against, e.g. `https://cooey.club`.
    pub pds: String,
}

/// Request body sent to the pds-pro broker.
#[derive(Serialize)]
struct BrokerRequest<'a> {
    email: &'a str,
}

/// pds-pro broker response (camelCase on the wire).
#[derive(Deserialize)]
struct BrokerResponse {
    #[serde(rename = "accessJwt")]
    access_jwt: String,
    #[serde(rename = "refreshJwt")]
    refresh_jwt: String,
    did: String,
    handle: String,
    pds: String,
}

/// `GET /auth/atproto/session` — broker the current user's atproto session.
#[get("/session")]
pub async fn session(account: Account) -> Result<Json<AtprotoSession>> {
    let config = config().await;
    let pds_pro = &config.api.pds_pro;
    if !pds_pro.enabled || pds_pro.base_url.is_empty() {
        return Err(create_error!(NotFound));
    }

    let url = format!(
        "{}/internal/atproto-session",
        pds_pro.base_url.trim_end_matches('/')
    );

    let response = reqwest::Client::new()
        .post(&url)
        .bearer_auth(&pds_pro.broker_secret)
        .json(&BrokerRequest {
            email: &account.email,
        })
        .send()
        .await
        .map_err(|_| create_error!(InternalError))?;

    // 404 from the broker => the user has no active linked atproto account yet.
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(create_error!(NotFound));
    }
    if !response.status().is_success() {
        return Err(create_error!(InternalError));
    }

    let body: BrokerResponse = response
        .json()
        .await
        .map_err(|_| create_error!(InternalError))?;

    Ok(Json(AtprotoSession {
        access_jwt: body.access_jwt,
        refresh_jwt: body.refresh_jwt,
        did: body.did,
        handle: body.handle,
        pds: body.pds,
    }))
}
