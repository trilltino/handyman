# Phase 2: Integration Testing

Testing complete workflows and database interactions.

---

## Current Integration Tests

### `tests/integration_tests.rs`

**Already implemented:**
- `test_workflow_booking_lifecycle` - Full booking CRUD
- `test_workflow_contact_submission` - Contact form flow

**Run with:**
```bash
cargo test -p tests
# Or from root:
cargo test --test integration_tests
```

---

## Integration Test Best Practices

### 1. Test Isolation

Each test should run in isolation with its own data:

```rust
#[tokio::test]
async fn test_isolated_booking() {
    // Setup: Create fresh test database state
    let mm = _dev_utils::init_test().await;
    
    // Act: Perform operations
    let id = BookingBmc::create(&mm, booking).await?;
    
    // Assert: Verify results
    let booking = BookingBmc::get_by_id(&mm, id).await?;
    assert!(booking.is_some());
    
    // Cleanup: Remove test data
    BookingBmc::delete(&mm, id).await?;
}
```

### 2. Transaction Rollback Pattern

Wrap tests in transactions that rollback:

```rust
#[tokio::test]
async fn test_with_rollback() {
    let pool = get_test_pool().await;
    let mut tx = pool.begin().await.unwrap();
    
    // Run test operations on transaction
    // ...
    
    // Rollback - no data persists
    tx.rollback().await.unwrap();
}
```

### 3. Database Seeding

Create reusable seed data:

```rust
// tests/common/mod.rs
pub mod fixtures {
    pub fn test_customer() -> CustomerForCreate {
        CustomerForCreate {
            name: "Test Customer".to_string(),
            email: "test@example.com".to_string(),
            phone: "+44 7833 263486".to_string(),
        }
    }
    
    pub fn test_booking(customer_id: Uuid) -> BookingForCreate {
        BookingForCreate {
            customer_id,
            service_type: "test".to_string(),
            // ...
        }
    }
}
```

---

## Missing Integration Tests

### 1. Quote Workflow

```rust
#[tokio::test]
async fn test_workflow_quote_to_booking() {
    let mm = _dev_utils::init_test().await;
    
    // 1. Create quote
    let quote_id = QuoteBmc::create(&mm, quote).await?;
    
    // 2. Customer accepts quote
    QuoteBmc::update_status(&mm, quote_id, QuoteStatus::Accepted).await?;
    
    // 3. Convert quote to booking
    let booking_id = BookingBmc::create_from_quote(&mm, quote_id).await?;
    
    // 4. Verify booking linked to quote
    let booking = BookingBmc::get_by_id(&mm, booking_id).await?.unwrap();
    assert_eq!(booking.quote_id, Some(quote_id));
}
```

### 2. Payment Flow

```rust
#[tokio::test]
async fn test_workflow_payment_success() {
    let mm = _dev_utils::init_test().await;
    
    // 1. Create booking (status: pending)
    let booking_id = create_test_booking(&mm).await?;
    
    // 2. Simulate Stripe webhook (payment success)
    let event = mock_stripe_event("payment_intent.succeeded");
    handle_stripe_webhook(&mm, event).await?;
    
    // 3. Verify booking status updated
    let booking = BookingBmc::get_by_id(&mm, booking_id).await?.unwrap();
    assert_eq!(booking.payment_status, PaymentStatus::Paid);
}
```

### 3. Email Notification Flow

```rust
#[tokio::test]
async fn test_workflow_contact_with_email() {
    // This test verifies the full contact flow including email
    // Uses mock email service for testing
    
    let mm = _dev_utils::init_test().await;
    let mock_email = MockEmailService::new();
    
    // Submit contact form
    let contact_id = ContactBmc::create(&mm, contact).await?;
    
    // Trigger email notification
    send_contact_notification(&mock_email, contact_id).await?;
    
    // Verify email was "sent"
    assert_eq!(mock_email.sent_count(), 1);
    assert!(mock_email.last_subject().contains("Contact Form"));
}
```

---

## Running Integration Tests

```bash
# All integration tests
cargo test --test integration_tests

# Specific test
cargo test test_workflow_booking_lifecycle

# With logging
RUST_LOG=debug cargo test --test integration_tests -- --nocapture

# With test database
DATABASE_URL=postgres://test:test@localhost/test_db cargo test
```

---

## Implementation Checklist

- [ ] Add quote workflow test
- [ ] Add payment webhook test (with mocks)
- [ ] Add email notification test (with mocks)
- [ ] Create shared fixtures module
- [ ] Add transaction rollback helper
- [ ] Set up test database migrations
