//! Web routes aggregation.
//!
//! Combines all route modules into a single router.

pub mod openapi;
pub mod handlers;
pub mod routes_contact;
pub mod routes_payment;
pub mod routes_static;
pub mod routes_seo;

use axum::Router;
use lib_core::model::ModelManager;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .merge(routes_contact::routes(mm.clone()))
        .merge(routes_payment::routes(mm.clone()))
        .merge(routes_static::routes())
        .merge(routes_seo::routes())
}
