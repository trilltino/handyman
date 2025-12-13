//! HTTP middleware for Axum.
//!
//! ## Middleware Stack Order (from outermost to innermost):
//! 1. `mw_req_stamp` - Request UUID + timestamp (first)
//! 2. `mw_ctx_resolver` - Extract & validate JWT token
//! 3. `mw_ctx_require` - Require valid context (per-route)
//! 4. Route handler
//! 5. `mw_res_map` - Map errors to responses (last)

mod mw_ctx_require;
mod mw_ctx_resolver;
mod mw_req_stamp;
mod mw_res_map;

pub use mw_ctx_require::{mw_ctx_require, CtxW};
pub use mw_ctx_resolver::{mw_ctx_resolver, CtxExtResult};
pub use mw_req_stamp::{mw_req_stamp_resolver, ReqStamp};
pub use mw_res_map::mw_response_map;
