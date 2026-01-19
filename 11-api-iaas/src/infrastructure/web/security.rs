use warp::{Filter, Rejection, Reply, http::StatusCode};
use std::convert::Infallible;
use serde_json::json;

/// SECURITY MODULE
/// 
/// --- Good to know ---
/// This module implements OWASP Top 10 API Security protections.
/// SOLID: By moving security logic here, we keep our `mod.rs` clean and focused.

pub const API_KEY: &str = "iaas-secret-key-123";

/// OWASP API-2: BROKEN AUTHENTICATION
/// 
/// This "Filter" acts like a piece of Middleware. It checks for a secure header
/// before allowing the request to reach the logic.
/// 
/// Comparison:
/// - Go: Like a Middleware function wrapping a `http.Handler`.
/// - Python: Similar to a FastAPI `Depends` dependency or a Flask decorator.
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

/// OWASP API-8: SECURITY MISCONFIGURATION (Error Handling)
/// 
/// In Rust/Warp, we use a "Rejection" handler to transform internal failures
/// into clean, sanitized JSON responses.
/// 
/// Why: We never want to leak database strings or stack traces to an attacker.
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Resource not found")
    } else if let Some(SecurityError::Unauthorized) = err.find() {
        (StatusCode::UNAUTHORIZED, "Invalid or missing API Key")
    } else if let Some(_) = err.find::<warp::reject::PayloadTooLarge>() {
        (StatusCode::PAYLOAD_TOO_LARGE, "Payload too large")
    } else {
        // We log the error internally for us to debug...
        eprintln!("Unhandled error: {:?}", err);
        // ...but we only send a generic "Internal Error" to the user.
        (StatusCode::INTERNAL_SERVER_ERROR, "An internal error occurred")
    };

    let json = warp::reply::json(&json!({ "error": message }));
    Ok(warp::reply::with_status(json, code))
}

/// OWASP API-8: SECURITY MISCONFIGURATION (Secure Headers)
/// 
/// This wraps our entire API and adds standard security headers to every response.
pub fn apply_security_headers<F: Filter<Extract = (R,), Error = Rejection> + Clone, R: Reply>(
    filter: F,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    filter
        .with(warp::reply::with::header("X-Content-Type-Options", "nosniff"))
        .with(warp::reply::with::header("X-Frame-Options", "DENY"))
        .with(warp::reply::with::header("X-XSS-Protection", "1; mode=block"))
        .with(warp::reply::with::header("Content-Security-Policy", "default-src 'none'"))
}
