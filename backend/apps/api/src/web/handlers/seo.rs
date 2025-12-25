//! SEO static file handlers.
//!
//! Provides `/robots.txt` and `/sitemap.xml` for search engine crawlers.

use axum::http::header;
use axum::response::{IntoResponse, Response};
use std::sync::OnceLock;

static ROBOTS_TXT: OnceLock<String> = OnceLock::new();
static SITEMAP_XML: OnceLock<String> = OnceLock::new();

/// Handler for /robots.txt
pub async fn robots_txt_handler() -> impl IntoResponse {
    let robots = ROBOTS_TXT.get_or_init(|| {
        r#"User-agent: *
Allow: /
Sitemap: https://xftradesmen.com/sitemap.xml
"#
        .to_string()
    });

    Response::builder()
        .header(header::CONTENT_TYPE, "text/plain")
        .body(robots.clone())
        .unwrap_or_else(|_| Response::default())
}

/// Handler for /sitemap.xml
pub async fn sitemap_xml_handler() -> impl IntoResponse {
    let sitemap = SITEMAP_XML.get_or_init(|| {
        let base_url = "https://xftradesmen.com";

        // List of all static routes
        let static_routes = [
            "",
            "/about",
            "/contact",
            "/pricing",
            "/blog",
            "/coventry",
            "/packages",
            "/handyman",
            "/industries",
        ];

        // List of dynamic blog posts (hardcoded for now as they are static in frontend)
        let blog_posts = [
            "/blog/why-tradesmen-need-websites",
            "/blog/local-seo-guide",
            "/blog/building-trust-online",
        ];

        let mut url_entries = String::new();

        // Add static routes
        for route in static_routes {
            url_entries.push_str(&format!(
                r#"  <url>
    <loc>{}{}</loc>
    <changefreq>weekly</changefreq>
    <priority>{}</priority>
  </url>
"#,
                base_url,
                route,
                if route.is_empty() { "1.0" } else { "0.8" }
            ));
        }

        // Add blog posts
        for post in blog_posts {
            url_entries.push_str(&format!(
                r#"  <url>
    <loc>{}{}</loc>
    <changefreq>monthly</changefreq>
    <priority>0.7</priority>
  </url>
"#,
                base_url, post
            ));
        }

        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{}</urlset>"#,
            url_entries
        )
    });

    Response::builder()
        .header(header::CONTENT_TYPE, "application/xml")
        .body(sitemap.clone())
        .unwrap_or_else(|_| Response::default())
}
