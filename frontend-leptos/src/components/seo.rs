//! SEO meta tag components.
//!
//! Provides components for injecting SEO metadata into page `<head>`.
//! Uses `PageMetadata` from shared crate for consistency.

use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use shared::PageMetadata;

/// Returns the canonical base URL for the site.
/// Controlled via CANONICAL_BASE env var. Defaults to production domain.
#[allow(dead_code)]
pub fn canonical_base() -> String {
    std::env::var("CANONICAL_BASE")
        .unwrap_or_else(|_| "https://handymanmarketplace.com".to_string())
}

/// Build an absolute URL from a path using the canonical base.
#[allow(dead_code)]
pub fn build_url(path: &str) -> String {
    let base = canonical_base();
    let base = base.trim_end_matches('/');
    let path = if path.starts_with('/') {
        path
    } else {
        &format!("/{}", path)
    };
    format!("{}{}", base, path)
}

/// SEO Head component.
///
/// Injects <title>, <meta>, and <link> tags into the <head> of the document.
/// Uses `PageMetadata` from the shared crate to ensure consistent SEO data.
#[component]
pub fn SeoHead(
    /// Page metadata (title, description, etc.)
    #[prop(into)]
    metadata: PageMetadata,
) -> impl IntoView {
    let title = metadata.title;
    let description = metadata.description;
    let image = metadata.og_image.unwrap_or_else(|| "https://handymanmarketplace.com/og-image.jpg".to_string());
    let url = metadata.canonical_url.unwrap_or_else(|| "https://handymanmarketplace.com".to_string());
    
    view! {
        <Title text=title.clone()/>
        <Meta name="description" content=description.clone()/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>
        <link rel="canonical" href=url.clone()/>

        <Meta property="og:title" content=title.clone()/>
        <Meta property="og:description" content=description.clone()/>
        <Meta property="og:image" content=image.clone()/>
        <Meta property="og:url" content=url.clone()/>
        <Meta property="og:type" content="website"/>

        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content=title/>
        <Meta name="twitter:description" content=description/>
        <Meta name="twitter:image" content=image/>
    }
}
