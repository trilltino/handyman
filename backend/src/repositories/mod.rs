//! # Repositories Module
//!
//! ## Purpose
//! Data access layer for database operations (CRUD).
//! Each repository handles database queries for one entity type.
//!
//! ## Repository Pattern
//! - **Separation of Concerns**: Isolates SQL queries from business logic
//! - **Type Safety**: Uses SQLx for compile-time SQL verification
//! - **Reusability**: Repositories used by multiple handlers
//!
//! ## Relation to Entire Program
//! - **Used By**: Handlers call repositories to fetch/store data
//! - **Uses**: PostgreSQL connection from pool
//! - **Flow**: Handler → Repository → Database → Repository → Handler

mod customer;  // Customer CRUD operations
mod booking;   // Booking CRUD operations
mod contact;   // Contact message CRUD operations
mod user;      // User CRUD operations

// Re-export repository structs
pub use customer::CustomerRepository;
pub use booking::BookingRepository;
pub use contact::ContactRepository;
pub use user::UserRepository;
