//! # Application Configuration Module
//!
//! ## Purpose
//! Centralized configuration management for the handyman marketplace backend.
//! Loads environment variables and provides a global singleton config instance.
//!
//! ## How It Works
//! 1. **Singleton Pattern**: Uses `OnceLock` for thread-safe, lazy initialization
//! 2. **Environment Loading**: Reads from .env file or environment variables
//! 3. **Fallback Values**: Provides sensible defaults for development
//! 4. **Panic on Error**: Fails fast if configuration is invalid
//!
//! ## Relation to Entire Program
//! - **Used By**: Every module needs config (database URL, JWT keys, Stripe)
//! - **Security Critical**: Stores JWT secret key and Stripe API key
//! - **Environment-Specific**: Different configs for dev/staging/production
//!
//! ## Configuration Values
//! - `DATABASE_URL`: PostgreSQL connection string (default: localhost)
//! - `TOKEN_KEY`: JWT signing key (256-bit secret, MUST change in production!)
//! - `TOKEN_DURATION_SEC`: JWT token lifetime (24 hours default)
//! - `STRIPE_SECRET_KEY`: Stripe API secret for payment processing
//!
//! ## Usage Example
//! ```rust
//! let database_url = config().DATABASE_URL;
//! let jwt_key = &config().TOKEN_KEY;
//! ```

use std::sync::OnceLock;

/// Global config accessor - returns singleton Config instance
/// Called by: db::create_pool(), token::generate_token(), handlers
pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
        })
    })
}

/// Application configuration struct
/// Loaded once at startup and shared across all threads
#[allow(non_snake_case)]
pub struct Config {
    // -- Database
    /// PostgreSQL connection string
    /// Format: postgresql://user:password@host:port/database
    pub DATABASE_URL: String,

    // -- JWT Authentication
    /// Secret key for signing JWT tokens (HMAC-SHA256)
    /// IMPORTANT: Change in production! Use 256-bit random key
    pub TOKEN_KEY: Vec<u8>,

    /// JWT token expiration time in seconds
    /// Default: 86,400 seconds = 24 hours
    pub TOKEN_DURATION_SEC: f64,

    // -- Stripe Payment Processing
    /// Stripe secret API key (starts with sk_test_ or sk_live_)
    /// Used for: Creating payment intents, subscription management
    pub STRIPE_SECRET_KEY: String,
}

impl Config {
    fn load_from_env() -> anyhow::Result<Config> {
        Ok(Config {
            DATABASE_URL: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/handyman".to_string()),
            
            TOKEN_KEY: std::env::var("TOKEN_KEY")
                .unwrap_or_else(|_| "your-256-bit-secret-key-change-this-in-production!!!!".to_string())
                .into_bytes(),
            
            TOKEN_DURATION_SEC: 60.0 * 60.0 * 24.0, // 24 hours
            
            STRIPE_SECRET_KEY: std::env::var("STRIPE_SECRET_KEY")
                .unwrap_or_else(|_| "sk_test_YOUR_KEY".to_string()),
        })
    }
}
