//! Static content handlers.
//!
//! This module provides endpoints for:
//! - Version information

use axum::Json;
use serde_json::{json, Value};

#[utoipa::path(
    get,
    path = "/status/version",
    tag = "health",
    responses(
        (status = 200, description = "Version information", body = serde_json::Value)
    )
)]
pub async fn version_handler() -> Json<Value> {
    Json(json!({
        "version": env!("CARGO_PKG_VERSION"),
        "name": env!("CARGO_PKG_NAME"),
    }))
}
