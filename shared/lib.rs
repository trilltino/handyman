//! # Shared Types Library
//!
//! ## Purpose
//! Shared data structures and types used by both frontend (Yew/WASM) and backend (Axum).
//! Ensures type consistency across the full stack.
//!
//! ## Shared Types
//! - **ContactForm**: Contact form submission data
//! - **ApiResponse<T>**: Generic API response wrapper
//!
//! ## Why Shared?
//! - **Type Safety**: Frontend and backend use identical types (compile-time validation)
//! - **DRY Principle**: Define types once, use everywhere
//! - **Serialization**: Same serde attributes for JSON serialization
//!
//! ## Relation to Entire Program
//! - **Used By**: Both frontend (form validation) and backend (API responses)
//! - **Compiled Twice**: Once for WASM (frontend), once for native (backend)
//! - **Future**: Will include more shared types (Booking, User, etc.)

use serde::{Deserialize, Serialize};  // JSON serialization for frontend-backend communication

/// Contact form data structure
/// Used by both frontend form and backend handler
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContactForm {
    /// Sender's name from contact form
    pub name: String,
    /// Sender's email for follow-up
    pub email: String,
    /// Message content from textarea
    pub message: String,
}

/// Generic API response wrapper
/// Standardizes response format across all endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Success flag (true if operation succeeded)
    pub success: bool,
    /// Human-readable message
    pub message: String,
    /// Optional data payload (type varies by endpoint)
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    /// Create a successful response with data
    /// Used by: Backend handlers to return successful results
    pub fn success(message: impl Into<String>, data: T) -> Self {
        Self {
            success: true,
            message: message.into(),
            data: Some(data),
        }
    }

    /// Create an error response without data
    /// Used by: Backend handlers to return error messages
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            data: None,
        }
    }
}
