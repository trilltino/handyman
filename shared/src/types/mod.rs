//! Core data types for API communication
//!
//! This module contains all data structures shared between the frontend and backend.
//! All types are serializable with serde for JSON communication.
//!
//! ## Modules
//! - `api` - Generic API response wrapper
//! - `contact` - Contact form submission data
//! - `product` - Product catalog and image data
//!
//! ## Types
//! - [`ApiResponse<T>`] - Generic response wrapper for all API endpoints
//! - [`ContactForm`] - Contact form submission data
//! - [`Product`] - Product for catalog display
//! - [`ProductImage`] - Product image metadata

pub mod api;
pub mod contact;
pub mod product;

pub use api::ApiResponse;
pub use contact::ContactForm;
pub use product::{Product, ProductImage, ProductWithImages};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_response_success() {
        let response = ApiResponse::success("Test", 42);
        assert!(response.success);
        assert_eq!(response.data, Some(42));
    }

    #[test]
    fn test_api_response_error() {
        let response: ApiResponse<()> = ApiResponse::error("Error");
        assert!(!response.success);
        assert_eq!(response.data, None);
    }
}
