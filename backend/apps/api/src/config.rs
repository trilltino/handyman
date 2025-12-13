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

use config::{Config, Environment};
use serde::Deserialize;
use std::env;

/// Application configuration.
///
/// Contains all server and database settings loaded from environment.
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    /// Server configuration (host, port)
    pub server: ServerConfig,
    /// Database configuration (connection URL)
    #[allow(dead_code)]
    pub db: DbConfig,
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
    #[allow(dead_code)]
    pub url: String,
}

impl AppConfig {
    /// Load configuration from environment variables and defaults.
    ///
    /// Looks for environment variables prefixed with `APP_` and separated
    /// by double underscores (`__`) for nested keys.
    ///
    /// # Returns
    ///
    /// - `Ok(AppConfig)` - Configuration successfully loaded
    /// - `Err(ConfigError)` - Configuration loading failed
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Use environment variables
    /// APP_SERVER__PORT=3000 cargo run
    ///
    /// # Or use defaults
    /// cargo run
    /// ```
    pub fn load() -> Result<Self, config::ConfigError> {
        let _run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            .add_source(Environment::with_prefix("APP").separator("__"))
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8080)?
            .set_default(
                "db.url",
                "postgres://postgres:Ab13cba46def79_@localhost:5432/handyman",
            )?
            .build()?;

        s.try_deserialize()
    }
}
