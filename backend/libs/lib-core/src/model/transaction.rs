//! # Transaction Wrapper
//!
//! Database transaction helpers for multi-step operations.
//!
//! ## Usage
//!
//! ```rust
//! use lib_core::model::transaction::with_transaction;
//!
//! let result = with_transaction(&mm, |tx_mm| async move {
//!     let booking_id = BookingBmc::create(&tx_mm, booking).await?;
//!     let quote_id = QuoteBmc::create(&tx_mm, quote).await?;
//!     Ok((booking_id, quote_id))
//! }).await?;
//! ```

use crate::model::{Error, ModelManager, Result};
use std::future::Future;

/// Execute operations within a database transaction.
///
/// If any operation fails, all changes are rolled back.
/// On success, all changes are committed.
///
/// # Arguments
///
/// * `mm` - ModelManager for database access
/// * `f` - Async closure that receives a transactional ModelManager
///
/// # Returns
///
/// The result of the closure, or an error if any operation failed.
///
/// # Example
///
/// ```rust
/// use lib_core::model::transaction::with_transaction;
/// use lib_core::model::booking::{BookingBmc, BookingForCreate};
/// use lib_core::model::quote::{QuoteBmc, QuoteForCreate};
///
/// async fn create_booking_with_quote(
///     mm: &ModelManager,
///     booking: BookingForCreate,
///     quote: QuoteForCreate,
/// ) -> Result<(i32, i32)> {
///     with_transaction(mm, |tx_mm| async move {
///         let booking_id = BookingBmc::create(&tx_mm, booking).await?;
///         let quote_id = QuoteBmc::create(&tx_mm, quote).await?;
///         Ok((booking_id, quote_id))
///     }).await
/// }
/// ```
pub async fn with_transaction<F, Fut, T>(mm: &ModelManager, f: F) -> Result<T>
where
    F: FnOnce(ModelManager) -> Fut,
    Fut: Future<Output = Result<T>>,
{
    // Create a transactional ModelManager
    let tx_mm = mm.new_with_txn()?;

    // Execute the closure
    let result = f(tx_mm.clone()).await;

    // Handle commit/rollback based on result
    match &result {
        Ok(_) => {
            // Commit transaction on success
            tx_mm.dbx().commit_txn().await.map_err(|e| {
                Error::CantCreateModelManagerProvider(format!("Transaction commit failed: {}", e))
            })?;
        }
        Err(_) => {
            // Rollback is automatic when tx_mm is dropped without commit
            // We could explicitly rollback here if needed
        }
    }

    result
}

/// Execute operations with automatic retry on transient failures.
///
/// Useful for operations that may fail due to connection issues or deadlocks.
///
/// # Arguments
///
/// * `mm` - ModelManager for database access
/// * `max_retries` - Maximum number of attempts
/// * `f` - Async closure to execute
///
/// # Returns
///
/// The result of the closure, or the last error if all retries failed.
pub async fn with_retry<F, Fut, T>(mm: &ModelManager, max_retries: u32, f: F) -> Result<T>
where
    F: Fn(&ModelManager) -> Fut + Clone,
    Fut: Future<Output = Result<T>>,
{
    let mut last_error = None;
    let mut delay = std::time::Duration::from_millis(100);

    for attempt in 1..=max_retries {
        match f(mm).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                tracing::warn!(
                    attempt = attempt,
                    max_retries = max_retries,
                    error = ?e,
                    "Database operation failed, retrying..."
                );
                last_error = Some(e);

                if attempt < max_retries {
                    tokio::time::sleep(delay).await;
                    delay = delay.saturating_mul(2); // Exponential backoff
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        Error::CantCreateModelManagerProvider("Unknown error after retries".to_string())
    }))
}

/// Retry configuration.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of attempts
    pub max_retries: u32,
    /// Initial delay between retries
    pub initial_delay_ms: u64,
    /// Maximum delay between retries
    pub max_delay_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
        }
    }
}

impl RetryConfig {
    /// Creates a config for quick operations (fewer retries, shorter delays).
    pub fn quick() -> Self {
        Self {
            max_retries: 2,
            initial_delay_ms: 50,
            max_delay_ms: 500,
        }
    }

    /// Creates a config for important operations (more retries, longer delays).
    pub fn important() -> Self {
        Self {
            max_retries: 5,
            initial_delay_ms: 200,
            max_delay_ms: 10000,
        }
    }
}
