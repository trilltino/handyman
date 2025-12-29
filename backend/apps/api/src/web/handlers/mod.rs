//! API request handlers.
//!
//! This module contains all HTTP request handlers organized by domain:
//! - `contact`: Contact form submissions
//! - `static_content`: Health checks, version info, config
//! - `seo`: Robots.txt and sitemap generation
//! - `quote`: Quote management and instant quotes

pub mod contact;
pub mod quote;
pub mod seo;
pub mod static_content;
