//! # Input Validation Helpers
//!
//! Centralized validation functions for common fields and constraints.
//!
//! ## Usage
//!
//! Use these validators in your request types or handlers:
//!
//! ```rust,no_run
//! use crate::validators;
//! use lib_web::Error;
//!
//! fn validate_request(name: &str) -> Result<(), Error> {
//!     validators::validate_length(name, 1, 100, "Name")?;
//!     Ok(())
//! }
//! ```

use lib_web::Error;

/// Validates an email address format.
///
/// # Errors
///
/// Returns `ValidationError` if:
/// - Email is empty or only whitespace
/// - Email doesn't contain '@' character
///
/// # Example
///
/// ```rust,no_run
/// # use crate::validators::validate_email;
/// validate_email("user@example.com")?; // OK
/// validate_email("invalid-email")?; // Error
/// ```
#[allow(dead_code)]
pub fn validate_email(email: &str) -> Result<(), Error> {
    let email = email.trim();
    if email.is_empty() {
        return Err(Error::ValidationError("Email is required".into()));
    }
    if !email.contains('@') || !email.contains('.') {
        return Err(Error::ValidationError(
            "Please provide a valid email address".into(),
        ));
    }
    if email.len() > 254 {
        return Err(Error::ValidationError("Email is too long".into()));
    }
    Ok(())
}

/// Validates string length constraints.
///
/// # Arguments
///
/// * `value` - String to validate
/// * `min` - Minimum length (inclusive)
/// * `max` - Maximum length (inclusive)
/// * `field` - Field name for error messages
///
/// # Errors
///
/// Returns `ValidationError` if length is outside [min, max] range.
///
/// # Example
///
/// ```rust,no_run
/// # use crate::validators::validate_length;
/// validate_length("John Doe", 1, 100, "Name")?; // OK
/// validate_length("", 1, 100, "Name")?; // Error
/// ```
#[allow(dead_code)]
pub fn validate_length(value: &str, min: usize, max: usize, field: &str) -> Result<(), Error> {
    let len = value.trim().len();
    if len < min {
        return Err(Error::ValidationError(format!(
            "{} must be at least {} characters",
            field, min
        )));
    }
    if len > max {
        return Err(Error::ValidationError(format!(
            "{} must be no more than {} characters",
            field, max
        )));
    }
    Ok(())
}

/// Validates a price/currency value.
///
/// # Errors
///
/// Returns `ValidationError` if:
/// - Price is negative
/// - Price is NaN or infinite
///
/// # Example
///
/// ```rust,no_run
/// # use crate::validators::validate_price;
/// validate_price(19.99)?; // OK
/// validate_price(-5.0)?; // Error
/// ```
#[allow(dead_code)]
pub fn validate_price(price: f64) -> Result<(), Error> {
    if price < 0.0 {
        return Err(Error::ValidationError("Price cannot be negative".into()));
    }
    if price.is_nan() || price.is_infinite() {
        return Err(Error::ValidationError("Invalid price".into()));
    }
    Ok(())
}

/// Validates a non-empty string (after trimming).
///
/// # Errors
///
/// Returns `ValidationError` if string is empty or only whitespace.
///
/// # Example
///
/// ```rust,no_run
/// # use crate::validators::validate_required;
/// validate_required("John", "Name")?; // OK
/// validate_required("", "Name")?; // Error
/// ```
#[allow(dead_code)]
pub fn validate_required(value: &str, field: &str) -> Result<(), Error> {
    if value.trim().is_empty() {
        return Err(Error::ValidationError(format!("{} is required", field)));
    }
    Ok(())
}

/// Validates an integer falls within a range.
///
/// # Errors
///
/// Returns `ValidationError` if value is outside [min, max] range.
///
/// # Example
///
/// ```rust,no_run
/// # use crate::validators::validate_range;
/// validate_range(5, 1, 10, "Quantity")?; // OK
/// validate_range(15, 1, 10, "Quantity")?; // Error
/// ```
#[allow(dead_code)]
pub fn validate_range(value: i32, min: i32, max: i32, field: &str) -> Result<(), Error> {
    if value < min || value > max {
        return Err(Error::ValidationError(format!(
            "{} must be between {} and {}",
            field, min, max
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email_valid() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("test.name@domain.co.uk").is_ok());
    }

    #[test]
    fn test_validate_email_invalid() {
        assert!(validate_email("").is_err());
        assert!(validate_email("invalid-email").is_err());
        assert!(validate_email("@example.com").is_err());
    }

    #[test]
    fn test_validate_length_valid() {
        assert!(validate_length("John", 1, 100, "Name").is_ok());
    }

    #[test]
    fn test_validate_length_invalid() {
        assert!(validate_length("", 1, 100, "Name").is_err());
        assert!(validate_length("a", 2, 100, "Name").is_err());
        assert!(validate_length("a".repeat(101), 1, 100, "Name").is_err());
    }

    #[test]
    fn test_validate_price_valid() {
        assert!(validate_price(0.0).is_ok());
        assert!(validate_price(19.99).is_ok());
    }

    #[test]
    fn test_validate_price_invalid() {
        assert!(validate_price(-1.0).is_err());
        assert!(validate_price(f64::NAN).is_err());
        assert!(validate_price(f64::INFINITY).is_err());
    }

    #[test]
    fn test_validate_required() {
        assert!(validate_required("John", "Name").is_ok());
        assert!(validate_required("", "Name").is_err());
        assert!(validate_required("   ", "Name").is_err());
    }

    #[test]
    fn test_validate_range() {
        assert!(validate_range(5, 1, 10, "Qty").is_ok());
        assert!(validate_range(15, 1, 10, "Qty").is_err());
        assert!(validate_range(0, 1, 10, "Qty").is_err());
    }
}
