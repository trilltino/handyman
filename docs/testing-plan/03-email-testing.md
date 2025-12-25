# Phase 3: Email Testing & Environment Debugging

Testing the lettre email service and diagnosing SMTP configuration issues.

---

## Email Service Overview

**Location:** `backend/libs/lib-core/src/email.rs`

**Dependencies:**
- `lettre` - SMTP email library
- Async with `tokio`
- Automatic retry (3 attempts, exponential backoff)

**Required Environment Variables:**

| Variable | Required | Default |
|----------|----------|---------|
| `SMTP_HOST` | No | `smtp.gmail.com` |
| `SMTP_PORT` | No | `587` |
| `SMTP_USERNAME` | **Yes** | None |
| `SMTP_PASSWORD` | **Yes** | None |
| `FROM_EMAIL` | No | `noreply@xftradesmen.com` |
| `CONTACT_NOTIFICATION_EMAIL` | No | `admin@xftradesmen.com` |

---

## Diagnosing Email Issues

### Step 1: Check Environment Variables

```bash
# In your terminal, check if SMTP vars are set:
echo %SMTP_USERNAME%
echo %SMTP_PASSWORD%

# In PowerShell:
$env:SMTP_USERNAME
$env:SMTP_PASSWORD
```

### Step 2: Verify `.env` File

Your `.env` file should have:

```
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password
FROM_EMAIL=noreply@xftradesmen.com
CONTACT_NOTIFICATION_EMAIL=isicheivalentine@gmail.com
```

> **Gmail App Password:** If using Gmail, you need an App Password, not your regular password.
> Generate at: https://myaccount.google.com/apppasswords

### Step 3: Test SMTP Connection Manually

Create a simple test script:

```rust
// tests/email_debug.rs
use std::env;

#[test]
fn test_smtp_env_vars() {
    let host = env::var("SMTP_HOST");
    let port = env::var("SMTP_PORT");
    let user = env::var("SMTP_USERNAME");
    let pass = env::var("SMTP_PASSWORD");
    
    println!("SMTP_HOST: {:?}", host);
    println!("SMTP_PORT: {:?}", port);
    println!("SMTP_USERNAME: {:?}", user.map(|s| format!("{}...", &s[..5.min(s.len())])));
    println!("SMTP_PASSWORD: {:?}", pass.map(|_| "[SET]"));
    
    assert!(user.is_ok(), "SMTP_USERNAME not set!");
    assert!(pass.is_ok(), "SMTP_PASSWORD not set!");
}
```

Run with:
```bash
cargo test test_smtp_env_vars -- --nocapture
```

---

## Mock Email Service for Testing

To test email flows without actually sending emails:

### Create Mock Service

```rust
// backend/libs/lib-core/src/email/mock.rs

use std::sync::{Arc, Mutex};

pub struct MockEmailService {
    sent_emails: Arc<Mutex<Vec<EmailMessage>>>,
}

impl MockEmailService {
    pub fn new() -> Self {
        Self {
            sent_emails: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    pub async fn send_email(&self, message: EmailMessage) -> Result<(), EmailError> {
        self.sent_emails.lock().unwrap().push(message);
        Ok(())
    }
    
    pub fn sent_count(&self) -> usize {
        self.sent_emails.lock().unwrap().len()
    }
    
    pub fn last_email(&self) -> Option<EmailMessage> {
        self.sent_emails.lock().unwrap().last().cloned()
    }
    
    pub fn clear(&self) {
        self.sent_emails.lock().unwrap().clear();
    }
}
```

### Use Trait for Abstraction

```rust
// email/traits.rs
#[async_trait]
pub trait EmailSender: Send + Sync {
    async fn send_email(&self, message: EmailMessage) -> Result<(), EmailError>;
}

#[async_trait]
impl EmailSender for EmailService {
    async fn send_email(&self, message: EmailMessage) -> Result<(), EmailError> {
        self.send_email(message).await
    }
}

#[async_trait]
impl EmailSender for MockEmailService {
    async fn send_email(&self, message: EmailMessage) -> Result<(), EmailError> {
        // Record but don't send
        self.sent_emails.lock().unwrap().push(message);
        Ok(())
    }
}
```

---

## Email Integration Tests

### Test 1: Service Creation

```rust
#[test]
fn test_email_service_requires_credentials() {
    // Clear env vars
    std::env::remove_var("SMTP_USERNAME");
    std::env::remove_var("SMTP_PASSWORD");
    
    let result = EmailService::new();
    assert!(result.is_err());
    assert!(matches!(result, Err(EmailError::ConfigError(_))));
}
```

### Test 2: Message Building

```rust
#[test]
fn test_email_message_building() {
    let message = EmailMessage {
        to: "test@example.com".to_string(),
        subject: "Test Subject".to_string(),
        body: "Test body".to_string(),
        content_type: "text/plain; charset=utf8".to_string(),
    };
    
    assert!(message.to.contains("@"));
}
```

### Test 3: Contact Notification (with Mock)

```rust
#[tokio::test]
async fn test_contact_notification_format() {
    let mock = MockEmailService::new();
    
    mock.send_contact_notification(
        "John Doe",
        "john@example.com",
        Some("Question"),
        "What are your rates?"
    ).await.unwrap();
    
    let email = mock.last_email().unwrap();
    assert!(email.subject.contains("Contact Form"));
    assert!(email.body.contains("John Doe"));
    assert!(email.body.contains("john@example.com"));
}
```

---

## Running Live Email Test

> **Warning:** This sends a real email!

```rust
#[tokio::test]
#[ignore] // Don't run in CI
async fn test_live_email_delivery() {
    dotenv::dotenv().ok();
    
    let service = email_service();
    assert!(service.is_ok(), "Email service not configured");
    
    let result = service.unwrap()
        .send_contact_notification(
            "Test User",
            "test@example.com",
            Some("Live Test"),
            "This is a test message from automated tests."
        )
        .await;
    
    assert!(result.is_ok(), "Failed to send: {:?}", result.err());
}
```

Run with:
```bash
cargo test test_live_email -- --ignored --nocapture
```

---

## Fly.io Email Configuration

On Fly.io, set secrets:

```bash
fly secrets set SMTP_USERNAME="your-email@gmail.com"
fly secrets set SMTP_PASSWORD="your-app-password"
fly secrets set CONTACT_NOTIFICATION_EMAIL="admin@yourdomain.com"
```

Verify secrets are set:
```bash
fly secrets list
```

---

## Common Email Issues

| Symptom | Cause | Fix |
|---------|-------|-----|
| "SMTP credentials not configured" | Missing env vars | Set SMTP_USERNAME and SMTP_PASSWORD |
| "SMTP relay error" | Wrong host | Check SMTP_HOST value |
| "Authentication failed" | Wrong password | Use Gmail App Password |
| Email not received | Wrong recipient | Check CONTACT_NOTIFICATION_EMAIL |
| Connection timeout | Firewall/port | Ensure port 587 is open |

---

## Implementation Checklist

- [ ] Add `test_smtp_env_vars` debug test
- [ ] Create `MockEmailService` 
- [ ] Create `EmailSender` trait
- [ ] Add unit tests for message building
- [ ] Add integration test for contact flow
- [ ] Verify Fly.io secrets are set
- [ ] Test live email delivery (manual)
