//! Contact form data structure
//!
//! Used by both the frontend form component and backend API handler
//! to ensure type safety across the full stack.

use serde::{Deserialize, Serialize};

/// Contact form data structure.
///
/// Used by both the frontend form component and backend API handler
/// to ensure type safety across the full stack.
///
/// # Fields
///
/// * `name` - Sender's full name (1-100 characters)
/// * `email` - Sender's email address (valid format, max 254 characters)
/// * `message` - Message content (1-5000 characters)
///
/// # Example
///
/// ```rust
/// use shared::ContactForm;
///
/// let form = ContactForm {
///     name: "Jane Smith".to_string(),
///     email: "jane@example.com".to_string(),
///     message: "I'd like to learn more about your services.".to_string(),
/// };
///
/// assert!(form.validate().is_ok());
/// ```
///
/// # Validation
///
/// Call [`validate()`](ContactForm::validate) to ensure all fields meet requirements:
/// - Name: 1-100 characters
/// - Email: Valid format with '@'
/// - Message: 1-5000 characters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContactForm {
    /// Sender's name from contact form (1-100 chars)
    pub name: String,
    
    /// Sender's email for follow-up (valid format, max 254 characters)
    pub email: String,
    
    /// Message content from textarea (1-5000 chars after trimming)
    pub message: String,
}

impl ContactForm {
    /// Creates a new contact form with sanitized inputs.
    ///
    /// # Arguments
    ///
    /// * `name` - Sender's name (will be trimmed)
    /// * `email` - Sender's email (will be trimmed and lowercased)
    /// * `message` - Message content (will be trimmed)
    ///
    /// # Example
    ///
    /// ```rust
    /// use shared::ContactForm;
    ///
    /// let form = ContactForm::new(
    ///     "  John Doe  ",
    ///     "  JOHN@EXAMPLE.COM  ",
    ///     "  Hello!  "
    /// );
    ///
    /// assert_eq!(form.name, "John Doe");
    /// assert_eq!(form.email, "john@example.com");
    /// assert_eq!(form.message, "Hello!");
    /// ```
    #[must_use]
    pub fn new(name: impl Into<String>, email: impl Into<String>, message: impl Into<String>) -> Self {
        let name = name.into();
        let email = email.into();
        let message = message.into();
        
        Self {
            name: name.trim().to_string(),
            email: email.trim().to_lowercase(),
            message: message.trim().to_string(),
        }
    }

    /// Validates the contact form data.
    ///
    /// Checks that:
    /// - Name is between 1-100 characters (after trimming)
    /// - Email contains '@' and is max 254 characters
    /// - Message is between 1-5000 characters (after trimming)
    ///
    /// # Returns
    ///
    /// - `Ok(())` if all fields are valid
    /// - `Err(String)` with descriptive error message
    ///
    /// # Example
    ///
    /// ```rust
    /// use shared::ContactForm;
    ///
    /// let form = ContactForm {
    ///     name: "John".to_string(),
    ///     email: "john@example.com".to_string(),
    ///     message: "Hello".to_string(),
    /// };
    /// assert!(form.validate().is_ok());
    ///
    /// let invalid = ContactForm {
    ///     name: "".to_string(),
    ///     email: "invalid".to_string(),
    ///     message: "Hi".to_string(),
    /// };
    /// assert!(invalid.validate().is_err());
    /// ```
    #[must_use = "validation results should be checked"]
    pub fn validate(&self) -> Result<(), String> {
        // Validate name
        let name_len = self.name.trim().len();
        if name_len == 0 {
            return Err("Name is required".to_string());
        }
        if name_len > 100 {
            return Err("Name must be 100 characters or less".to_string());
        }

        // Validate email (basic check for WASM compatibility)
        let email_trimmed = self.email.trim();
        if email_trimmed.is_empty() {
            return Err("Email is required".to_string());
        }
        if !email_trimmed.contains('@') {
            return Err("Email must contain '@'".to_string());
        }
        if email_trimmed.len() > 254 {
            return Err("Email must be 254 characters or less".to_string());
        }

        // Validate message
        let message_len = self.message.trim().len();
        if message_len == 0 {
            return Err("Message is required".to_string());
        }
        if message_len > 5000 {
            return Err("Message must be 5000 characters or less".to_string());
        }

        Ok(())
    }

    /// Checks if the form is valid without returning an error message.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shared::ContactForm;
    ///
    /// let form = ContactForm {
    ///     name: "John".to_string(),
    ///     email: "john@example.com".to_string(),
    ///     message: "Hello".to_string(),
    /// };
    /// assert!(form.is_valid());
    /// ```
    #[must_use]
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    /// Returns a sanitized copy of the form with trimmed fields.
    ///
    /// Useful for cleaning user input before storage or processing.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shared::ContactForm;
    ///
    /// let form = ContactForm {
    ///     name: "  John  ".to_string(),
    ///     email: "  JOHN@EXAMPLE.COM  ".to_string(),
    ///     message: "  Hello  ".to_string(),
    /// };
    ///
    /// let sanitized = form.sanitized();
    /// assert_eq!(sanitized.name, "John");
    /// assert_eq!(sanitized.email, "john@example.com");
    /// assert_eq!(sanitized.message, "Hello");
    /// ```
    #[must_use]
    pub fn sanitized(&self) -> Self {
        Self {
            name: self.name.trim().to_string(),
            email: self.email.trim().to_lowercase(),
            message: self.message.trim().to_string(),
        }
    }

    /// Checks if the email has a valid format (basic check).
    ///
    /// This performs a simple check for '@' symbol. For production use,
    /// consider using a proper email validation library on the backend.
    #[must_use]
    #[inline]
    pub fn has_valid_email_format(&self) -> bool {
        let email = self.email.trim();
        email.contains('@') && email.len() <= 254 && !email.is_empty()
    }

    /// Checks if all required fields are non-empty after trimming.
    #[must_use]
    #[inline]
    pub fn has_all_fields(&self) -> bool {
        !self.name.trim().is_empty()
            && !self.email.trim().is_empty()
            && !self.message.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_contact_form() {
        let form = ContactForm {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            message: "Hello world".to_string(),
        };
        assert!(form.validate().is_ok());
    }

    #[test]
    fn test_empty_name() {
        let form = ContactForm {
            name: "".to_string(),
            email: "john@example.com".to_string(),
            message: "Hello".to_string(),
        };
        assert!(form.validate().is_err());
        assert!(form.validate().unwrap_err().contains("Name is required"));
    }

    #[test]
    fn test_name_too_long() {
        let form = ContactForm {
            name: "a".repeat(101),
            email: "john@example.com".to_string(),
            message: "Hello".to_string(),
        };
        assert!(form.validate().is_err());
    }

    #[test]
    fn test_invalid_email_no_at() {
        let form = ContactForm {
            name: "John".to_string(),
            email: "notanemail".to_string(),
            message: "Hello".to_string(),
        };
        assert!(form.validate().is_err());
        assert!(form.validate().unwrap_err().contains("@"));
    }

    #[test]
    fn test_email_too_long() {
        let form = ContactForm {
            name: "John".to_string(),
            email: format!("{}@example.com", "a".repeat(300)),
            message: "Hello".to_string(),
        };
        assert!(form.validate().is_err());
    }

    #[test]
    fn test_empty_message() {
        let form = ContactForm {
            name: "John".to_string(),
            email: "john@example.com".to_string(),
            message: "".to_string(),
        };
        assert!(form.validate().is_err());
    }

    #[test]
    fn test_message_too_long() {
        let form = ContactForm {
            name: "John".to_string(),
            email: "john@example.com".to_string(),
            message: "a".repeat(5001),
        };
        assert!(form.validate().is_err());
    }

    #[test]
    fn test_is_valid() {
        let valid = ContactForm {
            name: "John".to_string(),
            email: "john@example.com".to_string(),
            message: "Hello".to_string(),
        };
        assert!(valid.is_valid());

        let invalid = ContactForm {
            name: "".to_string(),
            email: "john@example.com".to_string(),
            message: "Hello".to_string(),
        };
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_serialization() {
        let form = ContactForm {
            name: "John".to_string(),
            email: "john@example.com".to_string(),
            message: "Hello".to_string(),
        };

        let json = serde_json::to_string(&form).unwrap();
        let deserialized: ContactForm = serde_json::from_str(&json).unwrap();
        assert_eq!(form, deserialized);
    }
}
