//! Payment webhook handling for Stripe
//!
//! This module processes Stripe webhook events for payment confirmations.

use axum::routing::post;
use axum::Router;
use lib_core::model::ModelManager;
use crate::web::handlers::payment::stripe_webhook_handler;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/webhooks/stripe", post(stripe_webhook_handler))
        .with_state(mm)
}
