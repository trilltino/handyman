//! # Middleware Module
//!
//! ## Purpose
//! Axum middleware for request/response processing pipeline.
//! Handles authentication and error response mapping.
//!
//! ## Middleware Functions
//! - **mw_response_map**: Converts internal errors to safe client JSON responses
//!
//! ## Relation to Entire Program
//! - **Applied In**: main.rs router layers
//! - **Execution Order**: handler → res_map
//! - **Security**: Protects routes, hides internal errors from clients

pub mod mw_res_map; // Response error mapping middleware

pub use mw_res_map::mw_response_map;
