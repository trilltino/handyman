# Deployment Fix Plan

## Overview
This document provides a comprehensive analysis of the Fly.io deployment failures and a detailed plan to fix each issue.

---

## Issue Analysis

### 1. Primary Error: Recursion Limit Overflow
**Error:**
```
error: queries overflow the depth limit!
= help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]`
```

**Root Cause:** 
Leptos applications with many nested routes and meta tags create deeply nested types that exceed Rust's default recursion limit (128). The SSR binary requires a higher limit.

**Status:** ✅ FIXED (increased to 1024 in both `lib.rs` and `main.rs`)

---

### 2. Secondary Issues: Unused Code Warnings (82 warnings)

While warnings don't block builds, they indicate code quality issues. Categories:

| Category | Count | Files |
|----------|-------|-------|
| Unused imports | 5 | `components/mod.rs`, `pages/mod.rs` |
| Dead code (fields never read) | 15+ | `seo.rs`, `error_boundary.rs`, `dashboard.rs` |
| Unused constants | 60+ | `design_system.rs` |

---

### 3. Build Speed Issues

**Current:** ~15-20 min per deployment

**Bottlenecks:**
1. Full rebuild every push (no Docker layer caching for Rust)
2. Building the same dependencies twice (WASM + SSR)
3. Large context transfer (800MB+)

---

## Fix Plan

### Phase 1: Critical Fixes (Recursion Limit) ✅ DONE
- [x] Add `#![recursion_limit = "1024"]` to `lib.rs`
- [x] Add `#![recursion_limit = "1024"]` to `main.rs`

### Phase 2: Code Cleanup (Reduce Warnings)
- [ ] Fix unused imports in `components/mod.rs`
- [ ] Fix unused imports in `pages/examples/handyman_app/pages/mod.rs`
- [ ] Add `#[allow(dead_code)]` to design system constants (intentional API)
- [ ] Add `#[allow(dead_code)]` to SEO component props (used in HTML output)
- [ ] Fix unused struct fields in `dashboard.rs`

### Phase 3: Build Optimization
- [ ] Improve `.dockerignore` to reduce context size
- [ ] Consider splitting large components to reduce type complexity
- [ ] Add build caching improvements

---

## Files to Modify

### Priority 1: Already Fixed
| File | Change | Status |
|------|--------|--------|
| `frontend-leptos/src/lib.rs` | `#![recursion_limit = "1024"]` | ✅ |
| `frontend-leptos/src/main.rs` | `#![recursion_limit = "1024"]` | ✅ |

### Priority 2: Cleanup
| File | Change | Status |
|------|--------|--------|
| `frontend-leptos/src/components/mod.rs` | Remove unused imports | ⏳ |
| `frontend-leptos/src/pages/examples/handyman_app/pages/mod.rs` | Remove unused imports | ⏳ |
| `frontend-leptos/src/design_system.rs` | Add `#![allow(dead_code)]` module attr | ⏳ |
| `frontend-leptos/src/components/seo.rs` | Add `#[allow(dead_code)]` to props | ⏳ |
| `frontend-leptos/src/components/error_boundary.rs` | Fix unused props | ⏳ |

---

## Verification Steps

1. Run local build test:
   ```powershell
   .\test_deploy.bat --full
   ```

2. Push and monitor GitHub Actions:
   ```powershell
   git push origin main
   .\fetch_logs.ps1 -List
   ```

3. Verify deployment:
   ```powershell
   curl https://xftradesmen.fly.dev/health
   ```
