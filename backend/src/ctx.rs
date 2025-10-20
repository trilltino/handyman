//! # Request Context Module
//!
//! ## Purpose
//! Stores authenticated user information for each HTTP request.
//! Links the current request to the user who made it.
//!
//! ## How It Works
//! 1. **Created by**: `mw_ctx_resolver` middleware after JWT token validation
//! 2. **Contains**: User ID extracted from validated JWT token
//! 3. **Stored in**: Axum request extensions (available to all handlers)
//! 4. **Used by**: Protected route handlers to identify current user
//!
//! ## Relation to Entire Program
//! - **Authentication Flow**: JWT Token → mw_ctx_resolver → Ctx → Handler
//! - **Links**: HTTP Request ↔ User in Database
//! - **Used By**: All protected handlers (booking, logout, /me endpoint)
//! - **Example**: When user creates booking, Ctx provides user_id to link booking to user

use crate::error::{Error, Result}; // Custom error types

/// Request Context - stores authenticated user info for current request
/// Created by: mw_ctx_resolver middleware
/// Stored in: Axum request extensions
/// Accessed by: Handlers via Axum extractor
#[derive(Clone, Debug)]
pub struct Ctx {
    /// User ID from validated JWT token
    /// Links this request to a user in the database
    user_id: i32,
}

impl Ctx {
    /// Creates a new request context with user ID
    /// Called by: mw_ctx_resolver after validating JWT token
    pub fn new(user_id: i32) -> Result<Self> {
        Ok(Self { user_id })
    }

    /// Returns the user ID for this request
    /// Used by: Handlers to identify which user made the request
    /// Example: booking handler uses this to set booking.user_id
    pub fn user_id(&self) -> i32 {
        self.user_id
    }
}
