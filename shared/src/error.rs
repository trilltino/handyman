//! Shared error types
//!
//! Error types used across the shared crate for consistent error handling.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Result type for shared crate operations
pub type SharedResult<T> = Result<T, SharedError>;

/// Shared error type for validation and serialization errors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SharedError {
    /// Validation error with descriptive message
    ValidationError(String),
    /// Serialization/deserialization error
    SerializationError(String),
    /// Resource not found error
    NotFound(String),
}

impl fmt::Display for SharedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ValidationError(msg) => write!(f, "Validation error: {msg}"),
            Self::SerializationError(msg) => write!(f, "Serialization error: {msg}"),
            Self::NotFound(msg) => write!(f, "Not found: {msg}"),
        }
    }
}

impl std::error::Error for SharedError {}

impl SharedError {
    /// Creates a validation error
    #[must_use]
    #[inline]
    pub fn validation(message: impl Into<String>) -> Self {
        Self::ValidationError(message.into())
    }

    /// Creates a serialization error
    #[must_use]
    #[inline]
    pub fn serialization(message: impl Into<String>) -> Self {
        Self::SerializationError(message.into())
    }

    /// Creates a not found error
    #[must_use]
    #[inline]
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }

    /// Checks if this is a validation error
    #[must_use]
    pub const fn is_validation_error(&self) -> bool {
        matches!(self, Self::ValidationError(_))
    }

    /// Checks if this is a serialization error
    #[must_use]
    pub const fn is_serialization_error(&self) -> bool {
        matches!(self, Self::SerializationError(_))
    }

    /// Checks if this is a not found error
    #[must_use]
    pub const fn is_not_found(&self) -> bool {
        matches!(self, Self::NotFound(_))
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        match self {
            Self::ValidationError(msg) => msg,
            Self::SerializationError(msg) => msg,
            Self::NotFound(msg) => msg,
        }
    }
}

impl From<String> for SharedError {
    fn from(msg: String) -> Self {
        Self::ValidationError(msg)
    }
}

impl From<&str> for SharedError {
    fn from(msg: &str) -> Self {
        Self::ValidationError(msg.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_display() {
        let error = SharedError::ValidationError("Invalid email".to_string());
        assert_eq!(error.to_string(), "Validation error: Invalid email");
    }

    #[test]
    fn test_serialization_error_display() {
        let error = SharedError::SerializationError("Failed to parse JSON".to_string());
        assert_eq!(
            error.to_string(),
            "Serialization error: Failed to parse JSON"
        );
    }

    #[test]
    fn test_not_found_error_display() {
        let error = SharedError::NotFound("User not found".to_string());
        assert_eq!(error.to_string(), "Not found: User not found");
    }

    #[test]
    fn test_error_serialization() {
        let error = SharedError::ValidationError("Test".to_string());
        let json = serde_json::to_string(&error).unwrap();
        let deserialized: SharedError = serde_json::from_str(&json).unwrap();
        assert_eq!(error, deserialized);
    }

    #[test]
    fn test_error_validation_helper() {
        let error = SharedError::validation("Test validation error");
        assert!(error.is_validation_error());
        assert!(!error.is_serialization_error());
    }

    #[test]
    fn test_error_serialization_helper() {
        let error = SharedError::serialization("Test serialization error");
        assert!(error.is_serialization_error());
        assert!(!error.is_validation_error());
    }

    #[test]
    fn test_error_not_found_helper() {
        let error = SharedError::not_found("Test not found error");
        assert!(error.is_not_found());
        assert!(!error.is_validation_error());
    }

    #[test]
    fn test_error_message() {
        let error = SharedError::validation("Test message");
        assert_eq!(error.message(), "Test message");
    }

    #[test]
    fn test_error_from_string() {
        let error: SharedError = "Test".to_string().into();
        assert!(error.is_validation_error());
    }

    #[test]
    fn test_error_from_str() {
        let error: SharedError = "Test".into();
        assert!(error.is_validation_error());
    }
}
