//! # User Repository
//!
//! ## Purpose
//! Database access layer for User entity.
//! Handles user creation, authentication queries.
//!
//! ## Operations
//! - **create**: Insert new user with hashed password and unique salt
//! - **find_by_username**: Fetch full user data (for login password verification)
//! - **find_for_auth**: Fetch minimal auth data (for JWT validation in middleware)
//!
//! ## Relation to Entire Program
//! - **Called By**: Auth handlers (register, login), auth middleware (JWT validation)
//! - **Database**: Queries `users` table
//! - **Security**: Never exposes plain-text passwords, always returns hashed pwd

use crate::models::{User, UserForAuth, UserForCreate, hash_password};  // User models and password hashing
use anyhow::{Context, Result};  // Error handling with context
use tokio_postgres::Client;     // PostgreSQL async client
use uuid::Uuid;                  // Generate unique token salt

/// User repository - handles all database operations for users table
pub struct UserRepository;

impl UserRepository {
    /// Create a new user in the database
    /// Called by: POST /api/register handler
    ///
    /// # Process
    /// 1. Hash plain-text password using HMAC-SHA256
    /// 2. Generate unique UUID salt for JWT tokens
    /// 3. Insert user into database
    /// 4. Return created user with generated ID and timestamps
    ///
    /// # Errors
    /// - Duplicate username/email (database unique constraint violation)
    /// - Database connection failure
    pub async fn create(client: &Client, user_fc: &UserForCreate) -> Result<User> {
        // Hash password before storage (never store plain-text passwords!)
        let pwd_hash = hash_password(&user_fc.password);
        // Generate unique salt for this user's JWT tokens
        let token_salt = Uuid::new_v4();

        // Insert user and return generated fields (id, timestamps)
        let row = client
            .query_one(
                "INSERT INTO users (username, email, pwd_hash, token_salt)
                 VALUES ($1, $2, $3, $4)
                 RETURNING id, username, email, pwd_hash, token_salt, created_at, updated_at",
                &[&user_fc.username, &user_fc.email, &pwd_hash, &token_salt],
            )
            .await
            .context("Failed to create user")?;

        // Map database row to User struct
        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            pwd_hash: row.get("pwd_hash"),
            token_salt: row.get("token_salt"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    /// Find user by username (for login)
    /// Called by: POST /api/login handler
    /// Returns full user data including pwd_hash for password verification
    pub async fn find_by_username(client: &Client, username: &str) -> Result<Option<User>> {
        let row = client
            .query_opt(
                "SELECT id, username, email, pwd_hash, token_salt, created_at, updated_at
                 FROM users WHERE username = $1",
                &[&username],
            )
            .await
            .context("Failed to find user by username")?;

        // Map row to User if found, None if not found
        Ok(row.map(|r| User {
            id: r.get("id"),
            username: r.get("username"),
            email: r.get("email"),
            pwd_hash: r.get("pwd_hash"),
            token_salt: r.get("token_salt"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    /// Find minimal user data for JWT validation
    /// Called by: mw_ctx_resolver middleware on every protected request
    /// Only fetches id, username, token_salt (not pwd_hash, email, timestamps)
    /// More efficient than find_by_username for authentication checks
    pub async fn find_for_auth(client: &Client, username: &str) -> Result<Option<UserForAuth>> {
        let row = client
            .query_opt(
                "SELECT id, username, token_salt FROM users WHERE username = $1",
                &[&username],
            )
            .await
            .context("Failed to find user for auth")?;

        // Map row to UserForAuth (minimal user data)
        Ok(row.map(|r| UserForAuth {
            id: r.get("id"),
            username: r.get("username"),
            token_salt: r.get("token_salt"),
        }))
    }
}
