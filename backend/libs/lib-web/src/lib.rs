//! # Web Layer Utilities
//!
//! Common utilities for building Axum web servers with consistent error
//! handling, middleware, and request/response patterns.
//!
//! ## Purpose
//!
//! This library provides the bridge between the core business logic layer
//! (lib-core) and the HTTP API layer. It ensures:
//! - Consistent error responses across all endpoints
//! - Standardized middleware chain composition
//! - Type-safe HTTP request/response handling
//! - Proper error conversion from domain to HTTP
//!
//! ## Modules
//!
//! - **[`error`]** - Web error types and HTTP status code mapping
//! - **[`middleware`]** - Custom middleware implementations
//!
//! ## Key Types
//!
//! - **[`Error`]** - Unified error type for web operations
//! - **[`Result<T>`]** - Web operation result type
//!
//! ## Architecture
//!
//! ```text
//! HTTP Request
//!     ↓
//! [Middleware Layer] ← Authentication, logging, etc.
//!     ↓
//! [Handler] ← Uses BMC from lib-core
//!     ↓
//! [Error Conversion] ← lib_web::Error converts errors
//!     ↓
//! HTTP Response
//! ```
//!
//! ## Error Handling Pattern
//!
//! Errors from lib-core (database, validation, etc.) are automatically
//! converted to appropriate HTTP status codes:
//! - `ModelError::EntityNotFound` → 404 Not Found
//! - `ModelError::InvalidData` → 400 Bad Request
//! - Auth errors → 401 Unauthorized
//! - Permission errors → 403 Forbidden
//! - Everything else → 500 Internal Server Error
//!
//! ## Example Handler
//!
//! ```rust
//! use axum::{extract::State, Json};
//! use lib_web::Result;
//!
//! async fn create_item(
//!     State(mm): State<lib_core::model::ModelManager>,
//!     Json(payload): Json<ItemRequest>,
//! ) -> Result<Json<ItemResponse>> {
//!     // Errors automatically convert to HTTP responses
//!     let item = ItemBmc::create(&mm, payload).await?;
//!     Ok(Json(ItemResponse { item }))
//! }
//! ```
//!
//! ## Usage
//! ```rust
//! use lib_web::prelude::*;
//! ```

pub mod error;
pub mod extractors;
pub mod middleware;
pub mod rate_limit;

// Re-export commonly used types
pub use error::{Error, Result};
pub use extractors::ValidatedJson;
pub use middleware::CtxW;

// Prelude for convenient importing
pub mod prelude {
    pub use crate::error::{Error as WebError, Result as WebResult};
    pub use crate::middleware::*;
}
