//! SEO routes for robots.txt and sitemap.xml.
//!
//! Provides static SEO files for search engine crawlers.

use axum::routing::get;
use axum::Router;
use crate::web::handlers::seo::{robots_txt_handler, sitemap_xml_handler};

pub fn routes() -> Router {
    Router::new()
        .route("/robots.txt", get(robots_txt_handler))
        .route("/sitemap.xml", get(sitemap_xml_handler))
}
