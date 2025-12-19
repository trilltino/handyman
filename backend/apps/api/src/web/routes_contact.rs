//! # Contact Form API Routes
//!
//! This module handles contact form submissions via HTTP POST requests.

use crate::web::handlers::contact::api_contact_handler;
use axum::routing::post;
use axum::Router;
use lib_core::model::ModelManager;

/// Creates the contact routes for the API.
pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/contact", post(api_contact_handler))
        .with_state(mm)
}
