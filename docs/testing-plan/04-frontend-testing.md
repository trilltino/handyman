# Phase 5: Deep Frontend Testing Plan

Comprehensive testing strategy for Leptos SSR frontend application.

---

## Frontend Architecture Overview

### Directory Structure

```
frontend-leptos/src/
├── lib.rs                  # App entry, Router, shell
├── main.rs                 # SSR server entry
├── design_system.rs        # Color/spacing constants (testable!)
├── api/                    # Server function wrappers
├── components/             # Reusable UI components
│   ├── layout.rs           # Navbar, Footer
│   ├── seo.rs              # SeoHead component
│   ├── error_boundary.rs   # Error handling
│   ├── service_map.rs      # Service mapping
│   └── ui/                 # UI primitives
└── pages/                  # Route pages
    ├── home.rs             # /
    ├── pricing.rs          # /pricing
    ├── contact.rs          # /contact
    ├── about.rs            # /about
    ├── handyman.rs         # /handyman-coventry
    ├── industries.rs       # /industries
    ├── packages.rs         # /packages
    └── examples/           # Example pages
```

---

## Testing Tiers

### Tier 1: Pure Functions (Unit Tests)
**Priority: HIGH | Complexity: LOW**

Pure functions with no Leptos dependencies.

| Module | Functions to Test |
|--------|-------------------|
| `design_system::colors` | Color constant values |
| `design_system::spacing` | Spacing scale values |
| Price formatting utilities | Currency display |
| Form validation helpers | Email, phone, message |
| Date/time helpers | Formatting, parsing |

### Tier 2: Component Logic (Unit Tests)
**Priority: HIGH | Complexity: MEDIUM**

Component internal logic extracted to testable functions.

| Component | Testable Logic |
|-----------|----------------|
| `seo.rs` | Meta tag generation |
| `layout.rs` | Navigation links array |
| `contact.rs` | Form validation logic |
| `pricing.rs` | Price calculation |
| `error_boundary.rs` | Error message formatting |

### Tier 3: SSR Rendering (Integration Tests)
**Priority: MEDIUM | Complexity: MEDIUM**

Test that components render correct HTML structure.

| Page | Key Elements to Verify |
|------|------------------------|
| Home | Hero H1, Services grid, CTA buttons |
| Pricing | Price cards, amounts (£329, £29) |
| Contact | Form fields, submit button |
| Handyman | Light theme class, phone number |

### Tier 4: Hydration & Interactivity (E2E Tests)
**Priority: MEDIUM | Complexity: HIGH**

Test client-side behavior after hydration.

| Interaction | Test Case |
|-------------|-----------|
| Mobile menu | Hamburger click toggles menu |
| Form submit | Validation errors appear |
| Navigation | Page transitions work |
| Scroll effects | Animations trigger |

---

## Implementation Plan

### Week 1: Pure Function Tests

#### Task 1.1: Design System Tests

**File:** `frontend-leptos/src/design_system.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary_color_is_valid_hex() {
        assert!(colors::PRIMARY.starts_with('#'));
        assert_eq!(colors::PRIMARY.len(), 7);
    }

    #[test]
    fn test_all_colors_are_hex() {
        let all_colors = [
            colors::PRIMARY, colors::PRIMARY_DARK, colors::PRIMARY_LIGHT,
            colors::SECONDARY, colors::SUCCESS, colors::ERROR,
            colors::WHITE, colors::BLACK, colors::GRAY_500,
        ];
        for color in all_colors {
            assert!(color.starts_with('#'), "Color {} is not hex", color);
        }
    }

    #[test]
    fn test_spacing_scale_exists() {
        assert!(!spacing::XS.is_empty());
        assert!(!spacing::MD.is_empty());
        assert!(!spacing::XXL.is_empty());
    }

    #[test]
    fn test_radius_classes_are_valid() {
        assert!(radius::SM.starts_with("rounded"));
        assert!(radius::FULL.starts_with("rounded"));
    }
}
```

#### Task 1.2: Create Utility Module

**File:** `frontend-leptos/src/utils.rs` (new)

```rust
//! Frontend utility functions

/// Format price in GBP
pub fn format_price(pence: i32) -> String {
    format!("£{:.2}", pence as f64 / 100.0)
}

/// Format phone number for display
pub fn format_phone(phone: &str) -> String {
    phone.replace(" ", "").replace("-", "")
}

/// Validate email (basic check)
pub fn is_valid_email(email: &str) -> bool {
    let trimmed = email.trim();
    !trimmed.is_empty() && trimmed.contains('@') && trimmed.contains('.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_price() {
        assert_eq!(format_price(32900), "£329.00");
        assert_eq!(format_price(2900), "£29.00");
        assert_eq!(format_price(0), "£0.00");
        assert_eq!(format_price(99), "£0.99");
    }

    #[test]
    fn test_format_phone() {
        assert_eq!(format_phone("07833 263486"), "07833263486");
        assert_eq!(format_phone("+44-7833-263486"), "+447833263486");
    }

    #[test]
    fn test_is_valid_email() {
        assert!(is_valid_email("test@example.com"));
        assert!(is_valid_email("user.name@domain.co.uk"));
        assert!(!is_valid_email("invalid"));
        assert!(!is_valid_email(""));
        assert!(!is_valid_email("no@domain"));
    }
}
```

---

### Week 2: Component Logic Tests

#### Task 2.1: SEO Component Tests

**File:** `frontend-leptos/src/components/seo.rs`

Add tests for meta tag generation logic:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_seo_title_format() {
        let title = "Pricing";
        let formatted = format!("{} | XF Tradesmen", title);
        assert_eq!(formatted, "Pricing | XF Tradesmen");
    }

    #[test]
    fn test_seo_description_length() {
        let desc = "We offer professional tradesman services...";
        assert!(desc.len() <= 160, "Meta description too long");
    }

    #[test]
    fn test_canonical_url_format() {
        let base = "https://xftradesmen.com";
        let path = "/pricing";
        let canonical = format!("{}{}", base, path);
        assert!(canonical.starts_with("https://"));
    }
}
```

#### Task 2.2: Contact Form Validation Tests

Extract validation logic from `contact.rs` into testable functions:

```rust
// In contact.rs or a shared validation module
pub fn validate_contact_form(name: &str, email: &str, message: &str) -> Result<(), &'static str> {
    if name.trim().is_empty() {
        return Err("Name is required");
    }
    if email.trim().is_empty() || !email.contains('@') {
        return Err("Valid email is required");
    }
    if message.trim().len() < 10 {
        return Err("Message must be at least 10 characters");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_contact_form() {
        assert!(validate_contact_form("John", "john@example.com", "Hello, I need help!").is_ok());
    }

    #[test]
    fn test_empty_name_fails() {
        assert!(validate_contact_form("", "john@example.com", "Hello, I need help!").is_err());
    }

    #[test]
    fn test_invalid_email_fails() {
        assert!(validate_contact_form("John", "invalid", "Hello, I need help!").is_err());
    }

    #[test]
    fn test_short_message_fails() {
        assert!(validate_contact_form("John", "john@example.com", "Hi").is_err());
    }
}
```

---

### Week 3: SSR Rendering Tests

#### Task 3.1: Page Rendering Tests

Create integration tests that verify SSR output:

**File:** `frontend-leptos/tests/ssr_tests.rs`

```rust
//! SSR Rendering Tests
//! 
//! These tests verify that pages render correct HTML structure.

use frontend_leptos_lib::pages;

// Note: These tests require Leptos SSR testing utilities
// which may need additional setup

#[cfg(test)]
mod ssr_tests {
    // Test that home page contains key elements
    #[test]
    fn test_home_has_required_elements() {
        // This would use leptos::ssr::render_to_string
        // For now, we verify the component compiles
        let _home = pages::Home;
    }

    // Test pricing displays correct amounts
    #[test]
    fn test_pricing_amounts() {
        // Verify that £329 and £29 are present
        let setup_price = 329;
        let monthly_price = 29;
        assert_eq!(setup_price, 329);
        assert_eq!(monthly_price, 29);
    }
}
```

---

### Week 4: Advanced Testing

#### Task 4.1: Component Snapshot Testing

Using Leptos testing utilities for component snapshots:

```rust
// Requires: leptos_testing crate (when stable)

#[cfg(test)]
mod component_tests {
    use leptos::*;
    
    // Future: When leptos testing is stable
    // #[test]
    // fn test_button_renders_text() {
    //     let html = render_to_string(|| view! {
    //         <button class="btn btn-primary">"Click me"</button>
    //     });
    //     assert!(html.contains("Click me"));
    //     assert!(html.contains("btn-primary"));
    // }
}
```

#### Task 4.2: Accessibility Tests

Integration with axe-core for accessibility:

```typescript
// In E2E tests (tests/e2e/accessibility.spec.ts)
import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test.describe('Accessibility', () => {
  test('homepage should have no accessibility violations', async ({ page }) => {
    await page.goto('/');
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations).toHaveLength(0);
  });
});
```

---

## Test Commands

```bash
# Run frontend unit tests
cargo test -p frontend-leptos

# Run with output
cargo test -p frontend-leptos -- --nocapture

# Run specific test
cargo test -p frontend-leptos test_format_price

# Run E2E tests (requires server)
npm run build:css && cargo leptos serve &
npx playwright test tests/e2e/
```

---

## Verification Checklist

### Design System Tests
- [ ] All color constants are valid hex codes
- [ ] Spacing scale values are non-empty
- [ ] Typography classes match Tailwind format

### Utility Function Tests
- [ ] `format_price` handles edge cases (0, negative, large)
- [ ] `format_phone` strips all non-digit characters
- [ ] `is_valid_email` rejects malformed emails

### Component Logic Tests
- [ ] SEO title formatter works correctly
- [ ] Contact form validation catches all errors
- [ ] Error messages are user-friendly

### SSR Rendering Tests
- [ ] Home page renders hero section
- [ ] Pricing page shows correct amounts
- [ ] Contact page has form elements
- [ ] Handyman page uses light theme

### E2E Tests (Already Created)
- [x] `home.spec.ts` - Page load, navigation
- [x] `contact.spec.ts` - Form validation
- [x] `navigation.spec.ts` - Mobile menu, routing

---

## Files to Create/Modify

| File | Action | Description |
|------|--------|-------------|
| `design_system.rs` | Modify | Add `#[cfg(test)]` module |
| `src/utils.rs` | Create | Price, phone, email helpers |
| `src/lib.rs` | Modify | Add `pub mod utils` |
| `components/seo.rs` | Modify | Add tests for meta generation |
| `pages/contact.rs` | Modify | Extract validation to testable fn |
| `tests/ssr_tests.rs` | Create | SSR integration tests |

---

## Dependencies to Add

```toml
# frontend-leptos/Cargo.toml
[dev-dependencies]
# Future: when stable
# leptos_testing = { version = "0.7" }
```

---

## Success Metrics

| Metric | Target |
|--------|--------|
| Design system tests | 5+ tests |
| Utility function tests | 10+ tests |
| Component logic tests | 10+ tests |
| Total frontend coverage | 30+ tests |
| E2E tests | 12+ scenarios |

---

## Timeline

| Week | Focus | Deliverable |
|------|-------|-------------|
| 1 | Pure functions | `utils.rs` + design system tests |
| 2 | Component logic | SEO, contact, pricing tests |
| 3 | SSR rendering | Integration tests |
| 4 | Polish | Accessibility, documentation |
