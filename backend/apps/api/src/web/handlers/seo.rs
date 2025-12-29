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
Sitemap: https://xftradesman.com/sitemap.xml
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
        let base_url = "https://xftradesman.com";

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
            "/terms",
            "/faq",
            "/service-agreement",
            // Handyman Example App Routes
            "/handyman-coventry",
            "/handyman-coventry/services",
            "/handyman-coventry/features",
            "/handyman-coventry/testimonials",
            "/handyman-coventry/service-area",
            "/handyman-coventry/booking",
            "/handyman-coventry/quote",
            "/handyman-coventry/about",
            "/handyman-coventry/contact",
            "/handyman-coventry/blog",
            "/handyman-coventry/emergency",
            "/handyman-coventry/privacy",
            "/handyman-coventry/terms",
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

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;

    #[tokio::test]
    async fn test_robots_txt_handler_returns_text() {
        let response = robots_txt_handler().await.into_response();

        // Check content type
        assert_eq!(
            response.headers().get("content-type"),
            Some(&header::HeaderValue::from_static("text/plain"))
        );
    }

    #[tokio::test]
    async fn test_robots_txt_contains_sitemap() {
        let response = robots_txt_handler().await.into_response();
        let body = to_bytes(response.into_body(), 4096).await.unwrap();
        let content = String::from_utf8(body.to_vec()).unwrap();

        assert!(content.contains("Sitemap:"));
        assert!(content.contains("User-agent:"));
    }

    #[tokio::test]
    async fn test_sitemap_xml_handler_returns_xml() {
        let response = sitemap_xml_handler().await.into_response();

        // Check content type
        assert_eq!(
            response.headers().get("content-type"),
            Some(&header::HeaderValue::from_static("application/xml"))
        );
    }

    #[tokio::test]
    async fn test_sitemap_contains_required_pages() {
        let response = sitemap_xml_handler().await.into_response();
        let body = to_bytes(response.into_body(), 16384).await.unwrap();
        let content = String::from_utf8(body.to_vec()).unwrap();

        // Should be valid XML structure
        assert!(content.contains("<?xml"));
        assert!(content.contains("<urlset"));
        assert!(content.contains("<url>"));
        assert!(content.contains("<loc>"));

        // Should contain main pages
        assert!(content.contains("xftradesman.com"));
        assert!(content.contains("/about"));
        assert!(content.contains("/contact"));
        assert!(content.contains("/pricing"));
    }

    #[tokio::test]
    async fn test_sitemap_contains_blog_posts() {
        let response = sitemap_xml_handler().await.into_response();
        let body = to_bytes(response.into_body(), 16384).await.unwrap();
        let content = String::from_utf8(body.to_vec()).unwrap();

        // Should contain blog posts
        assert!(content.contains("/blog/why-tradesmen-need-websites"));
        assert!(content.contains("/blog/local-seo-guide"));
        assert!(content.contains("/blog/building-trust-online"));
    }

    #[tokio::test]
    async fn test_sitemap_has_priorities() {
        let response = sitemap_xml_handler().await.into_response();
        let body = to_bytes(response.into_body(), 16384).await.unwrap();
        let content = String::from_utf8(body.to_vec()).unwrap();

        // Should have priority elements
        assert!(content.contains("<priority>"));
        assert!(content.contains("1.0")); // Homepage
        assert!(content.contains("0.8")); // Main pages
        assert!(content.contains("0.7")); // Blog posts
    }
}

// endregion: --- Tests
