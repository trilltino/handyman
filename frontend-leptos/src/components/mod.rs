//! Frontend UI components.
//!
//! - `ui`: Reusable primitive components (Button, Badge, Card, Section, etc.)
//! - `seo`: SEO metadata injection
//! - `layout`: Navbar and Footer
//! - `service_map`: Interactive map for service areas

pub mod layout;
pub mod seo;
pub mod service_map;
pub mod ui;

// Re-export ServiceMap for convenience
pub use service_map::CoventryServiceMap;
