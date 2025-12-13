//! Environment variable utilities.
//!
//! Provides fail-fast environment variable access.

use std::collections::HashMap;
use std::env;
use std::sync::RwLock;

/// Get required environment variable.
///
/// # Panics
/// Panics if the environment variable is not set.
/// This is intentional - missing required config should fail fast.
///
/// # Example
/// ```
/// let db_url = get_env("DATABASE_URL");
/// ```
pub fn get_env(name: &'static str) -> String {
    static ENV_CACHE: RwLock<Option<HashMap<String, String>>> = RwLock::new(None);

    // Try to read from cache first
    {
        let cache = ENV_CACHE
            .read()
            .expect("Failed to acquire read lock on ENV_CACHE");
        if let Some(ref map) = *cache {
            if let Some(value) = map.get(name) {
                return value.clone();
            }
        }
    }

    // Not in cache, fetch from environment
    let value = env::var(name)
        .unwrap_or_else(|_| panic!("FATAL: Missing required environment variable: {name}"));

    // Store in cache
    {
        let mut cache = ENV_CACHE
            .write()
            .expect("Failed to acquire write lock on ENV_CACHE");
        let map = cache.get_or_insert_with(HashMap::new);
        map.insert(name.to_string(), value.clone());
    }

    value
}

/// Get optional environment variable.
///
/// Returns None if not set.
pub fn get_env_opt(name: &str) -> Option<String> {
    env::var(name).ok()
}

/// Get environment variable with default value.
pub fn get_env_or(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_string())
}

/// Parse environment variable as specific type.
///
/// # Errors
/// Returns error if variable is not set or cannot be parsed.
pub fn get_env_parse<T>(name: &str) -> Result<T, Error>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let value = env::var(name).map_err(|_| Error::NotFound(name.to_string()))?;
    value
        .parse()
        .map_err(|e: T::Err| Error::ParseError(name.to_string(), e.to_string()))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Environment variable not found: {0}")]
    NotFound(String),

    #[error("Failed to parse environment variable {0}: {1}")]
    ParseError(String, String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_env_opt() {
        // This test won't panic if the var doesn't exist
        let _ = get_env_opt("NONEXISTENT_VAR_12345");
    }

    #[test]
    fn test_get_env_or() {
        let value = get_env_or("NONEXISTENT_VAR_12345", "default_value");
        assert_eq!(value, "default_value");
    }
}
