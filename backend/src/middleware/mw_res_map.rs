//! # Response Mapping Middleware
//!
//! ## Purpose
//! Converts internal server errors into safe, client-friendly JSON responses.
//! Prevents leaking internal error details (database errors, stack traces) to clients.
//!
//! ## How It Works
//! 1. Check if response has an Error in extensions (set by Error::into_response)
//! 2. If error exists: Map to (StatusCode, ClientError) via Error::client_status_and_error()
//! 3. Create JSON error response with type, message, and request URI
//! 4. Return mapped error OR original response if no error
//!
//! ## Security
//! - Hides sensitive info: Database connection strings, internal paths, user IDs
//! - Returns generic error codes: LOGIN_FAIL, NO_AUTH, SERVICE_ERROR
//! - Logs full error details server-side for debugging
//!
//! ## Relation to Entire Program
//! - **Applied Last**: Final middleware layer before response sent to client
//! - **Used By**: All routes (catches errors from any handler)
//! - **Error Flow**: Handler Error → into_response → Extensions → mw_res_map → Client JSON

use crate::error::Error;                 // Custom error type
use axum::http::{StatusCode, Uri};       // HTTP status codes and request URI
use axum::response::{IntoResponse, Response};  // Response types
use axum::Json;                          // JSON response
use serde_json::json;                    // JSON helper macro

/// Response mapping middleware
/// Converts internal errors to safe client JSON responses
///
/// # Process
/// 1. Extract Error from response extensions (if exists)
/// 2. Map Error to (StatusCode, ClientError)
/// 3. Create JSON error response
/// 4. Return error response or original response
///
/// # Example Error Response
/// ```json
/// {
///   "error": {
///     "type": "NO_AUTH",
///     "message": "Authentication required",
///     "req_uri": "/api/bookings"
///   }
/// }
/// ```
pub async fn mw_response_map(
    uri: Uri,
    res: Response,
) -> Response {
    // Extract internal Error from response extensions (if present)
    let service_error = res.extensions().get::<Error>().cloned();

    // Map Error to (StatusCode, ClientError) tuple
    let client_status_error = service_error.as_ref().map(|se| se.client_status_and_error());

    // Create error response JSON
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error = client_error.as_ref();
            let message = format!("{client_error}");

            let client_error_body = json!({
                "error": {
                    "type": client_error,      // Generic error code (NO_AUTH, LOGIN_FAIL, etc.)
                    "message": message,        // Human-readable message
                    "req_uri": uri.to_string(), // Request URI for debugging
                }
            });

            (*status_code, Json(client_error_body)).into_response()
        });

    // Return error response if error exists, otherwise return original response
    error_response.unwrap_or(res)
}
