//! Web routes aggregation.
//!
//! Combines all route modules into a single router.

pub mod handlers;
pub mod openapi;
pub mod routes_contact;
pub mod routes_health;
pub mod routes_payment;
pub mod routes_seo;
pub mod routes_static;

use axum::Router;
use lib_core::model::ModelManager;

pub fn routes(mm: ModelManager) -> Router {
    let api_routes = Router::new()
        .merge(routes_contact::routes(mm.clone()))
        .merge(routes_payment::routes(mm.clone()));

    let system_routes = Router::new()
        .merge(routes_static::routes())
        .merge(routes_seo::routes())
        .merge(routes_health::routes(mm.clone()));

    Router::new().nest("/api", api_routes).merge(system_routes)
}
