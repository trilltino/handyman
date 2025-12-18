//! # Handlers Module
//!
//! ## Purpose
//! HTTP request handlers (controllers) for all API endpoints.
//! Each handler receives requests, calls repositories, returns responses.
//!
//! ## Handler Categories
//! - **health**: Server health and status endpoints
//! - **booking**: Create bookings, payment processing
//! - **contact**: Contact form submissions
//!
//! ## Relation to Entire Program
//! - **Called By**: Axum router (main.rs routes)
//! - **Calls**: Repositories (database operations), external APIs (Stripe)
//! - **Flow**: HTTP Request → Handler → Repository → Database → Response

pub mod booking;
pub mod contact; // Contact form handler
pub mod health; // Health check handlers // Booking creation handler

pub use booking::handle_booking;
pub use contact::handle_contact;
pub use health::{health_check, root};
