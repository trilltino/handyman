//! Payment webhook handling for Stripe
//!
//! This module processes Stripe webhook events for payment confirmations.

use crate::web::handlers::payment::stripe_webhook_handler;
use crate::web::handlers::static_content::stripe_config_handler;
use axum::routing::{get, post};
use axum::Router;
use lib_core::model::ModelManager;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/webhooks/stripe", post(stripe_webhook_handler))
        .route("/stripe-config", get(stripe_config_handler))
        .with_state(mm)
}
