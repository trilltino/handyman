//! Context requirement middleware.
//!
//! Checks that a valid Ctx was resolved by mw_ctx_resolver.
//! Used to protect routes that require authentication.

use super::mw_ctx_resolver::CtxExtResult;
use crate::error::{Error, Result};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use lib_core::Ctx;

/// Wrapper for Ctx that can be extracted in handlers.
///
/// Automatically fails request if no valid Ctx exists.
///
/// # Example
/// ```rust
/// async fn handler(CtxW(ctx): CtxW) -> Result<String> {
///     Ok(format!("Hello user {}", ctx.user_id()))
/// }
/// ```
#[derive(Debug, Clone)]
pub struct CtxW(pub Ctx);

/// Allow extracting CtxW in handlers.
///
/// This will fail the request if no valid Ctx was resolved.
impl<S: Send + Sync> FromRequestParts<S> for CtxW {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        // Extract CtxExtResult from extensions
        let ctx_result = parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::AuthRequired)?
            .clone();

        // Convert CtxExtResult to Ctx or error
        let ctx = ctx_result.map_err(|_| Error::InvalidToken)?;

        Ok(CtxW(ctx))
    }
}

/// Middleware to require valid context.
///
/// Apply this to routes/route groups that need authentication.
///
/// # Example
/// ```rust
/// let protected_routes = Router::new()
///     .route("/dashboard", get(dashboard))
///     .layer(from_fn(mw_ctx_require));
/// ```
pub async fn mw_ctx_require(
    ctx_result: Result<CtxW>,
    req: axum::extract::Request,
    next: Next,
) -> Result<Response> {
    // Fail fast if no valid context
    ctx_result?;

    Ok(next.run(req).await)
}
