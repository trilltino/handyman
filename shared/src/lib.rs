//! # Shared Types Library
//!
//! ## Purpose
//! Shared data structures and types used by both frontend (Leptos/WASM) and backend (Axum).
//! Ensures type consistency across the full stack.
//!
//! ## Modules
//! - **types** - Core data structures (`ContactForm`, `ApiResponse`, `Product`)
//! - **metadata** - SEO metadata for pages (`PageMetadata`)
//! - **schema** - Structured data generators (JSON-LD schemas)
//! - **error** - Shared error types
//!
//! ## Why Shared?
//! - **Type Safety**: Frontend and backend use identical types (compile-time validation)
//! - **DRY Principle**: Define types once, use everywhere
//! - **Serialization**: Same serde attributes for JSON serialization
//!
//! ## Relation to Entire Program
//! - **Used By**: Both frontend (form validation, metadata) and backend (API responses, SEO)
//! - **Compiled Twice**: Once for WASM (frontend), once for native (backend)
//!
//! ## Example
//!
//! ```rust
//! use shared::{ContactForm, ApiResponse, Product, PageMetadata};
//!
//! // Create and validate a contact form
//! let form = ContactForm {
//!     name: "John Doe".to_string(),
//!     email: "john@example.com".to_string(),
//!     message: "Hello!".to_string(),
//! };
//! assert!(form.validate().is_ok());
//!
//! // Create an API response
//! let response = ApiResponse::success("Contact saved", ());
//!
//! // Get page metadata for SEO
//! let metadata = PageMetadata::for_homepage();
//! ```

pub mod error;
pub mod metadata;
pub mod newtypes;
pub mod schema;
pub mod types;
pub mod validation;

pub use error::{SharedError, SharedResult};
pub use metadata::{PageMetadata, FULL_BUSINESS_DESCRIPTION};
pub use newtypes::{Email, NonEmptyString, PhoneNumber, PositiveInt, PriceCents};
pub use types::{ApiResponse, ContactForm, Product, ProductImage, ProductWithImages};
pub use validation::Validate;
