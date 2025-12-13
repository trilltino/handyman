//! Caching Middleware
//!
//! Provides HTTP caching headers and strategies for improved performance.
//! Includes browser caching, CDN optimization, and cache control headers.

use axum::{
    extract::Request,
    http::{header, HeaderMap, Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::time::Duration;

/// Cache control middleware
/// Adds appropriate caching headers based on request path and method
pub async fn cache_control_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let mut response = next.run(req).await;

    // Only add cache headers to GET requests
    if req.method() != Method::GET {
        return Ok(response);
    }

    let path = req.uri().path();

    // Set cache headers based on content type
    let cache_headers = get_cache_headers(path);

    // Add cache control headers
    for (key, value) in cache_headers {
        response.headers_mut().insert(key, value);
    }

    Ok(response)
}

/// Determine appropriate cache headers for a given path
fn get_cache_headers(path: &str) -> Vec<(header::HeaderName, header::HeaderValue)> {
    let mut headers = Vec::new();

    // Static assets - long cache
    if path.starts_with("/pkg/") || path.starts_with("/assets/") {
        headers.push((
            header::CACHE_CONTROL,
            header::HeaderValue::from_static("public, max-age=31536000, immutable"),
        ));
    }
    // Images - medium cache
    else if path.ends_with(".jpg") || path.ends_with(".jpeg") || path.ends_with(".png") ||
              path.ends_with(".gif") || path.ends_with(".webp") || path.ends_with(".svg") {
        headers.push((
            header::CACHE_CONTROL,
            header::HeaderValue::from_static("public, max-age=86400, stale-while-revalidate=604800"),
        ));
    }
    // CSS and JS - medium cache with revalidation
    else if path.ends_with(".css") || path.ends_with(".js") {
        headers.push((
            header::CACHE_CONTROL,
            header::HeaderValue::from_static("public, max-age=3600, stale-while-revalidate=86400"),
        ));
    }
    // API responses - short cache
    else if path.starts_with("/api/") {
        headers.push((
            header::CACHE_CONTROL,
            header::HeaderValue::from_static("private, max-age=300, stale-while-revalidate=600"),
        ));
    }
    // HTML pages - no cache for dynamic content
    else {
        headers.push((
            header::CACHE_CONTROL,
            header::HeaderValue::from_static("no-cache, no-store, must-revalidate"),
        ));
        headers.push((
            header::PRAGMA,
            header::HeaderValue::from_static("no-cache"),
        ));
        headers.push((
            header::EXPIRES,
            header::HeaderValue::from_static("0"),
        ));
    }

    headers
}

/// Compression middleware (placeholder for future implementation)
/// Would integrate with tower-http compression
pub async fn compression_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // For now, just pass through
    // In production, this would compress responses
    Ok(next.run(req).await)
}

/// Security headers middleware
/// Adds security-related headers for better protection
pub async fn security_headers_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let mut response = next.run(req).await;

    let headers = response.headers_mut();

    // Security headers
    headers.insert(
        "X-Content-Type-Options",
        header::HeaderValue::from_static("nosniff"),
    );

    headers.insert(
        "X-Frame-Options",
        header::HeaderValue::from_static("DENY"),
    );

    headers.insert(
        "X-XSS-Protection",
        header::HeaderValue::from_static("1; mode=block"),
    );

    headers.insert(
        "Referrer-Policy",
        header::HeaderValue::from_static("strict-origin-when-cross-origin"),
    );

    // Content Security Policy (basic)
    headers.insert(
        "Content-Security-Policy",
        header::HeaderValue::from_static(
            "default-src 'self'; script-src 'self' 'unsafe-inline' https://js.stripe.com https://www.googletagmanager.com; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; connect-src 'self' https://api.stripe.com;"
        ),
    );

    Ok(response)
}