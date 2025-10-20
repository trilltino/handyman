//! # Authentication Handlers
//!
//! ## Purpose
//! Handles user authentication: registration, login, logout, current user info.
//! Manages JWT token generation and cookie-based session management.
//!
//! ## Endpoints
//! - **POST /api/register**: Create new user account
//! - **POST /api/login**: Authenticate user and set auth cookie
//! - **POST /api/logout**: Clear auth cookie
//! - **GET /api/me**: Get current logged-in user info (protected)
//!
//! ## Authentication Flow
//! 1. User submits credentials
//! 2. Verify password hash
//! 3. Generate JWT token with user salt
//! 4. Set httpOnly cookie with token
//! 5. Middleware validates token on protected requests
//!
//! ## Relation to Entire Program
//! - **Called By**: Axum router for auth endpoints
//! - **Calls**: UserRepository (database), token module (JWT generation)
//! - **Security**: httpOnly cookies prevent XSS, tokens expire in 24h

use crate::db::DbPool;                    // Database connection pool
use crate::error::{Error, Result};        // Custom error types
use crate::middleware::AUTH_TOKEN;        // Cookie name constant
use crate::models::{UserForCreate, verify_password};  // User models and password verification
use crate::repositories::UserRepository;  // User database operations
use crate::token::generate_token;         // JWT token generation
use axum::{extract::State, http::StatusCode};  // Axum HTTP types
use axum::Json;                           // JSON request/response
use serde::{Deserialize, Serialize};      // JSON serialization
use serde_json::{json, Value};            // JSON helpers
use tower_cookies::{Cookie, Cookies};     // Cookie management

/// Registration request payload from frontend
#[derive(Debug, Deserialize)]
pub struct RegisterPayload {
    pub username: String,  // Desired username (must be unique)
    pub email: String,     // User email (must be unique)
    pub password: String,  // Plain-text password (will be hashed)
}

/// Login request payload from frontend
#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub username: String,  // Username for authentication
    pub password: String,  // Plain-text password to verify
}

/// Authentication response sent to frontend
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub success: bool,         // true if operation succeeded
    pub message: String,       // Human-readable message
    pub user: Option<UserInfo>,  // User info (null on error)
}

/// User information returned in auth responses
#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: i32,         // User ID
    pub username: String,  // Username
    pub email: String,   // User email
}

/// Handle user registration
/// Route: POST /api/register
///
/// # Process
/// 1. Get database connection from pool
/// 2. Create user in database (password is hashed by repository)
/// 3. Generate JWT token with user's unique salt
/// 4. Set httpOnly cookie with token
/// 5. Return success with user info
///
/// # Errors
/// - DatabaseError: Duplicate username/email, connection failure
/// - AuthFailTokenWrongFormat: Token generation failure
pub async fn handle_register(
    State(pool): State<DbPool>,
    cookies: Cookies,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<AuthResponse>> {
    // Step 1: Get connection from pool
    let client = pool
        .get()
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))?;

    // Step 2: Create user DTO
    let user_fc = UserForCreate {
        username: payload.username,
        email: payload.email,
        password: payload.password,  // Will be hashed by repository
    };

    // Step 3: Create user in database
    let user = UserRepository::create(&client, &user_fc)
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))?;

    // Step 4: Generate JWT token
    let token = generate_token(&user.username, user.token_salt)?;

    // Step 5: Set httpOnly cookie (prevents JavaScript access for security)
    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);  // Prevent XSS attacks
    cookie.set_path("/");        // Cookie valid for all routes
    cookies.add(cookie);

    // Step 6: Return success response
    Ok(Json(AuthResponse {
        success: true,
        message: "User registered successfully".to_string(),
        user: Some(UserInfo {
            id: user.id,
            username: user.username,
            email: user.email,
        }),
    }))
}

/// Handle user login
/// Route: POST /api/login
///
/// # Process
/// 1. Find user by username
/// 2. Verify password matches stored hash
/// 3. Generate JWT token
/// 4. Set httpOnly cookie
/// 5. Return success with user info
///
/// # Errors
/// - LoginFail: User not found or wrong password
/// - DatabaseError: Connection failure
pub async fn handle_login(
    State(pool): State<DbPool>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AuthResponse>> {
    // Step 1: Get connection from pool
    let client = pool
        .get()
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))?;

    // Step 2: Find user by username
    let user = UserRepository::find_by_username(&client, &payload.username)
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))?
        .ok_or(Error::LoginFail)?;  // User not found

    // Step 3: Verify password hash
    if !verify_password(&payload.password, &user.pwd_hash) {
        return Err(Error::LoginFail);  // Wrong password
    }

    // Step 4: Generate JWT token
    let token = generate_token(&user.username, user.token_salt)?;

    // Step 5: Set httpOnly cookie
    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);  // Prevent XSS attacks
    cookie.set_path("/");        // Cookie valid for all routes
    cookies.add(cookie);

    // Step 6: Return success response
    Ok(Json(AuthResponse {
        success: true,
        message: "Login successful".to_string(),
        user: Some(UserInfo {
            id: user.id,
            username: user.username,
            email: user.email,
        }),
    }))
}

/// Handle user logout
/// Route: POST /api/logout
///
/// Simply removes the auth cookie, invalidating the session.
/// The JWT token itself remains valid until expiration, but without
/// the cookie, the client cannot send it with requests.
pub async fn handle_logout(cookies: Cookies) -> Result<Json<Value>> {
    // Remove auth cookie
    cookies.remove(Cookie::from(AUTH_TOKEN));

    Ok(Json(json!({
        "success": true,
        "message": "Logged out successfully"
    })))
}

/// Get current user info
/// Route: GET /api/me (protected endpoint)
///
/// # Protected Route
/// This handler requires authentication via mw_ctx_resolver middleware.
/// The Ctx is injected by middleware after validating the JWT token.
///
/// # Process
/// 1. Extract user_id from Ctx (set by middleware)
/// 2. Fetch user from database
/// 3. Return user info
///
/// # Note
/// There's a bug in line 143: ctx.user_id() returns i32 but we're converting
/// to string and passing to find_by_username. Should use find_by_id or fix logic.
pub async fn handle_me(
    State(pool): State<DbPool>,
    crate::middleware::CtxW(ctx): crate::middleware::CtxW,
) -> Result<Json<Value>> {
    // Get connection from pool
    let client = pool
        .get()
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))?;

    // Get user info from database using ctx.user_id()
    // BUG: This converts user_id (i32) to string, but find_by_username expects username
    // Should use find_by_id instead
    let user = UserRepository::find_by_username(&client, &ctx.user_id().to_string())
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))?;

    Ok(Json(json!({
        "user": user.map(|u| UserInfo {
            id: u.id,
            username: u.username,
            email: u.email,
        })
    })))
}
