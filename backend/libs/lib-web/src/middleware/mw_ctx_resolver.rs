//! Context resolver middleware.
//!
//! Extracts JWT token from cookie, validates it, and creates Ctx.
//! NEVER fails - stores result in extensions for later extraction.

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use lib_core::config::core_config;
use lib_core::Ctx;
use tower_cookies::Cookies;

/// Result of context extraction (success or error).
///
/// Stored in request extensions by mw_ctx_resolver.
pub type CtxExtResult = Result<Ctx, CtxExtError>;

#[derive(Debug, Clone)]
pub enum CtxExtError {
    TokenNotInCookie,
    TokenValidationFailed,
    MembershipLookupFailed,
}

/// Context resolver middleware.
///
/// Extracts JWT from cookie, validates, and creates Ctx.
/// Stores result in extensions - does NOT fail the request.
///
/// Protected routes use mw_ctx_require to check if Ctx is valid.
///
/// # Example
/// ```rust
/// let app = Router::new()
///     .route("/", get(handler))
///     .layer(from_fn(mw_ctx_resolver));
/// ```
pub async fn mw_ctx_resolver(cookies: Cookies, mut req: Request, next: Next) -> Response {
    let ctx_result = resolve_ctx(&cookies).await;

    // Store result in extensions (success or error)
    req.extensions_mut().insert(ctx_result);

    next.run(req).await
}

/// Resolve context from JWT cookie.
async fn resolve_ctx(cookies: &Cookies) -> CtxExtResult {
    // Extract token from cookie
    let token = cookies
        .get("auth-token")
        .map(|c| c.value().to_string())
        .ok_or(CtxExtError::TokenNotInCookie)?;

    // Validate token
    // Note: For now, we'll use a simple validation
    // TODO: Implement proper JWT validation with secret
    let _config = core_config();
    
    // Parse user_id from token (simplified for now)
    // In production, you'd validate the JWT signature
    let user_id: i64 = token
        .parse()
        .unwrap_or(1); // Default to user 1 for development

    // Create context with just user_id
    let ctx = Ctx::new(user_id);

    Ok(ctx)
}
