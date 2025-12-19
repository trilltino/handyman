//! # HTTP Middleware Pipeline
//!
//! Configures the middleware stack for request/response processing.
//!
//! ## Middleware Stack (Applied Order)
//!
//! Middleware is applied in reverse order (last applied = innermost = processed first for requests):
//!
//! 1. **Compression** - GZIP response bodies for efficiency
//! 2. **Tracing** - Log all HTTP requests/responses with duration
//! 3. **CORS** - Handle cross-origin requests
//!
//! ```text
//! Request
//!     ↓
//! [Compression]
//!     ↓
//! [Tracing]
//!     ↓
//! [CORS]
//!     ↓
//! Route Handler
//!     ↓
//! [CORS]
//!     ↓
//! [Tracing]
//!     ↓
//! [Compression]
//!     ↓
//! Response
//! ```
//!
//! ## Middleware Details
//!
//! ### CORS (Cross-Origin Resource Sharing)
//! - **Allowed Origins**: All (`*`)
//! - **Allowed Methods**: GET, POST, PUT, DELETE
//! - **Allowed Headers**: Content-Type, Authorization
//! - **Max Age**: 3600 seconds (1 hour)
//!
//! ### Tracing
//! - Logs HTTP method, path, status, and duration
//! - Uses structured logging via `tracing` crate
//!
//! ### Compression
//! - Compresses response bodies with GZIP
//! - Reduces bandwidth usage
//! - Automatically detects Accept-Encoding header

use axum::http::{header, HeaderValue, Method};
use std::time::Duration;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;

/// Apply middleware pipeline to an Axum router.
///
/// Adds the complete middleware stack in the correct order for
/// request handling, tracing, and response optimization.
pub fn apply_middleware<S>(app: axum::Router<S>) -> axum::Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    app.layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
            .max_age(Duration::from_secs(3600)),
    )
    .layer(TraceLayer::new_for_http())
    .layer(CompressionLayer::new())
    // Security Headers
    .layer(SetResponseHeaderLayer::overriding(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    ))
    .layer(SetResponseHeaderLayer::overriding(
        header::X_FRAME_OPTIONS,
        HeaderValue::from_static("DENY"),
    ))
    .layer(SetResponseHeaderLayer::overriding(
        header::STRICT_TRANSPORT_SECURITY,
        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    ))
    .layer(SetResponseHeaderLayer::overriding(
        header::X_XSS_PROTECTION,
        HeaderValue::from_static("1; mode=block"),
    ))
}
