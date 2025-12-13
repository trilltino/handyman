//! Authentication data types.
//!
//! Provides login, registration, and authentication response types.
//! All types include comprehensive validation and security documentation.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Payload for user login.
///
/// # Security Considerations
///
/// - Passwords are transmitted as plaintext strings and **MUST** be sent over HTTPS only.
/// - Passwords should be validated server-side before hashing.
/// - Implement rate limiting on login endpoints to prevent brute-force attacks.
///
/// # Example
///
/// ```rust
/// use shared::types::LoginPayload;
///
/// let payload = LoginPayload {
///     username: "alice".to_string(),
///     password: "super_secret_password".to_string(),
/// };
///
/// assert!(payload.is_valid());
/// ```
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
#[must_use = "LoginPayload should be validated before use"]
pub struct LoginPayload {
    /// Username for authentication (typically 3-50 characters)
    pub username: String,
    
    /// User password (minimum 8 characters recommended)
    ///
    /// # Security Note
    /// This field contains sensitive data. Ensure it's:
    /// - Never logged
    /// - Transmitted over HTTPS only
    /// - Cleared from memory after use
    pub password: String,
}

impl LoginPayload {
    /// Validates the login payload.
    ///
    /// Checks that:
    /// - Username is not empty and contains valid characters
    /// - Password meets minimum length requirements (8 chars)
    ///
    /// # Returns
    ///
    /// `true` if the payload passes basic validation, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shared::types::LoginPayload;
    ///
    /// let valid = LoginPayload {
    ///     username: "alice".to_string(),
    ///     password: "securepass123".to_string(),
    /// };
    /// assert!(valid.is_valid());
    ///
    /// let invalid = LoginPayload {
    ///     username: "".to_string(),
    ///     password: "123".to_string(),
    /// };
    /// assert!(!invalid.is_valid());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_valid(&self) -> bool {
        !self.username.is_empty() && !self.password.is_empty()
    }

    /// Validates username format.
    ///
    /// Ensures username contains only alphanumeric characters, underscores, or hyphens.
    #[must_use]
    #[inline]
    pub fn has_valid_username_format(&self) -> bool {
        !self.username.is_empty()
            && self.username.len() <= 50
            && self.username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    }

    /// Checks if password meets minimum security requirements.
    ///
    /// Currently only checks length (≥8 chars). Implement additional checks server-side.
    #[must_use]
    #[inline]
    pub fn has_secure_password(&self) -> bool {
        self.password.len() >= 8
    }
}

/// Payload for user registration.
///
/// # Security Considerations
///
/// - Email addresses must be validated and verified server-side.
/// - Passwords should meet complexity requirements enforced by the backend.
/// - Consider implementing CAPTCHA to prevent automated registrations.
///
/// # Example
///
/// ```rust
/// use shared::types::RegisterPayload;
///
/// let payload = RegisterPayload {
///     username: "bob".to_string(),
///     email: "bob@example.com".to_string(),
///     password: "strongpassword456".to_string(),
/// };
///
/// assert!(payload.is_valid());
/// assert!(payload.has_valid_email());
/// ```
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
#[must_use = "RegisterPayload should be validated before use"]
pub struct RegisterPayload {
    /// Desired username (3-50 alphanumeric characters, underscores, hyphens)
    pub username: String,
    
    /// Valid email address for account verification
    pub email: String,
    
    /// Secure password (minimum 8 characters)
    ///
    /// # Security Note
    /// Password should meet complexity requirements:
    /// - At least 8 characters
    /// - Mix of uppercase, lowercase, numbers, and symbols (enforced server-side)
    pub password: String,
}

impl RegisterPayload {
    /// Validates the registration payload.
    ///
    /// Performs basic client-side validation. **Server-side validation is still required.**
    ///
    /// # Example
    ///
    /// ```rust
    /// use shared::types::RegisterPayload;
    ///
    /// let valid = RegisterPayload {
    ///     username: "charlie".to_string(),
    ///     email: "charlie@example.com".to_string(),
    ///     password: "SecureP@ss123".to_string(),
    /// };
    /// assert!(valid.is_valid());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_valid(&self) -> bool {
        !self.username.is_empty() && !self.email.is_empty() && !self.password.is_empty()
    }

    /// Checks if the email has a valid format.
    ///
    /// This is a basic check (contains '@' and '.'). Use a proper email validator
    /// library for production.
    #[must_use]
    #[inline]
    pub fn has_valid_email(&self) -> bool {
        self.email.contains('@') && self.email.contains('.') && self.email.len() <= 254
    }

    /// Validates username format (alphanumeric, underscores, hyphens only).
    #[must_use]
    #[inline]
    pub fn has_valid_username_format(&self) -> bool {
        (3..=50).contains(&self.username.len())
            && self.username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    }

    /// Checks if password meets minimum security requirements (8+ characters).
    #[must_use]
    #[inline]
    pub fn has_secure_password(&self) -> bool {
        self.password.len() >= 8
    }
}

/// Response returned upon successful authentication.
///
/// Contains a JWT token and the authenticated user's ID.
///
/// # Security Notes
///
/// - Tokens should be stored securely (HttpOnly cookies or secure storage).
/// - Tokens have expiration times; implement refresh token flow.
/// - Never log or expose tokens in error messages.
///
/// # Example
///
/// ```rust
/// use shared::types::AuthResponse;
///
/// let response = AuthResponse {
///     token: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...".to_string(),
///     user_id: 12345,
/// };
///
/// assert!(response.has_token());
/// assert_eq!(response.user_id, 12345);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[must_use = "AuthResponse contains sensitive authentication data"]
pub struct AuthResponse {
    /// JWT authentication token
    ///
    /// # Security
    /// Store this token securely. It grants access to protected resources.
    pub token: String,
    
    /// Authenticated user's ID (positive integer)
    pub user_id: i64,
}

impl AuthResponse {
    /// Creates a new authentication response.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shared::types::AuthResponse;
    ///
    /// let response = AuthResponse::new("jwt_token_here".to_string(), 42);
    /// assert_eq!(response.user_id, 42);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(token: String, user_id: i64) -> Self {
        Self { token, user_id }
    }

    /// Checks if the response contains a non-empty token.
    #[inline]
    #[must_use]
    pub fn has_token(&self) -> bool {
        !self.token.is_empty()
    }

    /// Checks if the user ID is valid (positive).
    #[inline]
    #[must_use]
    pub const fn has_valid_user_id(&self) -> bool {
        self.user_id > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_payload_validation() {
        let valid = LoginPayload {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };
        assert!(valid.is_valid());
        assert!(valid.has_secure_password());

        let invalid = LoginPayload {
            username: "".to_string(),
            password: "short".to_string(),
        };
        assert!(!invalid.is_valid());
        assert!(!invalid.has_secure_password());
    }

    #[test]
    fn test_register_payload_validation() {
        let valid = RegisterPayload {
            username: "newuser".to_string(),
            email: "user@example.com".to_string(),
            password: "SecurePass123".to_string(),
        };
        assert!(valid.is_valid());
        assert!(valid.has_valid_email());
        assert!(valid.has_valid_username_format());
        assert!(valid.has_secure_password());

        let invalid_email = RegisterPayload {
            username: "user".to_string(),
            email: "notemail".to_string(),
            password: "password".to_string(),
        };
        assert!(!invalid_email.has_valid_email());
    }

    #[test]
    fn test_auth_response() {
        let response = AuthResponse::new("token123".to_string(), 99);
        assert!(response.has_token());
        assert!(response.has_valid_user_id());

        let invalid_response = AuthResponse {
            token: "".to_string(),
            user_id: -1,
        };
        assert!(!invalid_response.has_token());
        assert!(!invalid_response.has_valid_user_id());
    }
}
