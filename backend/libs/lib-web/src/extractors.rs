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

        value.validate().map_err(|e| Error::ValidationError(e))?;

        Ok(ValidatedJson(value))
    }
}

impl<T> std::ops::Deref for ValidatedJson<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
