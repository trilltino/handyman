//! # Health Check Handlers
//!
//! ## Purpose
//! Provides API health check and status endpoints.
//! Used for monitoring, load balancer health checks, debugging.
//!
//! ## Endpoints
//! - **GET /**: Root endpoint showing API name
//! - **GET /api/health**: Health check with timestamp and version
//!
//! ## Relation to Entire Program
//! - **Public endpoints**: No authentication required
//! - **Used by**: Monitoring systems, deployment health checks
//! - **Future**: Will add database ping, cache status, queue depth

use axum::{http::StatusCode, Json};  // Axum HTTP types
use serde_json::json;                 // JSON response helper

/// Root endpoint - simple API identifier
/// Route: GET /
/// Returns: Plain text "Handyman API Server"
pub async fn root() -> &'static str {
    "Handyman API Server"
}

/// Health check endpoint with server info
/// Route: GET /api/health
///
/// Returns JSON with:
/// - status: "healthy" (always, for now)
/// - timestamp: Current UTC time (RFC3339 format)
/// - version: API version from Cargo.toml
///
/// # Future Enhancements
/// - Check database connection
/// - Check Redis cache connection
/// - Check Stripe API connectivity
/// - Report queue depth for background jobs
pub async fn health_check() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "version": env!("CARGO_PKG_VERSION"),  // Compile-time version from Cargo.toml
        }))
    )
}
