//! Database connection and pooling.
//!
//! Provides Postgres database pool configuration.

// region:    --- Modules

pub(in crate::model) mod dbx;

use crate::core_config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

// endregion: --- Modules

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> sqlx::Result<Db> {
    let max_connections = if cfg!(test) { 1 } else { 5 };

    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&core_config().DB_URL)
        .await
}
