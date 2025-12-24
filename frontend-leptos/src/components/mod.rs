//! Frontend UI components.
//!
//! - `ui`: Reusable primitive components (Button, Badge, Card, Section, etc.)
//! - `seo`: SEO metadata injection
//! - `layout`: Navbar and Footer
//! - `service_map`: Interactive map for service areas
//! - `error_boundary`: Error handling components

pub mod error_boundary;
pub mod layout;
pub mod seo;
pub mod service_map;
pub mod ui;

// Re-exports for convenience (intentional public API)
#[allow(unused_imports)]
pub use error_boundary::{ErrorBoundary, LoadingPlaceholder, LoadingSpinner};
#[allow(unused_imports)]
pub use service_map::CoventryServiceMap;
