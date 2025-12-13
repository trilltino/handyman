//! # Authentication & Authorization Library
//!
//! Password hashing and JWT token handling for secure authentication.
//!
//! ## Purpose
//!
//! Provides cryptographic operations for user authentication:
//! - **[`pwd`]** - Password hashing with Argon2
//! - **[`token`]** - JWT token creation and validation
//!
//! ## Key Types
//!
//! - **[`pwd::Error`]** - Password operation errors
//! - **[`token::Claims`]** - JWT claims structure
//! - **[`token::Error`]** - Token validation errors
//!
//! ## Security Considerations
//!
//! - Passwords are hashed with Argon2, a modern and secure hashing algorithm
//! - Tokens use JWT with HMAC signing
//! - All cryptographic operations follow security best practices
//! - Sensitive data is never logged
//!
//! ## Usage
//!
//! ### Password Hashing
//!
//! ```rust,no_run
//! use lib_auth::pwd;
//!
//! let password = "my_secure_password";
//! let hash = pwd::hash_password(password)?;
//!
//! // Store hash in database, never store password
//! // Later, verify password
//! let valid = pwd::verify_password(password, &hash)?;
//! ```
//!
//! ### Token Handling
//!
//! ```rust,no_run
//! use lib_auth::token;
//!
//! let secret = b"my_secret_key";
//! let claims = token::Claims {
//!     sub: "user_123".to_string(),
//!     iat: 1234567890,
//!     exp: 1234571490,
//! };
//!
//! // Token validation handled by token module
//! ```

pub mod pwd;
pub mod token;

pub use pwd::Error as PwdError;
pub use token::{Claims, Error as TokenError};
