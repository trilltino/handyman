//! Core configuration management.
//!
//! Provides centralized configuration loaded from environment variables.

use lib_utils::envs::get_env;
use std::sync::OnceLock;

pub fn core_config() -> &'static CoreConfig {
    static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| CoreConfig::load_from_env())
}

#[allow(non_snake_case)]
pub struct CoreConfig {
    // -- Db
    pub DB_URL: String,
}

impl CoreConfig {
    fn load_from_env() -> CoreConfig {
        CoreConfig {
            // -- Db
            DB_URL: get_env("DATABASE_URL"),
        }
    }
}
