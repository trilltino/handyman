# Issue #2: Unused Code Warnings

## Warning Summary
The build generates 82 warnings, primarily:
- Unused imports
- Dead code (never-read fields/constants)

While warnings don't block builds, they:
1. Clutter build output
2. May hide real issues
3. Indicate code quality problems

## Warnings by Category

### A. Unused Imports (5 warnings)

**File:** `frontend-leptos/src/components/mod.rs:16`
```rust
// Warning: unused imports
pub use error_boundary::{ErrorBoundary, LoadingPlaceholder, LoadingSpinner};
```

**File:** `frontend-leptos/src/pages/examples/handyman_app/pages/mod.rs:15-18`
```rust
// Warning: unused imports
pub use admin::*;
pub use booking::*;
pub use content::*;
pub use main::*;
```

**Fix:** These are re-exports for public API. Either:
1. Remove if not needed
2. Add `#[allow(unused_imports)]` if intentional API

---

### B. Component Props Never Read (12 warnings)

**File:** `frontend-leptos/src/components/error_boundary.rs`
- `ErrorBoundary::children` never read
- `ErrorFallbackSmall::message` never read
- `LoadingSpinner::size` never read
- `LoadingPlaceholder::message` never read

**File:** `frontend-leptos/src/components/seo.rs`
- `ServiceSchema::name/description/price_from` never read
- `FaqSchema::faqs` never read
- `ReviewSchema::author/rating/review_body/date` never read

**Root Cause:** These components exist but aren't used yet, OR they use the props in HTML but Rust doesn't detect that.

**Fix:** Add `#[allow(dead_code)]` above `#[component]` macro:
```rust
#[allow(dead_code)]
#[component]
pub fn ServiceSchema(...) -> impl IntoView {
```

---

### C. Unused Constants (60+ warnings)

**File:** `frontend-leptos/src/design_system.rs`

All constants in the design system are marked "never used":
- `Colors::PRIMARY`, `SECONDARY`, etc.
- `Spacing::XS`, `SM`, etc.
- `Radius::NONE`, `SM`, etc.
- `Shadows::SM`, `MD`, etc.
- `Typography::*`
- `Animation::*`
- `Components::*`

**Root Cause:** Design system is defined but not yet fully utilized.

**Fix:** Add module-level attribute:
```rust
// At top of design_system.rs
#![allow(dead_code)]
```

---

### D. Struct Fields Never Read (4 warnings)

**File:** `frontend-leptos/src/pages/examples/handyman_app/pages/admin/dashboard.rs`
- `Job::id` never read
- `DashboardStats::month_revenue` never read

**Fix:** Either use the fields or add `#[allow(dead_code)]`:
```rust
#[allow(dead_code)]
struct Job {
    id: String,
    // ...
}
```

---

## Implementation Checklist

- [ ] Fix `components/mod.rs` unused imports
- [ ] Fix `pages/mod.rs` unused imports  
- [ ] Add `#![allow(dead_code)]` to `design_system.rs`
- [ ] Add `#[allow(dead_code)]` to SEO components
- [ ] Add `#[allow(dead_code)]` to error boundary components
- [ ] Fix dashboard struct fields

## Status: ⏳ PENDING
