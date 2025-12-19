//! # Application Configuration
//!
//! Centralized configuration management for the API server.
//!
//! ## Configuration Sources (Priority Order)
//!
//! 1. Environment variables with `APP_` prefix
//! 2. Hardcoded defaults for development
//!
//! ## Environment Variables
//!
//! Use underscore separator for nested keys (`APP_SECTION__KEY`):
//!
//! ```bash
//! APP_SERVER__HOST=0.0.0.0
//! APP_SERVER__PORT=3000
//! APP_DB__URL=postgres://...
//! ```
//!
//! ## Examples
//!
//! ```bash
//! # Development (defaults)
//! cargo run -p api
//!
//! # Production with custom configuration
//! APP_SERVER__PORT=3000 APP_DB__URL=postgres://prod_db cargo run -p api
//! ```

use config::{Config, ConfigError, Environment};
use serde::Deserialize;
use std::env;
use std::sync::OnceLock;

/// Global application configuration instance.
pub fn app_config() -> &'static AppConfig {
    static INSTANCE: OnceLock<AppConfig> = OnceLock::new();
    INSTANCE.get_or_init(|| AppConfig::load().expect("Failed to load application configuration"))
}

/// Application configuration.
///
/// Contains all server, database and service settings loaded from environment.
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    /// Server configuration (host, port)
    pub server: ServerConfig,
    /// Database configuration (connection URL)
    pub db: DbConfig,
    /// Stripe configuration
    pub stripe: StripeConfig,
}

/// Server network configuration.
#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    /// Bind address (0.0.0.0 for public, 127.0.0.1 for localhost)
    pub host: String,
    /// Port number (default: 8080)
    pub port: u16,
}

/// Database connection configuration.
#[derive(Debug, Deserialize, Clone)]
pub struct DbConfig {
    /// PostgreSQL connection string
    pub url: String,
}

/// Stripe service configuration.
#[derive(Debug, Deserialize, Clone)]
pub struct StripeConfig {
    /// Stripe public key
    pub public_key: String,
    /// Stripe secret key
    pub secret_key: String,
    /// Stripe product ID for the main package
    pub product_id: String,
    /// Stripe webhook secret
    pub webhook_secret: String,
}

impl AppConfig {
    /// Load configuration from environment variables and defaults.
    ///
    /// Looks for environment variables prefixed with `APP_` and separated
    /// by double underscores (`__`) for nested keys.
    pub fn load() -> Result<Self, ConfigError> {
        let _run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start with development defaults
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8080)?
            .set_default(
                "db.url",
                "postgres://postgres:Ab13cba46def79_@localhost:5432/handyman",
            )?
            .set_default("stripe.public_key", "")?
            .set_default("stripe.secret_key", "")?
            .set_default("stripe.product_id", "")?
            .set_default("stripe.webhook_secret", "")?
            // Add environment variables (APP_SERVER__PORT etc)
            .add_source(Environment::with_prefix("APP").separator("__"))
            // Map legacy vars to structure
            .set_override_option("db.url", env::var("DATABASE_URL").ok())?
            .set_override_option("stripe.public_key", env::var("STRIPE_PUBLIC_KEY").ok())?
            .set_override_option("stripe.secret_key", env::var("STRIPE_SECRET_KEY").ok())?
            .set_override_option("stripe.product_id", env::var("STRIPE_PRODUCT_ID").ok())?
            .set_override_option(
                "stripe.webhook_secret",
                env::var("STRIPE_WEBHOOK_SECRET").ok(),
            )?
            .build()?;

        s.try_deserialize()
    }
}
