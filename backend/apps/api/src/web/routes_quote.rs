//! Quote API routes.
//!
//! Routes for quote management and instant quote calculator.

use axum::{
    routing::{get, post},
    Router,
};
use lib_core::model::ModelManager;

use super::handlers::quote;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        // Quote CRUD
        .route("/quotes", post(quote::create_quote))
        .route("/quotes/:id", get(quote::get_quote))
        .route("/quotes/:id/accept", post(quote::accept_quote))
        // Templates
        .route("/quotes/templates", get(quote::get_quote_templates))
        // Public instant quote
        .route("/quote/instant", post(quote::get_instant_quote))
        .with_state(mm)
}
