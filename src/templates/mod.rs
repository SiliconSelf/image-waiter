//! Contains all page templates rendered by the server

pub(crate) mod index;

/// Responder for Sailfish Templates
#[derive(rocket::Responder)]
#[response(status = 200, content_type = "html")]
pub(crate) struct SailfishResponder(pub(crate) String);
