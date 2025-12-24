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

// Re-export all components for backward compatibility
pub use admin::*;
pub use booking::*;
pub use content::*;
pub use main::*;
