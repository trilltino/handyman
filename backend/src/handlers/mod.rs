//! # Handlers Module
//!
//! ## Purpose
//! HTTP request handlers (controllers) for all API endpoints.
//! Each handler receives requests, calls repositories, returns responses.
//!
//! ## Handler Categories
//! - **health**: Server health and status endpoints
//! - **auth**: User registration, login, logout, current user
//! - **booking**: Create bookings, payment processing
//! - **contact**: Contact form submissions
//!
//! ## Relation to Entire Program
//! - **Called By**: Axum router (main.rs routes)
//! - **Calls**: Repositories (database operations), external APIs (Stripe)
//! - **Flow**: HTTP Request → Handler → Repository → Database → Response

pub mod health;   // Health check handlers
pub mod contact;  // Contact form handler
pub mod booking;  // Booking creation handler
pub mod auth;     // Authentication handlers

// Re-export handler functions for router
pub use health::{root, health_check};
pub use contact::handle_contact;
pub use booking::handle_booking;
pub use auth::{handle_register, handle_login, handle_logout, handle_me};
