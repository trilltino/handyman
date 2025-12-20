//! # Web Error Handling
//!
//! Converts internal errors from lib-core and lib-auth layers into
//! appropriate HTTP responses with secure error messages.
//!
//! ## Design
//!
//! This module follows the principle of error hiding: internal details
//! are never exposed to clients. Sensitive errors are logged server-side
//! but generic messages are returned to clients.
//!
//! ## Error Mapping
//!
//! Internal errors are automatically mapped to HTTP status codes:
//!
//! | Error Type | HTTP Status | Meaning |
//! |------------|-------------|---------|
//! | AuthRequired | 401 | No authentication provided |
//! | InvalidCredentials | 401 | Bad username/password |
//! | InvalidToken | 401 | Token malformed or invalid |
//! | TokenExpired | 401 | Token has expired |
//! | InsufficientPermissions | 403 | User lacks required permissions |
//! | NotAMember | 403 | User not member of resource |
//! | EntityNotFound | 404 | Resource doesn't exist |
//! | TradesmanAlreadyExists | 409 | Username taken |
//! | InvalidData | 400 | Input validation failed |
//! | Other | 500 | Unexpected error |
//!
//! ## Example: Handler with Error Handling
//!
//! ```rust,no_run
//! use axum::{extract::State, Json};
//! use lib_web::Result;
//! use lib_core::model::ModelManager;
//!
//! async fn get_user(
//!     State(mm): State<ModelManager>,
//! ) -> Result<Json<UserResponse>> {
//!     // If user not found, automatically returns 404
//!     // If database error, automatically returns 500
//!     let user = UserBmc::get(&mm, 1).await?;
//!     Ok(Json(UserResponse { user }))
//! }
//! ```

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use lib_core::model::Error as ModelError;
use serde::Serialize;

/// Web operation result type.
///
/// All handler functions should return `Result<T>` where errors are
/// automatically converted to HTTP responses.
pub type Result<T> = core::result::Result<T, Error>;

use std::borrow::Cow;

/// Web layer errors.
///
/// This enum wraps errors from various layers (database, auth, business logic)
/// and maps them to appropriate HTTP status codes. All errors are client-safe
/// and don't leak internal implementation details.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// No authentication provided (401 Unauthorized).
    ///
    /// Returned when a protected endpoint is accessed without credentials.
    #[error("Authentication required")]
    AuthRequired,

    /// Invalid username or password (401 Unauthorized).
    ///
    /// Returned when login fails due to incorrect credentials.
    #[error("Invalid credentials")]
    InvalidCredentials,

    /// Malformed or invalid token (401 Unauthorized).
    ///
    /// Returned when a JWT or session token is invalid or corrupted.
    #[error("Invalid token")]
    InvalidToken,

    /// Token has expired (401 Unauthorized).
    ///
    /// Returned when a token exists but has passed its expiration time.
    #[error("Token expired")]
    TokenExpired,

    /// User lacks required permissions (403 Forbidden).
    ///
    /// Returned when an authenticated user doesn't have permission
    /// to perform an action.
    #[error("Insufficient permissions")]
    InsufficientPermissions,

    /// User is not a member of the resource (403 Forbidden).
    ///
    /// Returned when a user tries to access a resource that belongs
    /// to an organization they're not a member of.
    #[error("Not a member of this organization")]
    NotAMember,

    /// Input validation failed (400 Bad Request).
    ///
    /// Returned when request data fails validation constraints.
    #[error("Validation error: {0}")]
    ValidationError(Cow<'static, str>),

    /// Business logic error from lib-core (varies by type).
    ///
    /// Wraps errors from the model layer. The specific error type
    /// determines the HTTP status code and response.
    #[error("Model error: {0}")]
    Model(#[from] ModelError),

    /// Password hashing or validation error (400 Bad Request).
    ///
    /// Returned when password operations fail (hashing, verification).
    #[error("Password error: {0}")]
    PasswordError(String),

    /// Token processing error (401 Unauthorized).
    ///
    /// Returned when token creation or verification fails.
    #[error("Token error: {0}")]
    TokenError(String),
}

impl Error {
    /// Add context information to the error for logging purposes.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use lib_web::Error;
    /// let err = Error::ValidationError("Email is invalid".into());
    /// let contextual = err.context("contact_form_handler");
    /// ```
    pub fn context(self, msg: &str) -> Self {
        tracing::debug!("{}: {:?}", msg, self);
        self
    }

    /// Get the HTTP status code for this error.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use lib_web::Error;
    /// # use axum::http::StatusCode;
    /// let err = Error::AuthRequired;
    /// assert_eq!(err.status_code(), StatusCode::UNAUTHORIZED);
    /// ```
    pub fn status_code(&self) -> StatusCode {
        match self {
            Error::AuthRequired
            | Error::InvalidCredentials
            | Error::InvalidToken
            | Error::TokenExpired
            | Error::TokenError(_) => StatusCode::UNAUTHORIZED,

            Error::InsufficientPermissions | Error::NotAMember => StatusCode::FORBIDDEN,

            Error::ValidationError(_) | Error::PasswordError(_) => StatusCode::BAD_REQUEST,

            Error::Model(model_err) => match model_err {
                ModelError::EntityNotFound { .. } => StatusCode::NOT_FOUND,
                ModelError::UserAlreadyExists { .. } | ModelError::UniqueViolation { .. } => {
                    StatusCode::CONFLICT
                }
                ModelError::ValidationError(_) => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }
}

/// Client error response (safe to send to clients).
#[derive(Serialize)]
struct ClientError {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<Cow<'static, str>>,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!("Web error: {:?}", self);

        let (status, error_message, detail) = match &self {
            // Auth errors (401)
            Error::AuthRequired => (StatusCode::UNAUTHORIZED, "Authentication required", None),
            Error::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials", None),
            Error::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token", None),
            Error::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expired", None),

            // Authorization errors (403)
            Error::InsufficientPermissions => {
                (StatusCode::FORBIDDEN, "Insufficient permissions", None)
            }
            Error::NotAMember => (
                StatusCode::FORBIDDEN,
                "Not a member of this organization",
                None,
            ),

            // Validation errors (400)
            Error::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                "Validation failed",
                Some(msg.clone()),
            ),

            // Model errors
            Error::Model(model_error) => {
                match model_error {
                    ModelError::EntityNotFound { entity, id } => (
                        StatusCode::NOT_FOUND,
                        "Resource not found",
                        Some(format!("{} with id {} not found", entity, id).into()),
                    ),
                    ModelError::UserAlreadyExists { username } => (
                        StatusCode::CONFLICT,
                        "User already exists",
                        Some(username.clone().into()),
                    ),
                    ModelError::UniqueViolation { table, constraint } => (
                        StatusCode::CONFLICT,
                        "Unique constraint violation",
                        Some(format!("{} constraint on {}", constraint, table).into()),
                    ),
                    ModelError::ValidationError(msg) => (
                        StatusCode::BAD_REQUEST,
                        "Validation error",
                        Some(msg.clone()),
                    ),
                    // Internal errors - don't expose details
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error",
                        None,
                    ),
                }
            }

            // Other errors
            Error::PasswordError(_) => (StatusCode::BAD_REQUEST, "Password error", None),
            Error::TokenError(_) => (StatusCode::UNAUTHORIZED, "Token error", None),
        };

        let client_error = ClientError {
            error: error_message.to_string(),
            detail,
        };

        (status, axum::Json(client_error)).into_response()
    }
}
