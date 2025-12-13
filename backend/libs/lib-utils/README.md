# lib-utils

**Shared utility functions for the Handyman Marketplace.**

## Purpose

`lib-utils` provides common utilities used across the codebase:

- **Base64 Encoding/Decoding**: URL-safe base64 operations
- **Time Utilities**: Timestamp formatting, timezone handling
- **Environment Parsing**: Type-safe environment variable access
- **String Helpers**: Slug generation, sanitization
- **Validation**: Email, phone, URL validation

Zero dependencies on other internal crates - pure utilities.

---

## Modules

### `b64` - Base64 Utilities

URL-safe base64 encoding/decoding:

```rust
use lib_utils::b64;

// Encode bytes to base64 string
let data = b"Hello, World!";
let encoded = b64::encode(data);
// Result: "SGVsbG8sIFdvcmxkIQ"

// Decode base64 string to bytes
let decoded = b64::decode(&encoded)?;
assert_eq!(decoded, data);

// URL-safe encoding (for tokens, slugs)
let token_bytes = [0u8; 32];  // 32 random bytes
let token = b64::encode_url_safe(&token_bytes);
// Safe to use in URLs (no +, /, = characters)
```

**Use cases**:
- JWT token encoding
- Password reset tokens
- Session IDs
- File uploads (base64 image data)

---

### `time_utils` - Time Helpers

Consistent timestamp handling:

```rust
use lib_utils::time_utils;
use time::OffsetDateTime;

// Get current UTC timestamp
let now = time_utils::now();

// Format timestamp for display
let formatted = time_utils::format_datetime(now);
// Result: "2025-11-01 14:30:00 UTC"

// Format for database (RFC3339)
let db_format = time_utils::to_rfc3339(now);
// Result: "2025-11-01T14:30:00Z"

// Parse from string
let parsed = time_utils::parse_rfc3339("2025-11-01T14:30:00Z")?;

// Add duration
let tomorrow = time_utils::add_days(now, 1);
let next_week = time_utils::add_weeks(now, 1);

// Check if expired
let expires_at = time_utils::add_hours(now, 24);
if time_utils::is_expired(expires_at) {
    println!("Token expired!");
}
```

**Use cases**:
- Token expiration checks
- Activity timestamps
- Subscription billing periods
- Booking scheduling

---

### `envs` - Environment Variable Parsing

Type-safe environment variable access:

```rust
use lib_utils::envs;

// Get required string (panics if missing)
let db_url = envs::get_env("DATABASE_URL");

// Get optional string
let optional_key = envs::get_env_opt("OPTIONAL_KEY");

// Get with default
let port = envs::get_env_default("PORT", "3000");

// Parse as integer
let max_connections = envs::get_env_parse::<usize>("MAX_CONNECTIONS")?;

// Parse as boolean
let debug_mode = envs::get_env_bool("DEBUG")?;
// Accepts: "true", "1", "yes", "on" (case-insensitive)

// Parse as URL
let stripe_webhook_url = envs::get_env_url("STRIPE_WEBHOOK_URL")?;
```

**Benefits**:
- Type safety (compile-time checks where possible)
- Clear error messages
- Consistent parsing across codebase
- Reduces boilerplate

---

### `string_utils` - String Helpers

Common string operations:

```rust
use lib_utils::string_utils;

// Generate URL-safe slug
let slug = string_utils::slugify("John's Plumbing & Heating");
// Result: "johns-plumbing-heating"

// Sanitize user input (remove dangerous characters)
let safe_input = string_utils::sanitize("<script>alert('xss')</script>");
// Result: "scriptalert('xss')script"

// Truncate with ellipsis
let short = string_utils::truncate("Long description...", 20);
// Result: "Long description..."

// Generate random string (for tokens)
let token = string_utils::random_alphanumeric(32);
// Result: "a1b2c3d4e5f6g7h8..."

// Validate formats
assert!(string_utils::is_valid_email("user@example.com"));
assert!(string_utils::is_valid_phone("+1234567890"));
assert!(string_utils::is_valid_url("https://example.com"));
```

---

### `validation` - Input Validation

Comprehensive validation functions:

```rust
use lib_utils::validation;

// Email validation (RFC 5322 compliant)
validation::validate_email("user@example.com")?;
// Returns Err if invalid

// Phone validation (international format)
validation::validate_phone("+1234567890")?;

// URL validation
validation::validate_url("https://example.com")?;

// Password strength
validation::validate_password("MyP@ssw0rd123")?;
// Checks: length, uppercase, lowercase, numbers, special chars

// Slug validation
validation::validate_slug("johns-plumbing")?;
// Only lowercase, numbers, hyphens

// Business rules
validation::validate_booking_date(date)?;
// Ensures date is in future, not too far ahead
```

---

## Error Handling

```rust
pub enum UtilError {
    /// Base64 decode error
    Base64DecodeError(String),

    /// Time parse error
    TimeParseError(String),

    /// Environment variable missing
    EnvVarMissing(String),

    /// Environment variable parse error
    EnvVarParseError { key: String, expected_type: String },

    /// Validation error
    ValidationError(String),
}
```

---

## Usage Examples

### Generate Password Reset Token

```rust
use lib_utils::{string_utils, b64, time_utils};

// Generate secure random token
let token_bytes = string_utils::random_bytes(32);
let token = b64::encode_url_safe(&token_bytes);

// Set expiration (1 hour)
let expires_at = time_utils::add_hours(time_utils::now(), 1);

// Store in database
PasswordResetToken {
    token,
    expires_at,
    // ...
}
```

### Validate User Registration

```rust
use lib_utils::validation;

// Validate all fields
validation::validate_email(&email)?;
validation::validate_password(&password)?;
validation::validate_slug(&username)?;

// If all pass, create user
UserBmc::create(&ctx, &mm, user_data).await?;
```

### Parse Configuration

```rust
use lib_utils::envs;

pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
    pub max_connections: usize,
    pub debug_mode: bool,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, UtilError> {
        Ok(Self {
            database_url: envs::get_env("DATABASE_URL"),
            jwt_secret: envs::get_env("JWT_SECRET"),
            port: envs::get_env_parse("PORT").unwrap_or(3000),
            max_connections: envs::get_env_parse("MAX_CONNECTIONS").unwrap_or(5),
            debug_mode: envs::get_env_bool("DEBUG").unwrap_or(false),
        })
    }
}
```

---

## Testing

```bash
cargo test -p lib-utils
```

### Example Tests

```rust
#[test]
fn test_slugify() {
    assert_eq!(string_utils::slugify("John's Plumbing"), "johns-plumbing");
    assert_eq!(string_utils::slugify("ABC 123"), "abc-123");
    assert_eq!(string_utils::slugify("Test--Slug"), "test-slug");
}

#[test]
fn test_base64_roundtrip() {
    let data = b"Hello, World!";
    let encoded = b64::encode(data);
    let decoded = b64::decode(&encoded).unwrap();
    assert_eq!(decoded, data);
}

#[test]
fn test_email_validation() {
    assert!(validation::validate_email("user@example.com").is_ok());
    assert!(validation::validate_email("invalid").is_err());
    assert!(validation::validate_email("@example.com").is_err());
}
```

---

## Best Practices

### ✅ DO

- Use utilities for consistent behavior
- Validate all user input
- Parse environment variables at startup (fail fast)
- Use slugify for URL-safe strings
- Use time_utils for all timestamp operations

### ❌ DON'T

- Don't re-implement these utilities in domain libs
- Don't skip validation ("I'll validate later")
- Don't use unwrap() on environment variables in production code
- Don't mix time libraries (use time_utils consistently)
- Don't roll your own crypto (use proven libraries)

---

## Roadmap

### Planned Features
- [ ] Currency formatting (USD, EUR, etc.)
- [ ] Pagination helpers (offset, limit, page numbers)
- [ ] Rate limiting utilities
- [ ] Retry logic with exponential backoff
- [ ] Cache helpers (TTL, invalidation)
- [ ] Geolocation utilities (distance, bounding box)
- [ ] File size formatting (KB, MB, GB)
- [ ] Duration formatting (human-readable)

### Improvements
- [ ] More comprehensive email validation (DNS check)
- [ ] International phone number support (libphonenumber)
- [ ] More string sanitization options (markdown, SQL)
- [ ] Benchmark performance-critical functions
- [ ] Add property-based tests

---

## Dependencies

**Zero internal dependencies** - lib-utils is pure utilities.

**External dependencies**:
- **base64** - Base64 encoding/decoding
- **time** - Date/time handling (consistent with lib-core)
- **regex** - Pattern matching (email, phone validation)
- **url** - URL parsing and validation
- **serde** - For configuration parsing (optional)

---

## Design Principles

1. **Zero Dependencies**: No dependencies on other internal crates
2. **Stateless**: All functions are pure (no side effects)
3. **Type-Safe**: Use Rust's type system for safety
4. **Well-Tested**: High test coverage for reliability
5. **Documented**: Every public function has doc comments
6. **Performance**: Optimize hot paths (base64, validation)

---

## License

Copyright © 2025 Handyman Marketplace. All rights reserved.
