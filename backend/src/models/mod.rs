//! # Data Models Module
//!
//! ## Purpose
//! Defines all database entity structures and Data Transfer Objects (DTOs).
//! Represents the core data types of the handyman marketplace.
//!
//! ## Models Overview
//! - **Customer**: Customer contact information (full name, email, phone)
//! - **Booking**: Service bookings linking customers
//! - **Contact**: Contact form submissions
//!
//! ## Relation to Entire Program
//! - **Used By**: Repositories (CRUD operations), Handlers (request/response)
//! - **Database Mapping**: Each struct maps to a PostgreSQL table
//! - **Serialization**: All models implement Serialize/Deserialize for JSON

pub mod booking; // Booking model (service requests)
pub mod contact;
pub mod customer; // Customer model (booking requesters) // Contact form message model

// Re-export key types for easy access
pub use booking::{Booking, BookingStatus, NewBooking, PaymentStatus, WorkType};
pub use contact::{ContactMessage, NewContactMessage};
pub use customer::{Customer, NewCustomer};
