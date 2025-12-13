//! Generic API response wrapper
//!
//! Provides a standardized response format for all API endpoints,
//! ensuring consistent error handling and data serialization.

use serde::{Deserialize, Serialize};

/// Generic API response wrapper.
///
/// Standardizes response format across all API endpoints, making it easier
/// for frontend code to handle responses consistently.
///
/// # Type Parameters
///
/// * `T` - The type of data payload (varies by endpoint)
///
/// # Fields
///
/// * `success` - `true` if operation succeeded, `false` otherwise
/// * `message` - Human-readable message describing the result
/// * `data` - Optional data payload (present on success, absent on error)
///
/// # Example
///
/// ```rust
/// use shared::ApiResponse;
///
/// // Success response with data
/// let response = ApiResponse::success("User created", 42);
/// assert!(response.success);
/// assert_eq!(response.data, Some(42));
///
/// // Error response without data
/// let error: ApiResponse<()> = ApiResponse::error("Invalid input");
/// assert!(!error.success);
/// assert_eq!(error.data, None);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use = "API responses should be handled"]
pub struct ApiResponse<T> {
    /// Success flag (true if operation succeeded)
    pub success: bool,
    /// Human-readable message
    pub message: String,
    /// Optional data payload (type varies by endpoint)
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    /// Creates a successful response with data.
    ///
    /// # Arguments
    ///
    /// * `message` - Success message to display to the user
    /// * `data` - Data payload to include in the response
    ///
    /// # Example
    ///
    /// ```rust
    /// use shared::ApiResponse;
    ///
    /// let response = ApiResponse::success("Operation completed", vec![1, 2, 3]);
    /// assert!(response.success);
    /// ```
    #[must_use]
    #[inline]
    pub fn success(message: impl Into<String>, data: T) -> Self {
        Self {
            success: true,
            message: message.into(),
            data: Some(data),
        }
    }

    /// Creates an error response without data.
    ///
    /// # Arguments
    ///
    /// * `message` - Error message to display to the user
    ///
    /// # Example
    ///
    /// ```rust
    /// use shared::ApiResponse;
    ///
    /// let response: ApiResponse<()> = ApiResponse::error("Database connection failed");
    /// assert!(!response.success);
    /// assert_eq!(response.data, None);
    /// ```
    #[must_use]
    #[inline]
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            data: None,
        }
    }

    /// Maps the response data to a different type.
    ///
    /// Useful for transforming the response payload.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shared::ApiResponse;
    ///
    /// let response = ApiResponse::success("OK", vec![1, 2, 3]);
    /// let mapped = response.map(|data| data.map(|v| v.len()));
    /// assert_eq!(mapped.data, Some(Some(3)));
    /// ```
    #[must_use]
    pub fn map<U>(self, f: impl FnOnce(Option<T>) -> Option<U>) -> ApiResponse<U> {
        ApiResponse {
            success: self.success,
            message: self.message,
            data: f(self.data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_response() {
        let response = ApiResponse::success("User created", 42);
        assert!(response.success);
        assert_eq!(response.message, "User created");
        assert_eq!(response.data, Some(42));
    }

    #[test]
    fn test_error_response() {
        let response: ApiResponse<i32> = ApiResponse::error("Not found");
        assert!(!response.success);
        assert_eq!(response.message, "Not found");
        assert_eq!(response.data, None);
    }

    #[test]
    fn test_map_response() {
        let response = ApiResponse::success("OK", vec![1, 2, 3]);
        let mapped = response.map(|data| data.map(|v| v.len()));
        assert!(mapped.success);
        assert_eq!(mapped.data, Some(3));
    }

    #[test]
    fn test_serialization() {
        let response = ApiResponse::success("OK", 42);
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"success\":true"));
        assert!(json.contains("\"data\":42"));
    }
}
