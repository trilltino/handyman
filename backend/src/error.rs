//! # Error Handling Module
//!
//! ## Purpose
//! Centralized error handling for the handyman marketplace backend.
//! Defines custom error types and maps them to appropriate HTTP responses.
//!
//! ## How It Works
//! 1. **Error Enum**: Defines all possible application errors (Auth, Database, Stripe)
//! 2. **IntoResponse**: Automatically converts errors to HTTP responses with correct status codes
//! 3. **Client Errors**: Maps internal errors to safe, client-friendly error codes
//! 4. **Type Alias**: Provides `Result<T>` shorthand for `Result<T, Error>`
//!
//! ## Relation to Entire Program
//! - **Used By**: Every handler, repository, and service function returns `Result<T>`
//! - **Error Flow**: Handler Error → IntoResponse → HTTP Status + JSON → Client
//! - **Security**: Hides internal error details from clients via ClientError mapping
//! - **Examples**: LoginFail (403), DatabaseError (500), EntityNotFound (400)

use axum::http::StatusCode; // HTTP status codes (200, 401, 403, 500, etc.)
use axum::response::{IntoResponse, Response}; // Convert Error to HTTP response
use serde::Serialize; // JSON serialization for error responses

/// Type alias for Result with our custom Error
/// Used throughout the app: `Result<User>` instead of `Result<User, Error>`
pub type Result<T> = core::result::Result<T, Error>;

/// Main Error enum for all application errors
/// - Serializes to JSON for API responses
/// - AsRefStr allows converting to string for logging
#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- Database Errors (return 400 or 500)
    /// PostgreSQL query error with error message
    DatabaseError(String),
    /// Entity (User, Customer, Booking) not found by ID
    EntityNotFound { entity: &'static str, id: String },

    // -- External Service Errors
    /// Stripe API error (payment, subscription issues)
    StripeError(String),
}

// region:    --- Error Boilerplate
/// Implement Display trait for Error to enable println! and logging
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

/// Implement std::error::Error trait to make Error compatible with error handling libraries
impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

/// Convert our Error into an Axum HTTP Response
/// This is called automatically when a handler returns Err(Error)
/// Flow: Handler returns Err(Error) → into_response() → HTTP Response → Client
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // Log the error for debugging (visible in server logs)
        println!("->> {:<12} - {self:?}", "INTO_RES");

        // Create a default 500 Internal Server Error response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into response extensions so mw_res_map can read it
        // mw_res_map will convert this to a safe client error JSON
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    /// Maps internal Error to safe (StatusCode, ClientError) for the client
    /// Called by: mw_res_map middleware to create JSON error response
    /// Security: Hides internal error details (DB errors, stack traces) from clients
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            // 400 Bad Request - Entity (User, Booking) not found
            Self::EntityNotFound { .. } => (StatusCode::BAD_REQUEST, ClientError::ENTITY_NOT_FOUND),

            // 500 Internal Server Error - PostgreSQL errors
            Self::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),

            // 400 Bad Request - Stripe payment/subscription errors
            Self::StripeError(_) => (StatusCode::BAD_REQUEST, ClientError::STRIPE_ERROR),
        }
    }
}

/// Safe error codes sent to clients
/// These are generic and don't expose internal implementation details
#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    /// Requested entity not found in database
    ENTITY_NOT_FOUND,
    /// Internal server error (database, etc.)
    SERVICE_ERROR,
    /// Stripe payment service error
    STRIPE_ERROR,
}
