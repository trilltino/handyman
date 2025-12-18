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

mod booking; // Booking CRUD operations
mod contact;
mod customer; // Customer CRUD operations // Contact message CRUD operations

// Re-export repository structs
pub use booking::BookingRepository;
pub use contact::ContactRepository;
pub use customer::CustomerRepository;
