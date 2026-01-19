use warp::{Filter, Rejection, Reply, http::StatusCode};
use std::convert::Infallible;
use serde_json::json;

pub const API_KEY: &str = "iaas-secret-key-123";

/// OWASP API-2: BROKEN AUTHENTICATION
/// This filter checks for a valid API Key in the 'x-api-key' header.
pub fn with_auth() -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::header::optional::<String>("x-api-key")
        .and_then(|key: Option<String>| async move {
            match key {
                Some(k) if k == API_KEY => Ok(()),
                _ => Err(warp::reject::custom(SecurityError::Unauthorized)),
            }
        })
        .untuple_one()
}

#[derive(Debug)]
pub enum SecurityError {
    Unauthorized,
}

impl warp::reject::Reject for SecurityError {}

/// OWASP API-8: SECURITY MISCONFIGURATION
/// Custom rejection handler to ensure we don't leak server details in error messages.
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Resource not found")
    } else if let Some(SecurityError::Unauthorized) = err.find() {
        (StatusCode::UNAUTHORIZED, "Invalid or missing API Key")
    } else if let Some(_) = err.find::<warp::reject::PayloadTooLarge>() {
        (StatusCode::PAYLOAD_TOO_LARGE, "Payload too large")
    } else {
        eprintln!("Unhandled error: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "An internal error occurred")
    };

    let json = warp::reply::json(&json!({ "error": message }));
    Ok(warp::reply::with_status(json, code))
}

/// Applies security headers as a Warp wrap.
pub fn apply_security_headers<F: Filter<Extract = (R,), Error = Rejection> + Clone, R: Reply>(
    filter: F,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    filter
        .with(warp::reply::with::header("X-Content-Type-Options", "nosniff"))
        .with(warp::reply::with::header("X-Frame-Options", "DENY"))
        .with(warp::reply::with::header("X-XSS-Protection", "1; mode=block"))
        .with(warp::reply::with::header("Content-Security-Policy", "default-src 'none'"))
}
