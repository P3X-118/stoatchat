use rocket::Route;

mod atproto;
mod oidc;

/// Native authentication routes that sit alongside the authifier mounts.
///
/// These are plain Rocket routes (not part of the OpenAPI spec): they are
/// browser-facing `302` redirect endpoints for the OpenID Connect
/// authorization-code flow, not JSON API endpoints, so they are intentionally
/// kept out of the okapi document.
pub fn routes() -> Vec<Route> {
    routes![oidc::login, oidc::callback]
}

/// In-chat Bluesky session broker (server-to-server proxy to pds-pro). JSON, not
/// part of the okapi document; mounted at `/auth/atproto`.
pub fn atproto_routes() -> Vec<Route> {
    routes![atproto::session]
}
