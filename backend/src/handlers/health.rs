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

use axum::{extract::State, http::StatusCode, Json};
use serde_json::json;
use crate::db::DbPool;

/// Root endpoint - simple API identifier
/// Route: GET /
/// Returns: Plain text "Handyman API Server"
pub async fn root() -> &'static str {
    "Handyman API Server"
}

/// Health check endpoint with server info and deep dependency checks
/// Route: GET /api/health
///
/// Returns JSON with:
/// - status: "healthy" or "degraded"
/// - timestamp: Current UTC time
/// - version: API version
/// - checks: Object containing status of dependencies (database, etc)
pub async fn health_check(
    State(pool): State<DbPool>,
) -> (StatusCode, Json<serde_json::Value>) {
    // Check Database Connection
    let db_status = match pool.get().await {
        Ok(conn) => {
            // connection acquired, try a simple query
            match conn.query_one("SELECT 1", &[]).await {
                Ok(_) => "ok",
                Err(e) => {
                    tracing::error!("Health check database query failed: {}", e);
                    "error_query"
                }
            }
        },
        Err(e) => {
            tracing::error!("Health check failed to acquire database connection: {}", e);
            "error_connection"
        }
    };

    // Determine overall status
    let status = if db_status == "ok" { "healthy" } else { "degraded" };
    let http_status = if db_status == "ok" { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE };

    (
        http_status,
        Json(json!({
            "status": status,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "version": env!("CARGO_PKG_VERSION"),
            "checks": {
                "database": db_status
            }
        }))
    )
}
