//! # Newtype Wrappers
//!
//! Type-safe wrappers for common primitive types to provide compile-time
//! validation and prevent misuse.
//!
//! ## Example
//!
//! ```rust
//! use shared::newtypes::{Email, PhoneNumber};
//!
//! // Type-safe email handling
//! let email = Email::new("john@example.com")?;
//! let email_str: &str = email.as_str();
//!
//! // Type-safe phone number
//! let phone = PhoneNumber::new("+44 7833 263486")?;
//! ```

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;

/// Validated email address newtype.
///
/// Ensures the string contains an `@` symbol and is not empty.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Email(String);

impl Email {
    /// Creates a new validated Email.
    ///
    /// # Arguments
    ///
    /// * `email` - Email string to validate
    ///
    /// # Returns
    ///
    /// - `Ok(Email)` if the email is valid
    /// - `Err` with validation message if invalid
    ///
    /// # Example
    ///
    /// ```rust
    /// use shared::newtypes::Email;
    ///
    /// let valid = Email::new("user@example.com");
    /// assert!(valid.is_ok());
    ///
    /// let invalid = Email::new("not-an-email");
    /// assert!(invalid.is_err());
    /// ```
    pub fn new(email: impl Into<String>) -> Result<Self, Cow<'static, str>> {
        let email = email.into();
        let trimmed = email.trim();

        if trimmed.is_empty() {
            return Err(Cow::Borrowed("Email cannot be empty"));
        }

        if !trimmed.contains('@') {
            return Err(Cow::Borrowed("Email must contain @"));
        }

        // Basic format check: something@something.something
        let parts: Vec<&str> = trimmed.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(Cow::Borrowed("Invalid email format"));
        }

        if !parts[1].contains('.') {
            return Err(Cow::Borrowed("Email domain must contain a period"));
        }

        Ok(Email(trimmed.to_lowercase()))
    }

    /// Returns the email as a string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the Email and returns the inner String.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Validated phone number newtype.
///
/// Ensures the phone number contains only valid characters.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    /// Creates a new validated PhoneNumber.
    ///
    /// Allows digits, spaces, hyphens, plus signs, and parentheses.
    pub fn new(phone: impl Into<String>) -> Result<Self, Cow<'static, str>> {
        let phone = phone.into();
        let trimmed = phone.trim();

        if trimmed.is_empty() {
            return Err(Cow::Borrowed("Phone number cannot be empty"));
        }

        // Only allow digits, spaces, hyphens, plus, and parentheses
        let valid_chars = trimmed.chars().all(|c| {
            c.is_ascii_digit() || c == ' ' || c == '-' || c == '+' || c == '(' || c == ')'
        });

        if !valid_chars {
            return Err(Cow::Borrowed("Phone number contains invalid characters"));
        }

        // Must contain at least 7 digits
        let digit_count = trimmed.chars().filter(|c| c.is_ascii_digit()).count();
        if digit_count < 7 {
            return Err(Cow::Borrowed("Phone number must have at least 7 digits"));
        }

        Ok(PhoneNumber(trimmed.to_string()))
    }

    /// Returns the phone number as a string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns only the digits (for tel: links).
    pub fn digits_only(&self) -> String {
        self.0
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '+')
            .collect()
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Non-empty string newtype.
///
/// Guarantees the string is not empty after trimming.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    /// Creates a new NonEmptyString.
    pub fn new(s: impl Into<String>) -> Result<Self, Cow<'static, str>> {
        let s = s.into();
        let trimmed = s.trim();

        if trimmed.is_empty() {
            return Err(Cow::Borrowed("String cannot be empty"));
        }

        Ok(NonEmptyString(trimmed.to_string()))
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for NonEmptyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Positive integer (guaranteed > 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PositiveInt(i32);

impl PositiveInt {
    /// Creates a new PositiveInt if value > 0.
    pub fn new(value: i32) -> Result<Self, Cow<'static, str>> {
        if value <= 0 {
            return Err(Cow::Borrowed("Value must be positive"));
        }
        Ok(PositiveInt(value))
    }

    #[inline]
    pub fn get(&self) -> i32 {
        self.0
    }
}

/// Price in cents (non-negative).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PriceCents(i32);

impl PriceCents {
    /// Creates a new PriceCents if value >= 0.
    pub fn new(cents: i32) -> Result<Self, Cow<'static, str>> {
        if cents < 0 {
            return Err(Cow::Borrowed("Price cannot be negative"));
        }
        Ok(PriceCents(cents))
    }

    /// Creates a PriceCents from pounds and pence.
    pub fn from_pounds(pounds: i32, pence: i32) -> Result<Self, Cow<'static, str>> {
        let cents = pounds * 100 + pence;
        Self::new(cents)
    }

    #[inline]
    pub fn get(&self) -> i32 {
        self.0
    }

    /// Returns as pounds (decimal).
    pub fn as_pounds(&self) -> f64 {
        self.0 as f64 / 100.0
    }

    /// Formats as GBP string (e.g., "£45.00").
    pub fn to_gbp_string(&self) -> String {
        format!("£{:.2}", self.as_pounds())
    }
}

impl fmt::Display for PriceCents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_gbp_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_valid() {
        assert!(Email::new("test@example.com").is_ok());
        assert!(Email::new("user.name@domain.co.uk").is_ok());
    }

    #[test]
    fn test_email_invalid() {
        assert!(Email::new("").is_err());
        assert!(Email::new("noat").is_err());
        assert!(Email::new("@nodomain").is_err());
        assert!(Email::new("noperiod@domain").is_err());
    }

    #[test]
    fn test_phone_valid() {
        assert!(PhoneNumber::new("+44 7833 263486").is_ok());
        assert!(PhoneNumber::new("07833263486").is_ok());
    }

    #[test]
    fn test_phone_invalid() {
        assert!(PhoneNumber::new("").is_err());
        assert!(PhoneNumber::new("123").is_err()); // Too short
        assert!(PhoneNumber::new("hello").is_err()); // Invalid chars
    }

    #[test]
    fn test_price_cents() {
        let price = PriceCents::new(4500).unwrap();
        assert_eq!(price.as_pounds(), 45.0);
        assert_eq!(price.to_gbp_string(), "£45.00");
    }
}
