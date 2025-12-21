//! Static content handlers.
//!
//! This module provides endpoints for:
//! - Version information
//! - Stripe configuration (public keys only)

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

#[utoipa::path(
    get,
    path = "/api/stripe-config",
    tag = "payments",
    responses(
        (status = 200, description = "Stripe configuration", body = serde_json::Value)
    )
)]
pub async fn stripe_config_handler() -> Json<Value> {
    let config = crate::config::app_config();
    Json(json!({
        "product_id": config.stripe.product_id,
        "public_key": config.stripe.public_key,
    }))
}
