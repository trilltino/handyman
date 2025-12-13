//! Time utilities for consistent time handling across the application.
//!
//! Uses the `time` crate for robust time operations.

use crate::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

/// Get current UTC time.
///
/// # Example
/// ```
/// let now = now_utc();
/// println!("Current time: {}", format_time(now));
/// ```
pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

/// Format time as RFC3339 string.
///
/// # Example
/// ```
/// let now = now_utc();
/// let formatted = format_time(now);
/// // Returns: "2025-10-22T15:30:00Z"
/// ```
pub fn format_time(time: OffsetDateTime) -> String {
    time.format(&Rfc3339)
        .unwrap_or_else(|_| "Invalid time".to_string())
}

/// Parse RFC3339 string to `OffsetDateTime`.
///
/// # Errors
/// Returns error if string is not valid RFC3339 format.
pub fn parse_utc(time_str: &str) -> Result<OffsetDateTime, Error> {
    OffsetDateTime::parse(time_str, &Rfc3339).map_err(|e| Error::ParseError(e.to_string()))
}

/// Convert Unix timestamp (seconds) to `OffsetDateTime`.
///
/// # Errors
/// Returns error if timestamp is out of range.
pub fn from_timestamp(timestamp: i64) -> Result<OffsetDateTime, Error> {
    OffsetDateTime::from_unix_timestamp(timestamp).map_err(|e| Error::RangeError(e.to_string()))
}

/// Convert `OffsetDateTime` to Unix timestamp (seconds).
pub const fn to_timestamp(time: OffsetDateTime) -> i64 {
    time.unix_timestamp()
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Time parse error: {0}")]
    ParseError(String),

    #[error("Time range error: {0}")]
    RangeError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_and_parse() {
        let now = now_utc();
        let formatted = format_time(now);
        let parsed = parse_utc(&formatted).unwrap();

        // Compare timestamps (avoids subsecond precision issues)
        assert_eq!(now.unix_timestamp(), parsed.unix_timestamp());
    }

    #[test]
    fn test_timestamp_conversion() {
        let timestamp = 1729612800i64; // 2024-10-22 16:00:00 UTC
        let time = from_timestamp(timestamp).unwrap();
        let back_to_timestamp = to_timestamp(time);
        assert_eq!(timestamp, back_to_timestamp);
    }
}
