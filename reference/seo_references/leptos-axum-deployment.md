---
description: Preflight checklist for deploying Leptos + Axum SSR applications
---

# Leptos + Axum Deployment Preflight Checklist

This is a comprehensive checklist covering all the quirks and gotchas when deploying a Leptos SSR app with Axum backend.

---

## Phase 1: Project Structure Verification

### Cargo.toml Configuration
- [ ] `crate-type = ["cdylib", "rlib"]` in `[lib]` section (REQUIRED for WASM + SSR)
- [ ] Binary target defined with `[[bin]]` pointing to `src/main.rs`
- [ ] Feature flags configured:
  - [ ] `ssr` feature includes: `leptos/ssr`, `leptos_router/ssr`, `leptos_meta/ssr`, `dep:axum`, `dep:leptos_axum`
  - [ ] `hydrate` feature includes: `leptos/hydrate`
- [ ] `default = ["ssr"]` set (server builds by default)
- [ ] `wasm-bindgen` version PINNED (e.g., `wasm-bindgen = "=0.2.106"`)

### Leptos.toml Configuration
- [ ] `output-name` matches your CSS filename expectations
- [ ] `bin-package` and `lib-package` point to correct workspace package
- [ ] `bin-target` matches `[[bin]] name` in Cargo.toml
- [ ] `site-root = "target/site"` configured
- [ ] `site-pkg-dir = "pkg"` configured
- [ ] `assets-dir` points to public assets folder
- [ ] `bin-features = ["ssr"]` and `lib-features = ["hydrate"]` set
- [ ] `bin-default-features = false` and `lib-default-features = false` set
- [ ] Server ports configured (`site-addr`, `reload-port`)

---

## Phase 2: CSS Build Pipeline

### Tailwind CSS v4 Setup
- [ ] `package.json` exists at workspace root with:
  ```json
  {
    "scripts": {
      "build:css": "npx @tailwindcss/cli -i ./frontend-leptos/input.css -o ./frontend-leptos/public/xftradesmen.css --minify",
      "watch:css": "npx @tailwindcss/cli -i ./frontend-leptos/input.css -o ./frontend-leptos/public/xftradesmen.css --watch"
    }
  }
  ```
- [ ] `input.css` exists with `@tailwind` directives or `@import "tailwindcss"`
- [ ] `tailwind.config.js` exists with correct `content` paths:
  ```js
  content: { files: ["*.html", "./src/**/*.rs"] }
  ```
- [ ] CSS output path matches `<Stylesheet>` href in `lib.rs`
- [ ] `node_modules` installed (`npm install`)

### CSS Loading in Leptos
- [ ] `<Stylesheet id="leptos" href="/xftradesmen.css"/>` in App component
- [ ] CSS file is in `assets-dir` or will be copied to `site-root/pkg/`
- [ ] CSS is built BEFORE starting cargo-leptos

---

## Phase 3: WASM-Bindgen Version Sync

> [!CAUTION]
> This is the #1 cause of build failures!

- [ ] Check installed `wasm-bindgen-cli` version:
  ```bash
  wasm-bindgen --version
  ```
- [ ] Ensure Cargo.toml `wasm-bindgen` version matches EXACTLY:
  ```toml
  wasm-bindgen = "=0.2.106"
  ```
- [ ] If mismatch, update CLI:
  ```bash
  cargo install wasm-bindgen-cli --version 0.2.106
  ```

---

## Phase 4: Development Startup Sequence

### Correct Order (Critical!)
1. [ ] Build CSS first: `npm run build:css`
2. [ ] Start CSS watcher: `npm run watch:css` (in separate terminal)
3. [ ] Start Leptos dev server: `cargo leptos watch`

### Common start.bat Pattern
```batch
@echo off
:: 1. Build CSS
call npm run build:css
:: 2. Start CSS watch in background
start "CSS Watch" cmd /k "npm run watch:css"
:: 3. Start Leptos
start "Frontend" cmd /k "cargo leptos watch"
```

---

## Phase 5: Feature Flag Gotchas

### Server vs Client Code Separation
- [ ] Server-only code guarded with `#[cfg(feature = "ssr")]`
- [ ] Client-only code guarded with `#[cfg(feature = "hydrate")]`
- [ ] Shared types in separate `shared` crate (compiles for both targets)

### Hydration Function
- [ ] `hydrate()` function exists and is properly annotated:
  ```rust
  #[cfg(feature = "hydrate")]
  #[wasm_bindgen::prelude::wasm_bindgen]
  pub fn hydrate() {
      console_error_panic_hook::set_once();
      leptos::mount::hydrate_body(App);
  }
  ```

---

## Phase 6: Production Build

### Build Commands
```bash
# Build CSS for production
npm run build:css

# Build release binary
cargo leptos build --release
```

### Dockerfile Checklist
- [ ] Multi-stage build (chef pattern recommended)
- [ ] Install `wasm-bindgen-cli` at correct version
- [ ] Add `wasm32-unknown-unknown` target
- [ ] Run `npm run build:css` before cargo-leptos
- [ ] Copy `target/site` to final image
- [ ] Copy compiled binary to final image

---

## Phase 7: Common Errors & Fixes

| Error | Cause | Fix |
|-------|-------|-----|
| `it looks like the Rust project used to create this WASM file was linked against version X of wasm-bindgen` | Version mismatch | Pin `wasm-bindgen` version and reinstall CLI |
| `error: failed to compile frontend-leptos` | Missing target | `rustup target add wasm32-unknown-unknown` |
| CSS not loading | Wrong path or not built | Check `<Stylesheet>` href matches output path |
| `leptos_macro` errors | Feature not enabled | Ensure `ssr` or `hydrate` feature is active |
| Hydration mismatch | Server/client HTML differs | Check component consistency |

---

## Phase 8: Environment Variables

### Required for Production
- [ ] `LEPTOS_OUTPUT_NAME` - Matches Leptos.toml output-name
- [ ] `LEPTOS_SITE_ROOT` - Usually `site` or `/app/site`
- [ ] `LEPTOS_SITE_PKG_DIR` - Usually `pkg`
- [ ] `LEPTOS_SITE_ADDR` - Production bind address (e.g., `0.0.0.0:8080`)

---

## Quick Verification Commands

```bash
# Check WASM target installed
rustup target list --installed | grep wasm32

# Check wasm-bindgen CLI version
wasm-bindgen --version

# Check cargo-leptos installed
cargo leptos --version

# Verify CSS builds
npm run build:css && echo "CSS OK"

# Test full build
cargo leptos build
```
