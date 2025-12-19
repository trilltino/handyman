//! Validation trait and implementations for all request types.
//!
//! This module provides a type-safe validation framework for all shared types.
//! The trait itself uses `String` errors for WASM compatibility while remaining
//! compatible with backend error types.
//!
//! ## Usage in Backend
//!
//! ```rust,no_run
//! use shared::validation::Validate;
//! use shared::ContactForm;
//! # fn example() -> Result<(), String> {
//! let form = ContactForm {
//!     name: "John".to_string(),
//!     email: "john@example.com".to_string(),
//!     message: "Hello".to_string(),
//! };
//!
//! // Validate and handle errors using the `?` operator
//! form.validate()?;
//! # Ok(())
//! # }
//! ```

use crate::types::{ContactForm, Product, ProductImage};

use std::borrow::Cow;

/// Trait for validating data structures.
///
/// All input types (especially those coming from external sources like forms or APIs)
/// should implement this trait to ensure consistent, type-safe validation.
///
/// # Returns
///
/// - `Ok(())` if validation passes
/// - `Err(Cow<'static, str>)` with a descriptive error message if validation fails
///
/// # Example
///
/// ```rust
/// use shared::validation::Validate;
/// use shared::ContactForm;
///
/// let form = ContactForm {
///     name: "Alice".to_string(),
///     email: "alice@example.com".to_string(),
///     message: "Hello!".to_string(),
/// };
///
/// assert!(form.validate().is_ok());
/// ```
pub trait Validate {
    /// Validates the value and returns an error if invalid.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the value is valid
    /// - `Err(Cow<'static, str>)` with a descriptive, user-friendly error message if validation fails
    ///
    /// # Errors
    ///
    /// Returns an error if any validation constraints are violated.
    #[must_use = "validation results should be checked"]
    fn validate(&self) -> Result<(), Cow<'static, str>>;

    /// Convenience method to check if the value is valid without error details.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shared::validation::Validate;
    /// use shared::ContactForm;
    ///
    /// let form = ContactForm {
    ///     name: "Bob".to_string(),
    ///     email: "bob@example.com".to_string(),
    ///     message: "Hi".to_string(),
    /// };
    ///
    /// assert!(form.is_valid());
    /// ```
    #[must_use]
    #[inline]
    fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
}

impl Validate for ContactForm {
    /// Validates contact form data.
    ///
    /// # Validation Rules
    ///
    /// - Name: must not be empty after trimming whitespace
    /// - Email: must not be empty and must contain '@'
    /// - Message: must not be empty after trimming whitespace
    ///
    /// # Errors
    ///
    /// Returns a descriptive error message for the first validation failure encountered.
    fn validate(&self) -> Result<(), Cow<'static, str>> {
        // Validate name (trim whitespace for better UX)
        if self.name.trim().is_empty() {
            return Err(Cow::Borrowed("Name is required"));
        }

        // Validate email
        let trimmed_email = self.email.trim();
        if trimmed_email.is_empty() {
            return Err(Cow::Borrowed("Email is required"));
        }
        if !trimmed_email.contains('@') {
            return Err(Cow::Borrowed("Email must be valid (must contain '@')"));
        }

        // Validate message
        if self.message.trim().is_empty() {
            return Err(Cow::Borrowed("Message is required"));
        }

        Ok(())
    }
}

impl Validate for Product {
    /// Validates product data.
    ///
    /// # Validation Rules
    ///
    /// - Name: must not be empty after trimming whitespace
    ///
    /// # Errors
    ///
    /// Returns an error if the product name is empty.
    fn validate(&self) -> Result<(), Cow<'static, str>> {
        if self.name.trim().is_empty() {
            return Err(Cow::Borrowed("Product name is required"));
        }
        Ok(())
    }
}

impl Validate for ProductImage {
    /// Validates product image data.
    ///
    /// # Validation Rules
    ///
    /// - Image URL: must not be empty after trimming whitespace
    ///
    /// # Errors
    ///
    /// Returns an error if the image URL is empty.
    fn validate(&self) -> Result<(), Cow<'static, str>> {
        if self.image_url.trim().is_empty() {
            return Err(Cow::Borrowed("Image URL is required"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_form_validate_success() {
        let form = ContactForm {
            name: "John".to_string(),
            email: "john@example.com".to_string(),
            message: "Hello".to_string(),
        };
        assert!(form.validate().is_ok());
        assert!(form.is_valid());
    }

    #[test]
    fn test_contact_form_validate_whitespace_trimming() {
        let form = ContactForm {
            name: "  John  ".to_string(),
            email: "  john@example.com  ".to_string(),
            message: "  Hello  ".to_string(),
        };
        assert!(form.validate().is_ok());
    }

    #[test]
    fn test_contact_form_validate_empty_name() {
        let form = ContactForm {
            name: "".to_string(),
            email: "john@example.com".to_string(),
            message: "Hello".to_string(),
        };
        let result = form.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Name"));
    }

    #[test]
    fn test_contact_form_validate_empty_email() {
        let form = ContactForm {
            name: "John".to_string(),
            email: "".to_string(),
            message: "Hello".to_string(),
        };
        let result = form.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Email"));
    }

    #[test]
    fn test_contact_form_validate_invalid_email() {
        let form = ContactForm {
            name: "John".to_string(),
            email: "notanemail".to_string(),
            message: "Hello".to_string(),
        };
        let result = form.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("@"));
    }

    #[test]
    fn test_contact_form_validate_empty_message() {
        let form = ContactForm {
            name: "John".to_string(),
            email: "john@example.com".to_string(),
            message: "".to_string(),
        };
        let result = form.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Message"));
    }

    #[test]
    fn test_product_validate_success() {
        let product = Product::new(1, "Website".to_string(), 500.0);
        assert!(product.validate().is_ok());
    }

    #[test]
    fn test_product_validate_empty_name() {
        let product = Product::new(1, "".to_string(), 500.0);
        let result = product.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Product name"));
    }

    #[test]
    fn test_product_image_validate_success() {
        let image = ProductImage {
            id: 1,
            product_id: 1,
            image_url: "https://example.com/image.jpg".to_string(),
            alt_text: Some("Product image".to_string()),
            is_primary: true,
            display_order: 0,
            created_at: None,
        };
        assert!(image.validate().is_ok());
    }

    #[test]
    fn test_product_image_validate_empty_url() {
        let image = ProductImage {
            id: 1,
            product_id: 1,
            image_url: "".to_string(),
            alt_text: None,
            is_primary: false,
            display_order: 0,
            created_at: None,
        };
        let result = image.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Image URL"));
    }
}
