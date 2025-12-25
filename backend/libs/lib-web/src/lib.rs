//! # Web Layer Utilities
//!
//! Common utilities for building Axum web servers with consistent error
//! handling and request/response patterns.
//!
//! ## Key Types
//!
//! - **[`Error`]** - Unified error type for web operations
//! - **[`Result<T>`]** - Web operation result type
//! - **[`ValidatedJson`]** - Auto-validating JSON extractor
//!
//! ## Error Handling Pattern
//!
//! Errors from lib-core (database, validation, etc.) are automatically
//! converted to appropriate HTTP status codes:
//! - `ModelError::EntityNotFound` → 404 Not Found
//! - `ModelError::InvalidData` → 400 Bad Request
//! - Everything else → 500 Internal Server Error

pub mod error;
pub mod extractors;

// Re-export commonly used types
pub use error::{Error, Result};
pub use extractors::ValidatedJson;
