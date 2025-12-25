# Phase 1: Unit Testing Patterns

Best practices for unit testing in Rust, applied to each crate.

---

## Rust Unit Testing Best Practices

### 1. Test Module Structure

Every module with logic should have tests:

```rust
// src/my_module.rs

pub fn calculate_price(quantity: u32, unit_price: u32) -> u32 {
    quantity * unit_price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_price_basic() {
        assert_eq!(calculate_price(2, 100), 200);
    }

    #[test]
    fn test_calculate_price_zero() {
        assert_eq!(calculate_price(0, 100), 0);
    }
}
```

### 2. Naming Conventions

```rust
#[test]
fn test_<function_name>_<scenario>() { }

// Examples:
fn test_validate_email_valid()
fn test_validate_email_missing_at()
fn test_create_booking_success()
fn test_create_booking_past_date_fails()
```

### 3. Test Fixtures

Use `rstest` for parameterized tests:

```rust
use rstest::rstest;

#[rstest]
#[case("john@example.com", true)]
#[case("invalid-email", false)]
#[case("", false)]
fn test_email_validation(#[case] email: &str, #[case] expected: bool) {
    assert_eq!(is_valid_email(email), expected);
}
```

---

## Shared Crate Tests

### `shared/src/validation.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email_valid() {
        assert!(validate_email("test@example.com").is_ok());
    }

    #[test]
    fn test_validate_email_empty() {
        assert!(validate_email("").is_err());
    }

    #[test]
    fn test_validate_email_no_domain() {
        assert!(validate_email("test@").is_err());
    }

    #[test]
    fn test_validate_phone_uk_mobile() {
        assert!(validate_phone("+44 7833 263486").is_ok());
    }
}
```

### `shared/src/newtypes.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_from_pounds() {
        let money = Money::from_pounds(10.50);
        assert_eq!(money.pence(), 1050);
    }

    #[test]
    fn test_money_display() {
        let money = Money::from_pence(1050);
        assert_eq!(money.to_string(), "£10.50");
    }
}
```

---

## lib-core Tests

### `backend/libs/lib-core/src/cache.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_insert_and_get() {
        let cache = Cache::new(100);
        cache.insert("key", "value".to_string());
        assert_eq!(cache.get("key"), Some("value".to_string()));
    }

    #[test]
    fn test_cache_expiry() {
        let cache = Cache::with_ttl(Duration::from_millis(100));
        cache.insert("key", "value");
        std::thread::sleep(Duration::from_millis(150));
        assert!(cache.get("key").is_none());
    }
}
```

### `backend/libs/lib-core/src/model/*.rs`

Each model should have CRUD tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;

    #[tokio::test]
    async fn test_booking_create() {
        let mm = _dev_utils::init_test().await;
        let booking = BookingForCreate { /* ... */ };
        let id = BookingBmc::create(&mm, booking).await.unwrap();
        assert!(!id.is_nil());
    }

    #[tokio::test]
    async fn test_booking_get_not_found() {
        let mm = _dev_utils::init_test().await;
        let result = BookingBmc::get_by_id(&mm, Uuid::new_v4()).await;
        assert!(result.unwrap().is_none());
    }
}
```

---

## API Handler Tests

### `backend/apps/api/src/handlers/*.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    async fn app() -> Router {
        // Create test router
    }

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = app().await;
        let response = app
            .oneshot(Request::get("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_contact_invalid_email() {
        let app = app().await;
        let body = json!({
            "name": "Test",
            "email": "invalid",
            "message": "Test"
        });
        let response = app
            .oneshot(Request::post("/api/contact")
                .header("Content-Type", "application/json")
                .body(Body::from(body.to_string())).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
```

---

## Implementation Checklist

### Week 1: shared + lib-core
- [ ] Add tests to `shared/src/validation.rs`
- [ ] Add tests to `shared/src/newtypes.rs`
- [ ] Add tests to `lib-core/src/cache.rs`
- [ ] Review existing model tests

### Week 2: backend API
- [ ] Add handler tests with mocked state
- [ ] Add auth middleware tests
- [ ] Add error response tests

### Week 3: Coverage & cleanup
- [ ] Run `cargo tarpaulin` for coverage
- [ ] Achieve 70%+ coverage on core modules
- [ ] Document test patterns
