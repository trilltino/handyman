//! Request stamping middleware.
//!
//! Adds a unique UUID and timestamp to each request for:
//! - Request correlation across logs
//! - Performance tracking
//! - Debugging

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use lib_utils::time_utils::now_utc;
use time::OffsetDateTime;
use uuid::Uuid;

/// Request stamp containing UUID and timestamp.
#[derive(Debug, Clone)]
pub struct ReqStamp {
    pub uuid: Uuid,
    pub time_in: OffsetDateTime,
}

impl ReqStamp {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            time_in: now_utc(),
        }
    }
}

impl Default for ReqStamp {
    fn default() -> Self {
        Self::new()
    }
}

/// Middleware to stamp each request with UUID and timestamp.
///
/// This should be the FIRST middleware in the stack.
///
/// # Example
/// ```rust
/// let app = Router::new()
///     .route("/", get(handler))
///     .layer(from_fn(mw_req_stamp_resolver));
/// ```
pub async fn mw_req_stamp_resolver(mut req: Request, next: Next) -> Response {
    let stamp = ReqStamp::new();

    // Store stamp in request extensions for use by other middleware/handlers
    req.extensions_mut().insert(stamp.clone());

    let response = next.run(req).await;

    // Log request completion with timing
    let elapsed = now_utc() - stamp.time_in;
    tracing::debug!(
        "Request {} completed in {}ms",
        stamp.uuid,
        elapsed.whole_milliseconds()
    );

    response
}
