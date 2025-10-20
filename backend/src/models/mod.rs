//! # Data Models Module
//!
//! ## Purpose
//! Defines all database entity structures and Data Transfer Objects (DTOs).
//! Represents the core data types of the handyman marketplace.
//!
//! ## Models Overview
//! - **User**: User accounts with authentication (username, email, password hash)
//! - **Customer**: Customer contact information (full name, email, phone)
//! - **Booking**: Service bookings linking users and customers
//! - **Contact**: Contact form submissions
//!
//! ## Relation to Entire Program
//! - **Used By**: Repositories (CRUD operations), Handlers (request/response)
//! - **Database Mapping**: Each struct maps to a PostgreSQL table
//! - **Serialization**: All models implement Serialize/Deserialize for JSON

pub mod customer;  // Customer model (booking requesters)
pub mod booking;   // Booking model (service requests)
pub mod contact;   // Contact form message model
pub mod user;      // User model (handymen, future: marketplace users)

// Re-export key types for easy access
pub use customer::{Customer, NewCustomer};
pub use booking::{Booking, NewBooking, WorkType, BookingStatus, PaymentStatus};
pub use contact::{ContactMessage, NewContactMessage};
pub use user::{User, UserForCreate, UserForLogin, UserForAuth, hash_password, verify_password};
