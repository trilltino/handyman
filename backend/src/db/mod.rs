//! # Database Module
//!
//! ## Purpose
//! Provides PostgreSQL database connection pooling functionality.
//! Exports connection pool types and creation functions.
//!
//! ## Relation to Entire Program
//! - **Used By**: main.rs creates pool at startup, handlers get connections from pool
//! - **Performance**: Reuses connections instead of creating new ones per request
//! - **Module Organization**: Re-exports pool.rs functionality

pub mod pool;  // Database connection pooling implementation

// Re-export key types and functions from pool module
pub use pool::{create_pool, DbPool, DatabaseConnection};
