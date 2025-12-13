//! Response mapping middleware.
//!
//! Maps errors to HTTP responses and logs requests.
//! Should be the OUTERMOST middleware.

use super::mw_req_stamp::ReqStamp;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

/// Response mapping middleware.
///
/// Logs all requests with UUID correlation and timing.
///
/// # Example
/// ```rust
/// let app = Router::new()
///     .route("/", get(handler))
///     .layer(from_fn(mw_response_map));
/// ```
pub async fn mw_response_map(req: Request, next: Next) -> Response {
    // Extract request stamp for logging
    let req_stamp = req.extensions().get::<ReqStamp>().cloned();
    let method = req.method().clone();
    let uri = req.uri().clone();

    // Run request
    let response = next.run(req).await;

    // Log request
    let status = response.status();
    if let Some(stamp) = req_stamp {
        let elapsed = lib_utils::time_utils::now_utc() - stamp.time_in;
        tracing::info!(
            uuid = %stamp.uuid,
            method = %method,
            uri = %uri,
            status = %status.as_u16(),
            duration_ms = elapsed.whole_milliseconds(),
            "HTTP request"
        );
    }

    response
}
