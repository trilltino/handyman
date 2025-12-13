//! # Utility Library
//!
//! Base utility functions shared across all application crates.
//!
//! ## Purpose
//!
//! Provides common functionality that doesn't depend on domain logic:
//! - Base64 encoding/decoding for tokens
//! - Time handling with UTC normalization
//! - Environment variable access patterns
//!
//! ## Philosophy
//!
//! This library contains only utilities that can be used by multiple
//! crates without creating circular dependencies. It has NO dependencies
//! on lib-core, lib-web, or lib-auth.
//!
//! ## Modules
//!
//! - **[`b64`]** - URL-safe base64 encoding/decoding (no padding)
//! - **[`envs`]** - Environment variable access helpers
//! - **[`time_utils`]** - RFC3339 time formatting and parsing
//!
//! ## Common Tasks
//!
//! ### Encode/Decode Data
//!
//! ```rust
//! use lib_utils::b64;
//!
//! let data = b"Hello, World!";
//! let encoded = b64::b64u_encode(data);
//! let decoded = b64::b64u_decode(&encoded).unwrap();
//! assert_eq!(data.as_slice(), decoded.as_slice());
//! ```
//!
//! ### Time Handling
//!
//! ```rust
//! use lib_utils::time_utils;
//!
//! let now = time_utils::now_utc();
//! let formatted = time_utils::format_time(now);
//! let parsed = time_utils::parse_utc(&formatted).unwrap();
//! ```
//!
//! ### Environment Configuration
//!
//! ```rust,no_run
//! use lib_utils::envs;
//!
//! // Required variable - panics if not set
//! let database_url = envs::get_env("DATABASE_URL");
//!
//! // Optional variable
//! let port = envs::get_env_opt("SERVER_PORT");
//!
//! // With default
//! let timeout = envs::get_env_or("TIMEOUT_SECONDS", "30");
//! ```
//!
//! ## Usage
//!
//! ```rust
//! // Import prelude for convenient access to all utilities
//! use lib_utils::prelude::*;
//!
//! let data = b64u_encode(b"test");
//! let now = now_utc();
//! let db_url = get_env("DATABASE_URL");
//! ```

// Re-export commonly used types
pub use base64::engine::general_purpose::URL_SAFE_NO_PAD;
pub use time::{Duration, OffsetDateTime};

pub mod b64;
pub mod envs;
pub mod time_utils;

// Prelude for convenient importing
/// Common imports for the library
pub mod prelude {
    pub use crate::b64::{b64u_decode, b64u_encode};
    pub use crate::envs::get_env;
    pub use crate::time_utils::{format_time, now_utc, parse_utc};
}
