# Phase 4: Frontend Testing (Leptos)

Testing strategies for Leptos SSR components.

---

## Leptos Testing Approaches

### 1. Unit Tests for Logic

Test non-UI logic in isolation:

```rust
// frontend-leptos/src/utils.rs
pub fn format_price(pence: i32) -> String {
    format!("£{:.2}", pence as f64 / 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_price() {
        assert_eq!(format_price(1050), "£10.50");
        assert_eq!(format_price(0), "£0.00");
        assert_eq!(format_price(99), "£0.99");
    }
}
```

### 2. Component Structure Tests

Test that components compile and have correct structure:

```rust
#[cfg(test)]
mod tests {
    use leptos::*;
    
    #[test]
    fn test_button_renders() {
        // Leptos components can be tested with leptos_dom::testing
        let button = view! { <button class="btn">"Click"</button> };
        // Assert structure
    }
}
```

### 3. Server Function Tests

Test server functions independently:

```rust
#[tokio::test]
async fn test_server_function_contact() {
    // Set up mock database
    let result = submit_contact_form(
        "John".to_string(),
        "john@example.com".to_string(),
        "Test".to_string(),
    ).await;
    
    assert!(result.is_ok());
}
```

---

## Pages to Test

### Critical Pages

| Page | Path | Key Features |
|------|------|--------------|
| Home | `/` | Hero, Services grid, CTA |
| Pricing | `/pricing` | Price cards, Stripe links |
| Contact | `/contact` | Form submission, validation |
| Handyman Home | `/handyman-coventry` | Light theme, services |

### Test Checklist per Page

- [ ] Page loads without errors
- [ ] SSR renders correct HTML
- [ ] Meta tags present (SEO)
- [ ] All links work
- [ ] Forms validate correctly
- [ ] Mobile responsive

---

## SSR Rendering Tests

Test that SSR produces expected HTML:

```rust
#[tokio::test]
async fn test_home_ssr() {
    use leptos::*;
    
    let html = leptos::ssr::render_to_string(|| view! {
        <HomePage />
    });
    
    // Check key elements present
    assert!(html.contains("XF Tradesmen"));
    assert!(html.contains("<h1"));
    assert!(html.contains("class=\"hero"));
}

#[tokio::test]
async fn test_pricing_ssr() {
    let html = leptos::ssr::render_to_string(|| view! {
        <PricingPage />
    });
    
    assert!(html.contains("£329"));
    assert!(html.contains("£29"));
}
```

---

## Form Validation Tests

```rust
#[cfg(test)]
mod contact_form_tests {
    use super::*;
    
    #[test]
    fn test_email_validation() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("invalid").is_err());
        assert!(validate_email("").is_err());
    }
    
    #[test]
    fn test_name_validation() {
        assert!(validate_name("John Doe").is_ok());
        assert!(validate_name("").is_err());
        assert!(validate_name("A").is_err()); // Too short
    }
    
    #[test]
    fn test_message_validation() {
        assert!(validate_message("Hello, I need help").is_ok());
        assert!(validate_message("Hi").is_err()); // Too short
    }
}
```

---

## Running Frontend Tests

```bash
# Run all frontend tests
cargo test -p frontend-leptos

# Run with features
cargo test -p frontend-leptos --features ssr

# Run specific test
cargo test -p frontend-leptos test_format_price
```

---

## Implementation Checklist

### Week 1: Utility Functions
- [ ] Add tests to price formatting
- [ ] Add tests to date formatting
- [ ] Add tests to input validation

### Week 2: SSR Tests
- [ ] Test home page renders
- [ ] Test pricing page renders
- [ ] Test contact page renders
- [ ] Test meta tags present

### Week 3: Server Functions
- [ ] Test contact form submission
- [ ] Test quote request
- [ ] Test error handling
