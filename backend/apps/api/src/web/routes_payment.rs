//! Payment webhook handling for Stripe
//!
//! This module processes Stripe webhook events for payment confirmations.

use axum::routing::get;
use axum::Router;
use lib_core::model::ModelManager;

pub fn routes(mm: ModelManager) -> Router {
    Router::new().with_state(mm)
}
