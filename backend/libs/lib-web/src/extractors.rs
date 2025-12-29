//! # Web Extractors
//!
//! Custom Axum extractors for common web patterns.
use crate::Error;
use axum::extract::{FromRequest, Json, Request};
use serde::de::DeserializeOwned;
use shared::validation::Validate;

/// Extractor that validates the JSON payload using the `shared::validation::Validate` trait.
///
/// If the payload is invalid, it returns a `400 Bad Request` with the validation error.
/// This prevents handler logic from executing with invalid data.
pub struct ValidatedJson<T>(pub T);
impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate + 'static,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|rejection| Error::ValidationError(rejection.to_string().into()))?;

        value.validate().map_err(Error::ValidationError)?;

        Ok(ValidatedJson(value))
    }
}

impl<T> std::ops::Deref for ValidatedJson<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;
    use shared::ContactForm;

    #[test]
    fn test_validated_json_deref() {
        let form = ContactForm {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            message: "Hello".to_string(),
        };
        let validated = ValidatedJson(form);

        // Test Deref trait
        assert_eq!(validated.name, "Test User");
        assert_eq!(validated.email, "test@example.com");
        assert_eq!(validated.message, "Hello");
    }

    #[test]
    fn test_validated_json_inner_access() {
        let form = ContactForm {
            name: "Jane Doe".to_string(),
            email: "jane@example.com".to_string(),
            message: "Test message".to_string(),
        };
        let validated = ValidatedJson(form);

        // Access inner value via .0
        assert_eq!(validated.0.name, "Jane Doe");
    }
}

// endregion: --- Tests
