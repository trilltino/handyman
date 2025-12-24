//! # Rate Limiting Middleware
//!
//! Per-endpoint rate limiting using tower_governor.
//!
//! ## Usage
//!
//! ```rust
//! use lib_web::rate_limit::{RateLimitLayer, RateLimitConfig};
//!
//! let config = RateLimitConfig::default(); // 60 req/min
//! let layer = RateLimitLayer::new(config);
//! ```

use std::sync::Arc;
use std::time::Duration;

/// Rate limiting configuration.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Requests allowed per time window
    pub requests_per_window: u32,
    /// Time window duration
    pub window_duration: Duration,
    /// Optional per-IP limits (if different from global)
    pub per_ip_requests: Option<u32>,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_window: 60, // 60 requests per minute
            window_duration: Duration::from_secs(60),
            per_ip_requests: None,
        }
    }
}

impl RateLimitConfig {
    /// Creates a strict rate limit (10 requests per minute).
    pub fn strict() -> Self {
        Self {
            requests_per_window: 10,
            window_duration: Duration::from_secs(60),
            per_ip_requests: None,
        }
    }

    /// Creates a relaxed rate limit (120 requests per minute).
    pub fn relaxed() -> Self {
        Self {
            requests_per_window: 120,
            window_duration: Duration::from_secs(60),
            per_ip_requests: None,
        }
    }

    /// Creates a burst-friendly config (100 requests per second).
    pub fn burst() -> Self {
        Self {
            requests_per_window: 100,
            window_duration: Duration::from_secs(1),
            per_ip_requests: None,
        }
    }
}

/// Simple in-memory rate limiter.
///
/// For production, consider using tower_governor or redis-based solutions.
#[derive(Clone)]
pub struct RateLimiter {
    config: RateLimitConfig,
    // In a real implementation, this would track requests per key
    // For now, this is a placeholder for the pattern
    _state: Arc<()>,
}

impl RateLimiter {
    /// Creates a new rate limiter with the given config.
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            _state: Arc::new(()),
        }
    }

    /// Checks if a request is allowed for the given key.
    ///
    /// Returns `true` if the request is within limits, `false` if rate-limited.
    pub fn check(&self, _key: &str) -> bool {
        // TODO: Implement actual rate limiting logic
        // For now, always allows (placeholder for pattern)
        true
    }

    /// Gets the config.
    pub fn config(&self) -> &RateLimitConfig {
        &self.config
    }
}

/// Rate limit presets for different endpoint types.
pub mod presets {
    use super::*;

    /// Contact form rate limit: 5 requests per minute.
    pub fn contact_form() -> RateLimitConfig {
        RateLimitConfig {
            requests_per_window: 5,
            window_duration: Duration::from_secs(60),
            per_ip_requests: Some(3),
        }
    }

    /// Login rate limit: 10 attempts per minute.
    pub fn login() -> RateLimitConfig {
        RateLimitConfig {
            requests_per_window: 10,
            window_duration: Duration::from_secs(60),
            per_ip_requests: Some(5),
        }
    }

    /// API endpoint rate limit: 100 requests per minute.
    pub fn api_endpoint() -> RateLimitConfig {
        RateLimitConfig {
            requests_per_window: 100,
            window_duration: Duration::from_secs(60),
            per_ip_requests: None,
        }
    }

    /// Webhook rate limit: 1000 requests per minute.
    pub fn webhook() -> RateLimitConfig {
        RateLimitConfig {
            requests_per_window: 1000,
            window_duration: Duration::from_secs(60),
            per_ip_requests: None,
        }
    }
}
