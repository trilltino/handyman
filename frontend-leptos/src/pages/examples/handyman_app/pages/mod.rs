//! Handyman App Pages
//!
//! Organized into logical submodules:
//! - `main`: Home, Services, Service Details, Features, Areas, ServiceMap
//! - `booking`: Booking, Quote, Customers, Emergency
//! - `content`: Blog, Testimonials, Legal, Other
//! - `admin`: Admin Dashboard

pub mod admin;
pub mod booking;
pub mod content;
pub mod main;
pub mod services;

// Re-export all components for backward compatibility
#[allow(unused_imports)]
pub use admin::*;
#[allow(unused_imports)]
pub use booking::*;
#[allow(unused_imports)]
pub use content::*;
#[allow(unused_imports)]
pub use main::*;
