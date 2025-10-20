//! # Database Connection Pool
//!
//! ## Purpose
//! Creates and manages a pool of PostgreSQL database connections using bb8.
//! Provides Axum extractor for automatic connection injection into handlers.
//!
//! ## How It Works
//! 1. **Pool Creation**: Creates pool of reusable PostgreSQL connections at startup
//! 2. **Connection Reuse**: Handlers borrow connections from pool, return when done
//! 3. **Axum Integration**: `DatabaseConnection` extractor automatically gets connection from pool
//! 4. **Configuration**: Max 15 concurrent connections, async with Tokio
//!
//! ## Relation to Entire Program
//! - **Created By**: main.rs calls create_pool() at server startup
//! - **Used By**: All handlers that need database access (UserRepository, BookingRepository, etc.)
//! - **Performance**: Eliminates overhead of creating new connections for each request
//! - **Connection Lifetime**: Connection borrowed → Used in handler → Auto-returned to pool

use anyhow::Result;                 // Error handling
use axum::{                         // Axum web framework
    extract::{FromRef, FromRequestParts},  // Custom extractors
    http::{request::Parts, StatusCode},    // HTTP types
};
use bb8::{Pool, PooledConnection};  // Async connection pool
use bb8_postgres::PostgresConnectionManager;  // PostgreSQL pool manager
use tokio_postgres::NoTls;          // No TLS for local development

/// Type alias for PostgreSQL connection pool
/// Pool manages up to 15 concurrent database connections
pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;

/// Create a new database connection pool
/// Called by: main.rs at server startup
///
/// # Arguments
/// * `database_url` - PostgreSQL connection string (e.g., "postgresql://user:pass@localhost/db")
///
/// # Configuration
/// - Max 15 concurrent connections
/// - Async connections using Tokio
/// - No TLS (for local development - use TLS in production!)
///
/// # Returns
/// DbPool that can be shared across all handlers
pub async fn create_pool(database_url: &str) -> Result<DbPool> {
    // Create connection manager from DATABASE_URL
    let manager = PostgresConnectionManager::new_from_stringlike(database_url, NoTls)?;

    // Build pool with max 15 connections
    let pool = Pool::builder()
        .max_size(15)  // Max concurrent connections
        .build(manager)
        .await?;

    tracing::info!("Database connection pool created");
    Ok(pool)
}

/// Custom Axum extractor for database connections
/// Automatically gets a connection from the pool for each request
///
/// # Usage in Handlers
/// ```rust
/// async fn my_handler(DatabaseConnection(conn): DatabaseConnection) -> Result<Json<User>> {
///     // Use conn to query database
///     let user = UserRepository::find_by_id(&conn, 1).await?;
///     Ok(Json(user))
/// }
/// ```
///
/// # How It Works
/// 1. Axum sees `DatabaseConnection` in handler signature
/// 2. Calls `from_request_parts()` to get connection from pool
/// 3. Passes connection to handler
/// 4. Connection auto-returned to pool when handler finishes
pub struct DatabaseConnection(pub PooledConnection<'static, PostgresConnectionManager<NoTls>>);

/// Implement FromRequestParts to make DatabaseConnection an Axum extractor
impl<S> FromRequestParts<S> for DatabaseConnection
where
    DbPool: FromRef<S>,  // Pool must be in Axum state
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Get pool from Axum state
        let pool = DbPool::from_ref(state);

        // Get owned connection from pool (doesn't need to be returned manually)
        let conn = pool
            .get_owned()
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        Ok(Self(conn))
    }
}
