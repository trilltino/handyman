//! # Database Query Logging
//!
//! Development-mode database query logging for debugging and optimization.
//!
//! ## Usage
//!
//! ```rust
//! use lib_core::model::query_log::{log_query, QueryStats};
//!
//! log_query("SELECT * FROM users", Duration::from_millis(5));
//! ```

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tracing::{debug, info, warn};

/// Global query statistics.
static QUERY_COUNT: AtomicU64 = AtomicU64::new(0);
static SLOW_QUERY_COUNT: AtomicU64 = AtomicU64::new(0);
static TOTAL_QUERY_TIME_MS: AtomicU64 = AtomicU64::new(0);

/// Threshold for slow query warnings (in milliseconds).
const SLOW_QUERY_THRESHOLD_MS: u64 = 100;

/// Log a database query execution.
///
/// # Arguments
///
/// * `query` - The SQL query string (or description)
/// * `duration` - How long the query took
///
/// # Example
///
/// ```rust
/// use lib_core::model::query_log::log_query;
/// use std::time::{Duration, Instant};
///
/// let start = Instant::now();
/// // ... execute query ...
/// let duration = start.elapsed();
/// log_query("SELECT * FROM contacts", duration);
/// ```
pub fn log_query(query: &str, duration: Duration) {
    let ms = duration.as_millis() as u64;

    // Update statistics
    QUERY_COUNT.fetch_add(1, Ordering::Relaxed);
    TOTAL_QUERY_TIME_MS.fetch_add(ms, Ordering::Relaxed);

    // Truncate query for logging
    let truncated: String = query.chars().take(100).collect();
    let truncated = if query.len() > 100 {
        format!("{}...", truncated)
    } else {
        truncated
    };

    if ms >= SLOW_QUERY_THRESHOLD_MS {
        SLOW_QUERY_COUNT.fetch_add(1, Ordering::Relaxed);
        warn!(
            query = %truncated,
            duration_ms = ms,
            "Slow query detected"
        );
    } else {
        debug!(
            query = %truncated,
            duration_ms = ms,
            "Query executed"
        );
    }
}

/// Query statistics snapshot.
#[derive(Debug, Clone)]
pub struct QueryStats {
    /// Total number of queries executed
    pub query_count: u64,
    /// Number of slow queries (>100ms)
    pub slow_query_count: u64,
    /// Total query time in milliseconds
    pub total_time_ms: u64,
    /// Average query time in milliseconds
    pub avg_time_ms: f64,
}

impl QueryStats {
    /// Get current query statistics.
    pub fn current() -> Self {
        let query_count = QUERY_COUNT.load(Ordering::Relaxed);
        let slow_query_count = SLOW_QUERY_COUNT.load(Ordering::Relaxed);
        let total_time_ms = TOTAL_QUERY_TIME_MS.load(Ordering::Relaxed);

        let avg_time_ms = if query_count > 0 {
            total_time_ms as f64 / query_count as f64
        } else {
            0.0
        };

        Self {
            query_count,
            slow_query_count,
            total_time_ms,
            avg_time_ms,
        }
    }

    /// Log the current statistics.
    pub fn log_summary() {
        let stats = Self::current();
        info!(
            query_count = stats.query_count,
            slow_queries = stats.slow_query_count,
            total_time_ms = stats.total_time_ms,
            avg_time_ms = format!("{:.2}", stats.avg_time_ms),
            "Query statistics"
        );
    }

    /// Reset all statistics.
    pub fn reset() {
        QUERY_COUNT.store(0, Ordering::Relaxed);
        SLOW_QUERY_COUNT.store(0, Ordering::Relaxed);
        TOTAL_QUERY_TIME_MS.store(0, Ordering::Relaxed);
    }
}

/// Query timer helper for automatic logging.
///
/// # Example
///
/// ```rust
/// use lib_core::model::query_log::QueryTimer;
///
/// let _timer = QueryTimer::new("SELECT * FROM users");
/// // Timer automatically logs when dropped
/// ```
pub struct QueryTimer {
    query: String,
    start: std::time::Instant,
}

impl QueryTimer {
    /// Create a new query timer.
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            start: std::time::Instant::now(),
        }
    }
}

impl Drop for QueryTimer {
    fn drop(&mut self) {
        log_query(&self.query, self.start.elapsed());
    }
}
