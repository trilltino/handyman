//! # Pages Module
//!
//! ## Purpose
//! Page components for each route in the application.
//! Each page is a full-screen view corresponding to a route.
//!
//! ## Pages
//! - **Home**: Landing page with hero, features, CTA
//! - **Login**: User login form
//! - **Register**: User registration form
//! - **Booking**: Service booking form with pricing
//! - **Contact**: Contact us form
//! - **Experience**: Handyman experience/portfolio showcase
//! - **MapPage**: Interactive 3D Cesium map
//!
//! ## Relation to Entire Program
//! - **Rendered By**: App component via router Switch
//! - **Each Page**: Self-contained component with its own state and API calls

pub mod home;       // Landing page
pub mod login;      // Login form page
pub mod register;   // Registration form page
pub mod booking;    // Booking form page
pub mod contact;    // Contact form page
pub mod experience; // Experience/portfolio page
pub mod map;        // 3D Cesium map page
pub mod map_leaflet_backup;  // Backup Leaflet map implementation

pub use home::Home;
pub use login::Login;
pub use register::Register;
pub use booking::Booking;
pub use contact::Contact;
pub use experience::Experience;
pub use map::MapPage;
