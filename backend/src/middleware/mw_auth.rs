//! # Authentication Middleware
//!
//! ## Purpose
//! JWT token validation and request context creation for protected routes.
//!
//! ## How It Works
//! 1. **mw_ctx_resolver**: Extracts JWT from cookie, validates, creates Ctx
//! 2. **mw_ctx_require**: Ensures Ctx exists (blocks unauthenticated requests)
//! 3. **CtxW**: Wrapper for Ctx that implements Axum extractor
//!
//! ## Middleware Flow
//! Client Request → Extract cookie → Parse JWT → Validate signature →
//! Check expiration → Fetch user → Create Ctx → Store in extensions →
//! Handler (extracts CtxW) → Response
//!
//! ## Relation to Entire Program
//! - **Used By**: All protected routes (logout, /me, future: bookings list)
//! - **Calls**: Token module (JWT validation), UserRepository (user lookup)
//! - **Security**: Prevents unauthorized access, validates every protected request

use crate::ctx::Ctx;                    // Request context with user ID
use crate::error::{Error, Result};      // Custom error types
use crate::repositories::UserRepository; // User database operations
use crate::token::{validate_token, Token};  // JWT validation
use axum::body::Body;                    // Request/response body
use axum::extract::{FromRequestParts, State};  // Axum extractors
use axum::http::request::Parts;          // Request parts
use axum::http::Request;                 // HTTP request
use axum::middleware::Next;              // Next middleware in chain
use axum::response::Response;            // HTTP response
use crate::db::DbPool;                   // Database connection pool
use tower_cookies::Cookies;              // Cookie handling

/// Cookie name for authentication token
pub const AUTH_TOKEN: &str = "auth-token";

/// Wrapper for Ctx that implements Axum extractor
/// Used in protected handlers to access authenticated user info
#[derive(Debug, Clone)]
pub struct CtxW(pub Ctx);

pub async fn mw_ctx_require(
    ctx_result: Result<CtxW>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    ctx_result?;
    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    State(pool): State<DbPool>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let ctx_result = resolve_ctx(pool, &cookies).await;
    
    // Store in request extensions
    req.extensions_mut().insert(ctx_result);
    
    next.run(req).await
}

async fn resolve_ctx(pool: DbPool, cookies: &Cookies) -> Result<CtxW> {
    // Get token from cookie
    let token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(Error::AuthFailNoAuthTokenCookie)?;
    
    // Parse token
    let token: Token = token.parse()?;
    
    // Get database connection
    let conn = pool.get().await.map_err(|e| Error::DatabaseError(e.to_string()))?;
    
    // Find user
    let user = UserRepository::find_for_auth(&conn, &token.ident)
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))?
        .ok_or(Error::AuthFailNoAuthTokenCookie)?;
    
    // Validate token
    validate_token(&token, user.token_salt)?;
    
    // Create context
    Ok(CtxW(Ctx::new(user.id)?))
}

impl<S: Send + Sync> FromRequestParts<S> for CtxW {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        parts
            .extensions
            .get::<Result<CtxW>>()
            .ok_or(Error::AuthFailCtxNotInRequestExt)?
            .clone()
    }
}
