//! SEO metadata for pages
//!
//! This module contains types for managing page metadata and SEO information.

use serde::{Deserialize, Serialize};

/// Page metadata for SEO purposes
///
/// Contains title, description, and other metadata used for
/// Open Graph, Twitter Cards, and search engine optimization.
/// Full business description for Google Business Profile and SEO (approx 750 chars)
pub const FULL_BUSINESS_DESCRIPTION: &str = "XF Tradesmen provides premium, SEO-optimized website solutions exclusively for UK tradespeople. We help electricians, plumbers, and handyman professionals establish a powerful online presence tailored to attract local clients. Our custom-built websites highlight your expertise, build trust, and drive direct enquiries without commission fees. Whether you need a simple portfolio or a complete business platform, XF Tradesmen delivers digital excellence to help your trade business grow.";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[must_use = "PageMetadata should be used for SEO"]
pub struct PageMetadata {
    /// Page title (shown in browser tab and search results)
    pub title: String,
    /// Page description (used in search results and social sharing)
    pub description: String,
    /// Open Graph image URL for social sharing
    #[serde(default)]
    pub og_image: Option<String>,
    /// Canonical URL for this page
    #[serde(default)]
    pub canonical_url: Option<String>,
}

impl PageMetadata {
    /// Create metadata for the homepage
    pub fn for_homepage() -> Self {
        Self {
            title: "XF Tradesmen - Connect with UK Tradesmen".to_string(),
            description: FULL_BUSINESS_DESCRIPTION.to_string(),
            og_image: Some("https://xftradesmen.com/og-image.jpg".to_string()),
            canonical_url: Some("https://xftradesmen.com/".to_string()),
        }
    }

    /// Create metadata for a service page
    pub fn for_service(service_name: &str, location: &str) -> Self {
        Self {
            title: format!(
                "{} in {} - Professional {} Services | XF Tradesmen",
                service_name.to_uppercase(),
                location,
                service_name
            ),
            description: format!(
                "Find qualified {} in {}. Professional services, verified tradesmen, competitive pricing. Get quotes from local {} today.",
                service_name, location, service_name
            ),
            og_image: Some("https://xftradesmen.com/og-image.jpg".to_string()),
            canonical_url: Some(format!(
                "https://xftradesmen.com/{}-{}",
                service_name.to_lowercase().replace(' ', "-"),
                location.to_lowercase().replace(' ', "-")
            )),
        }
    }

    /// Create metadata for a blog post
    pub fn for_blog(title: &str, description: &str) -> Self {
        Self {
            title: format!("{} | Blog | XF Tradesmen", title),
            description: description.to_string(),
            og_image: Some("https://xftradesmen.com/og-image.jpg".to_string()),
            canonical_url: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_homepage_metadata() {
        let metadata = PageMetadata::for_homepage();
        assert!(!metadata.title.is_empty());
        assert!(!metadata.description.is_empty());
        assert!(metadata.og_image.is_some());
    }

    #[test]
    fn test_service_metadata() {
        let metadata = PageMetadata::for_service("Electrician", "Coventry");
        assert!(metadata.title.contains("Electrician"));
        assert!(metadata.title.contains("Coventry"));
        assert!(metadata.description.contains("Electrician"));
    }

    #[test]
    fn test_blog_metadata() {
        let metadata = PageMetadata::for_blog("My Article", "This is a great article");
        assert!(metadata.title.contains("My Article"));
        assert_eq!(metadata.description, "This is a great article");
    }
}
