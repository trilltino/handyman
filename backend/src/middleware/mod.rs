//! # Middleware Module
//!
//! ## Purpose
//! Axum middleware for request/response processing pipeline.
//! Handles authentication and error response mapping.
//!
//! ## Middleware Functions
//! - **mw_ctx_resolver**: Validates JWT token and creates Ctx for protected routes
//! - **mw_ctx_require**: Ensures Ctx exists (fails if not authenticated)
//! - **mw_response_map**: Converts internal errors to safe client JSON responses
//!
//! ## Relation to Entire Program
//! - **Applied In**: main.rs router layers
//! - **Execution Order**: ctx_resolver → handler → ctx_require → res_map
//! - **Security**: Protects routes, hides internal errors from clients

pub mod mw_auth;      // Authentication middleware
pub mod mw_res_map;   // Response error mapping middleware

// Re-export middleware functions and types
pub use mw_auth::{mw_ctx_resolver, mw_ctx_require, CtxW, AUTH_TOKEN};
pub use mw_res_map::mw_response_map;
