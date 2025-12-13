//! Structured data (JSON-LD) schemas
//!
//! This module generates JSON-LD structured data for search engine optimization.
//! Structured data helps search engines understand page content and display rich results.
//!
//! ## Modules
//!
//! - `organization` - Organization information schema
//! - `service` - Service offering schema
//! - `faq` - FAQ page schema
//! - `blog_post` - Blog post schema
//!
//! ## References
//!
//! - [Schema.org](https://schema.org/) - Official schema documentation
//! - [Google Rich Results](https://developers.google.com/search/docs/appearance/structured-data/intro-structured-data)

pub mod blog_post;
pub mod faq;
pub mod organization;
pub mod service;

pub use blog_post::create_blog_post_schema;
pub use faq::create_faq_schema;
pub use organization::create_organization_schema;
pub use service::create_service_schema;
