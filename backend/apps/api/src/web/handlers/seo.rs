//! SEO static file handlers.
//!
//! Provides `/robots.txt` and `/sitemap.xml` for search engine crawlers.

use axum::response::{IntoResponse, Response};
use axum::http::header;

/// Handler for /robots.txt
pub async fn robots_txt_handler() -> impl IntoResponse {
    let robots = r#"User-agent: *
Allow: /
Sitemap: https://xftradesmen.com/sitemap.xml
"#;

    Response::builder()
        .header(header::CONTENT_TYPE, "text/plain")
        .body(robots.to_string())
        .unwrap()
}

/// Handler for /sitemap.xml
pub async fn sitemap_xml_handler() -> impl IntoResponse {
    let base_url = "https://xftradesmen.com";
    
    // List of all static routes
    let static_routes = vec![
        "",
        "/about",
        "/contact",
        "/pricing",
        "/blog",
        "/coventry",
        "/electrician-coventry",
        "/plumber-coventry",
    ];

    // List of dynamic blog posts (hardcoded for now as they are static in frontend)
    let blog_posts = vec![
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
            base_url,
            post
        ));
    }

    let sitemap = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{}</urlset>"#,
        url_entries
    );

    Response::builder()
        .header(header::CONTENT_TYPE, "application/xml")
        .body(sitemap)
        .unwrap()
}
