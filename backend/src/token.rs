//! # JWT Token Module
//!
//! ## Purpose
//! Generate and validate JWT (JSON Web Token) tokens for user authentication.
//! Implements custom JWT format using HMAC-SHA256 signing.
//!
//! ## How It Works
//! 1. **Generate**: Creates JWT with username (ident), expiration time, and HMAC signature
//! 2. **Format**: `base64(ident).base64(exp).base64(signature)`
//! 3. **Signing**: Uses HMAC-SHA256 with secret key + user salt for unique tokens
//! 4. **Validate**: Verifies signature and checks expiration time
//!
//! ## Relation to Entire Program
//! - **Authentication Core**: Login creates token, middleware validates it on every protected request
//! - **Flow**: Login → generate_token() → Set cookie → Client stores → Client sends → validate_token()
//! - **Security**: Tokens expire in 24 hours, unique per user (salt-based), cannot be forged
//! - **Used By**: handlers/auth.rs (login), middleware/mw_auth.rs (validation)

use crate::config::config;         // App configuration (TOKEN_KEY, TOKEN_DURATION_SEC)
use crate::error::{Error, Result};  // Custom error types
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine}; // Base64 encoding (URL-safe, no padding)
use hmac::{Hmac, Mac};              // HMAC for token signing
use sha2::Sha256;                   // SHA256 hash algorithm
use std::fmt::Display;              // Display trait for Token (converts to string)
use std::str::FromStr;              // Parse Token from string
use uuid::Uuid;                     // User salt (unique per user)

/// HMAC-SHA256 algorithm for token signing
type HmacSha256 = Hmac<Sha256>;

/// JWT Token structure
/// Format: `base64(ident).base64(exp).base64(signature)`
/// Example: "dXNlcjEyMw.MjAyNC0wMS0xNVQxMjowMDowMFo.YWJjZGVmZ2hpamtsbW5vcA"
#[derive(Debug, Clone)]
pub struct Token {
    /// User identifier (username)
    pub ident: String,
    /// Expiration time (RFC3339 format: "2024-01-15T12:00:00Z")
    pub exp: String,
    /// HMAC-SHA256 signature (base64-encoded)
    /// Signature of: "{ident}.{exp}.{user_salt}" using TOKEN_KEY
    pub sign_b64u: String,
}

/// Parse Token from string (e.g., from cookie)
/// Expected format: "base64.base64.base64"
/// Called by: mw_ctx_resolver middleware when extracting token from cookie
impl FromStr for Token {
    type Err = Error;

    fn from_str(token_str: &str) -> Result<Self> {
        // Split token string by '.' into 3 parts
        let splits: Vec<&str> = token_str.split('.').collect();
        if splits.len() != 3 {
            return Err(Error::AuthFailTokenWrongFormat);
        }

        let (ident_b64u, exp_b64u, sign_b64u) = (splits[0], splits[1], splits[2]);

        // Decode base64-encoded ident (username)
        let ident = String::from_utf8(
            URL_SAFE_NO_PAD.decode(ident_b64u).map_err(|_| Error::AuthFailTokenWrongFormat)?
        ).map_err(|_| Error::AuthFailTokenWrongFormat)?;

        // Decode base64-encoded expiration time
        let exp = String::from_utf8(
            URL_SAFE_NO_PAD.decode(exp_b64u).map_err(|_| Error::AuthFailTokenWrongFormat)?
        ).map_err(|_| Error::AuthFailTokenWrongFormat)?;

        Ok(Self {
            ident,
            exp,
            sign_b64u: sign_b64u.to_string(), // Keep signature as base64 string
        })
    }
}

/// Convert Token back to string format: "base64.base64.base64"
/// Called when: Setting auth cookie with token
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            URL_SAFE_NO_PAD.encode(&self.ident),  // Encode username to base64
            URL_SAFE_NO_PAD.encode(&self.exp),    // Encode expiration to base64
            self.sign_b64u                         // Signature already base64
        )
    }
}

/// Generate a new JWT token for a user
/// Called by: handle_login after successful password verification
///
/// # Arguments
/// * `username` - User's username (becomes token ident)
/// * `salt` - User's unique salt from database (makes token unique per user)
///
/// # Returns
/// Token with username, expiration (now + 24h), and HMAC signature
pub fn generate_token(username: &str, salt: Uuid) -> Result<Token> {
    let config = config();
    let duration_sec = config.TOKEN_DURATION_SEC; // Default: 86,400 seconds (24 hours)

    let ident = username.to_string();
    let exp = format_exp(duration_sec); // RFC3339 format: "2024-01-15T12:00:00Z"

    // Sign the token with HMAC-SHA256
    let sign_b64u = sign_token(&ident, &exp, salt, &config.TOKEN_KEY)?;

    Ok(Token {
        ident,
        exp,
        sign_b64u,
    })
}

/// Validate a JWT token
/// Called by: mw_ctx_resolver middleware on every protected request
///
/// # Validation Steps
/// 1. Re-sign the token with same ident, exp, salt, and key
/// 2. Compare signatures (prevents forgery)
/// 3. Check expiration time (ensures token not expired)
///
/// # Arguments
/// * `token` - Token parsed from cookie
/// * `salt` - User's salt from database
///
/// # Returns
/// Ok(()) if valid, Err(AuthFailTokenWrongFormat) if invalid or expired
pub fn validate_token(token: &Token, salt: Uuid) -> Result<()> {
    let config = config();

    // Re-sign the token to verify signature
    let new_sign = sign_token(&token.ident, &token.exp, salt, &config.TOKEN_KEY)?;

    // Compare signatures - if different, token was tampered with or forged
    if new_sign != token.sign_b64u {
        return Err(Error::AuthFailTokenWrongFormat);
    }

    // Parse expiration time from RFC3339 string
    let exp_time = chrono::DateTime::parse_from_rfc3339(&token.exp)
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    // Check if token has expired
    if exp_time < chrono::Utc::now() {
        return Err(Error::AuthFailTokenWrongFormat);
    }

    Ok(())
}

/// Sign token content with HMAC-SHA256
/// Creates signature of: "{ident}.{exp}.{salt}" using secret key
///
/// # Security
/// - User salt makes each user's token unique (even with same ident/exp)
/// - Secret key prevents forgery (only server can create valid signatures)
fn sign_token(ident: &str, exp: &str, salt: Uuid, key: &[u8]) -> Result<String> {
    // Content to sign: "username.2024-01-15T12:00:00Z.550e8400-e29b-41d4-a716-446655440000"
    let content = format!("{}.{}.{}", ident, exp, salt);

    // Create HMAC with secret key
    let mut mac = HmacSha256::new_from_slice(key)
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;
    mac.update(content.as_bytes());

    // Finalize HMAC and encode to base64
    let result = mac.finalize();
    let code_bytes = result.into_bytes();

    Ok(URL_SAFE_NO_PAD.encode(code_bytes))
}

/// Format expiration time as RFC3339 string
/// Adds duration_sec to current time
/// Example output: "2024-01-15T12:00:00Z"
fn format_exp(duration_sec: f64) -> String {
    let exp_time = chrono::Utc::now() + chrono::Duration::seconds(duration_sec as i64);
    exp_time.to_rfc3339()
}
