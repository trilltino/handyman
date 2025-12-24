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

## Phase 9: Leptos SEO Best Practices

> [!IMPORTANT]
> SSR is crucial for SEO. Leptos renders pages to HTML on the server, allowing search engines to index content properly.

### Meta Context Setup
- [ ] `provide_meta_context()` called in App component:
  ```rust
  #[component]
  pub fn App() -> impl IntoView {
      provide_meta_context();
      // ...
  }
  ```

### Essential Meta Components

| Component | Purpose | Example |
|-----------|---------|---------|
| `<Title/>` | Page title | `<Title text="Home - My Site"/>` |
| `<Meta/>` | Meta tags | `<Meta name="description" content="..."/>` |
| `<Link/>` | Canonical URLs, icons | `<Link rel="canonical" href="..."/>` |
| `<Html/>` | Lang, dir attributes | `<Html attr:lang="en-gb"/>` |
| `<Stylesheet/>` | CSS links | `<Stylesheet href="/styles.css"/>` |

### Page-Level SEO Checklist
- [ ] Every page has unique `<Title>` component
- [ ] Use formatter for consistent title pattern:
  ```rust
  <Title formatter=|text| format!("{text} | XFTradesmen")/>
  ```
- [ ] Meta description on every page (150-160 chars):
  ```rust
  <Meta name="description" content="Professional handyman services..."/>
  ```
- [ ] Canonical URL to prevent duplicate content:
  ```rust
  <Link rel="canonical" href="https://yoursite.com/page"/>
  ```

### Social Media / Open Graph
- [ ] OG tags for rich link previews:
  ```rust
  <Meta property="og:title" content="Page Title"/>
  <Meta property="og:description" content="Description"/>
  <Meta property="og:image" content="https://site.com/og-image.png"/>
  <Meta property="og:url" content="https://site.com/page"/>
  <Meta property="og:type" content="website"/>
  ```
- [ ] Twitter Card tags:
  ```rust
  <Meta name="twitter:card" content="summary_large_image"/>
  <Meta name="twitter:title" content="Page Title"/>
  <Meta name="twitter:description" content="Description"/>
  <Meta name="twitter:image" content="https://site.com/twitter-image.png"/>
  ```

### Structured Data (JSON-LD)
- [ ] LocalBusiness schema for service businesses:
  ```rust
  <Script type_="application/ld+json">
      r#"{
          "@context": "https://schema.org",
          "@type": "LocalBusiness",
          "name": "XFTradesmen",
          "telephone": "+44..."
      }"#
  </Script>
  ```

### Technical SEO Files
- [ ] `robots.txt` in public/assets folder:
  ```
  User-agent: *
  Allow: /
  Sitemap: https://yoursite.com/sitemap.xml
  ```
- [ ] `sitemap.xml` with all pages
- [ ] HTML lang attribute set:
  ```rust
  <Html attr:lang="en-gb"/>
  ```

### Shared PageMetadata Pattern (Recommended)
Create a reusable struct in your `shared` crate:
```rust
pub struct PageMetadata {
    pub title: String,
    pub description: String,
    pub canonical: Option<String>,
    pub og_image: Option<String>,
}
```
Then create a `<SeoHead>` component that renders all meta tags consistently.

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

---

## SEO Testing Tools

- [Google Rich Results Test](https://search.google.com/test/rich-results)
- [Facebook Sharing Debugger](https://developers.facebook.com/tools/debug/)
- [Twitter Card Validator](https://cards-dev.twitter.com/validator)
- Lighthouse in Chrome DevTools (SEO audit)

---

## Phase 10: Mobile Responsiveness

> [!IMPORTANT]
> Your SSR site works on mobile browsers without any app. Just ensure responsive CSS.

### Viewport Meta Tag
- [ ] Viewport meta in App component:
  ```rust
  <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
  ```

### Tailwind Breakpoint Strategy
```css
/* Mobile-first approach */
.container { @apply px-4; }           /* Mobile default */
.container { @apply sm:px-6; }        /* ≥640px */
.container { @apply md:px-8; }        /* ≥768px */
.container { @apply lg:px-12; }       /* ≥1024px */
```

### Touch-Friendly Checklist
- [ ] Minimum tap target size: 48x48px
- [ ] No hover-only interactions (touch devices can't hover)
- [ ] Adequate spacing between clickable elements
- [ ] Form inputs sized for thumb typing

### iOS Safe Area
```css
/* Handle iPhone notch and home indicator */
body {
  padding-top: env(safe-area-inset-top);
  padding-bottom: env(safe-area-inset-bottom);
  padding-left: env(safe-area-inset-left);
  padding-right: env(safe-area-inset-right);
}
```

### Testing
- [ ] Chrome DevTools → Toggle device toolbar (Ctrl+Shift+M)
- [ ] Test on real devices when possible
- [ ] Lighthouse mobile score ≥90

---

## Phase 11: Project Structure Best Practices

### Recommended Workspace Layout
```
xfhandyman/
├── backend/
│   ├── apps/api/              # Main API server
│   │   └── src/
│   │       ├── main.rs        # Entry point
│   │       ├── config.rs      # Environment config
│   │       └── web/           # Routes and handlers
│   └── libs/                  # Shared backend libraries
│       ├── lib-core/          # Models, database, business logic
│       ├── lib-web/           # Web utilities, extractors
│       └── lib-utils/         # Common utilities
├── frontend-leptos/
│   └── src/
│       ├── lib.rs             # App component, routes
│       ├── main.rs            # SSR server entry
│       ├── pages/             # Route page components
│       ├── components/        # Reusable UI components
│       └── api/               # Backend API clients
├── shared/                    # Types shared between FE & BE
│   └── src/
│       ├── lib.rs
│       ├── types/             # Request/Response DTOs
│       └── validation.rs      # Validation rules
├── migrations/                # SQL migrations
├── Leptos.toml               # Leptos config
├── Cargo.toml                # Workspace root
└── package.json              # CSS build scripts
```

### Workspace Cargo.toml Pattern
```toml
[workspace]
members = ["frontend-leptos", "backend/apps/api", "backend/libs/*", "shared"]
resolver = "2"

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
# ... shared dependency versions
```

---

## Phase 12: Backend Patterns (Axum)

### Route Organization
```rust
// web/mod.rs
pub fn routes(mm: ModelManager) -> Router {
    let api_routes = Router::new()
        .merge(routes_contact::routes(mm.clone()))
        .merge(routes_quote::routes(mm.clone()));

    Router::new()
        .nest("/api", api_routes)
        .merge(routes_health::routes(mm))
}
```

### Axum 0.8 Path Parameters
```rust
// OLD (0.7): .route("/quotes/:id", ...)
// NEW (0.8): .route("/quotes/{id}", ...)
.route("/quotes/{id}", get(get_quote))
```

### Handler Pattern
```rust
pub async fn create_quote(
    State(mm): State<ModelManager>,
    Json(payload): Json<CreateQuoteRequest>,
) -> Result<Json<Quote>, AppError> {
    let quote = QuoteBmc::create(&mm, payload).await?;
    Ok(Json(quote))
}
```

### Error Handling
```rust
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}
```

---

## Phase 13: Shared Crate Usage

### Purpose
Types that need to be identical on frontend and backend:
- API request/response DTOs
- Form validation
- Error types

### Example: ContactForm
```rust
// shared/src/types/contact.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub message: String,
}
```

### Usage in Backend
```rust
use shared::ContactForm;

pub async fn handle_contact(Json(form): Json<ContactForm>) -> impl IntoResponse {
    // form.name, form.email, form.message available
}
```

### Usage in Frontend
```rust
use shared::ContactForm;

let form = ContactForm {
    name: name.get(),
    email: email.get(),
    message: message.get(),
};
// Send to backend
```

---

## Phase 14: Email with Lettre

### Dependencies (backend)
```toml
lettre = { version = "0.11", features = ["smtp-transport"] }
```

### SMTP Configuration
```rust
pub fn send_email(to: &str, subject: &str, body: &str) -> Result<()> {
    let email = Message::builder()
        .from("noreply@xftradesmen.com".parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    let creds = Credentials::new(
        std::env::var("SMTP_USERNAME")?,
        std::env::var("SMTP_PASSWORD")?,
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    mailer.send(&email)?;
    Ok(())
}
```

### Environment Variables
```bash
SMTP_USERNAME=your@email.com
SMTP_PASSWORD=app-specific-password
```

---

## Phase 15: Database Patterns (SQLx)

### ModelManager Pattern
```rust
pub struct ModelManager {
    db: sqlx::PgPool,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db = PgPoolOptions::new()
            .max_connections(5)
            .connect(&std::env::var("DATABASE_URL")?)
            .await?;
        Ok(Self { db })
    }

    pub fn db(&self) -> &PgPool { &self.db }
}
```

### BMC (Backend Model Controller) Pattern
```rust
pub struct QuoteBmc;

impl QuoteBmc {
    pub async fn create(mm: &ModelManager, data: CreateQuote) -> Result<Quote> {
        sqlx::query_as!(
            Quote,
            "INSERT INTO quotes (customer_id, amount) VALUES ($1, $2) RETURNING *",
            data.customer_id,
            data.amount
        )
        .fetch_one(mm.db())
        .await
        .map_err(Into::into)
    }
}
```

### Migration Commands
```bash
# Create migration
sqlx migrate add create_quotes_table

# Run migrations
sqlx migrate run

# Or via cargo
cargo run -p api -- --migrate
```

## Phase 16: Fly.io Deployment (Comprehensive)

> [!IMPORTANT]
> Your app must listen on `0.0.0.0` (not `127.0.0.1`) to be accessible from outside the container.

### fly.toml Configuration
```toml
app = "xftradesmen"
primary_region = "lhr"  # London, UK

[build]
dockerfile = "Dockerfile.production"

[env]
LEPTOS_SITE_ROOT = "site"
LEPTOS_SITE_ADDR = "0.0.0.0:3000"
RUST_LOG = "info"

[http_service]
  internal_port = 3000
  force_https = true
  auto_stop_machines = false
  auto_start_machines = true
  min_machines_running = 1

[[http_service.checks]]
  path = "/health"
  interval = "30s"
  timeout = "5s"

[[vm]]
  memory = "1gb"
  cpu_kind = "shared"
  cpus = 1
```

### Cargo Chef Dockerfile Pattern
Fly.io recommends cargo-chef for fast rebuilds:
```dockerfile
# Stage 1: Chef (Planner)
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Builder
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - THIS IS THE CACHED LAYER!
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin app

# Stage 3: Runtime (minimal)
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates openssl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/app /usr/local/bin
ENTRYPOINT ["/usr/local/bin/app"]
```

### Environment Secrets (Never commit these!)
```bash
fly secrets set DATABASE_URL="postgres://user:pass@host:5432/db"
fly secrets set JWT_SECRET="your-super-secret-key"
fly secrets set SMTP_USERNAME="your@email.com"
fly secrets set SMTP_PASSWORD="app-specific-password"
fly secrets set STRIPE_SECRET_KEY="sk_live_..."
```

### Deployment Commands
```bash
# Install flyctl (Windows)
powershell -Command "iwr https://fly.io/install.ps1 -useb | iex"

# First time setup
fly launch

# Deploy
fly deploy

# View logs
fly logs

# SSH into container
fly ssh console

# Check status
fly status
```

### Common Gotchas & Fixes

| Issue | Cause | Fix |
|-------|-------|-----|
| App not accessible | Listening on 127.0.0.1 | Change to `0.0.0.0` |
| HTTPS errors | Missing ca-certificates | `apt-get install ca-certificates` in Dockerfile |
| TLS handshake failures | Missing openssl | `apt-get install openssl` in Dockerfile |
| Large binary size | Debug symbols included | Use `strip` or `opt-level = "z"` |
| Slow cold starts | Machine sleeping | Set `min_machines_running = 1` |
| DB connection failures | Fly Postgres waking | Add retry logic to connection |

### Axum-Specific Requirements
```rust
// Correct: Listen on all interfaces
let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

// Wrong: Only localhost (won't work on Fly.io)
let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
```

### Graceful Shutdown
```rust
use tokio::signal;

async fn shutdown_signal() {
    signal::ctrl_c().await.expect("Failed to install CTRL+C handler");
}

axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await?;
```

---

## Phase 17: Fly.io with Postgres

### Attach Fly Postgres
```bash
# Create Postgres cluster
fly postgres create

# Attach to app (sets DATABASE_URL automatically)
fly postgres attach --app xftradesmen <postgres-app-name>
```

### Connection String
Fly.io sets `DATABASE_URL` automatically. In your app:
```rust
let database_url = std::env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");
```

---

## Full Verification Checklist

```bash
# 1. Check Rust toolchain
rustup show
rustup target add wasm32-unknown-unknown

# 2. Check CLI tools
wasm-bindgen --version
cargo leptos --version
flyctl version

# 3. Build CSS
npm run build:css

# 4. Build Leptos (full test)
cargo leptos build

# 5. Run locally
cargo run -p api &
cargo leptos watch

# 6. Test endpoints
curl http://127.0.0.1:8080/api/health
curl http://127.0.0.1:3001/

# 7. Deploy to Fly.io
fly deploy

# 8. Verify deployment
fly status
fly logs
curl https://xftradesmen.fly.dev/health
```

---

## Phase 18: FOOLPROOF Fly.io Deployment (Lessons Learned)

> [!CAUTION]
> This section documents every issue encountered and fixed during deployment. Follow this exactly.

### Critical Issues We Fixed (December 2024)

| Issue | Error Message | Root Cause | Fix |
|-------|---------------|------------|-----|
| Recursion overflow | `queries overflow the depth limit!` | Complex Leptos view hierarchies | Add `#![recursion_limit = "1024"]` to BOTH `lib.rs` AND `main.rs` |
| App not listening | `not listening on expected address` | Startup script failing silently | Simplify `start_fly.sh`, remove broken commands |
| 82 compiler warnings | `field X is never read` | Unused Leptos component props | Add `#![allow(dead_code)]` to component modules |
| CSS not applying | White text on white background | Global dark theme override | Add `.handyman-theme` CSS scope class |

---

### Step-by-Step Foolproof Deployment

#### Step 1: Recursion Limit (CRITICAL)

Add this to the TOP of **both** files:

**`frontend-leptos/src/lib.rs`:**
```rust
//! Frontend library
#![recursion_limit = "1024"]  // <-- ADD THIS
use leptos::prelude::*;
```

**`frontend-leptos/src/main.rs`:**
```rust
//! Frontend entry point
#![recursion_limit = "1024"]  // <-- ADD THIS
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
```

> [!WARNING]
> You MUST add this to BOTH files. The lib.rs handles WASM, main.rs handles SSR.

---

#### Step 2: Startup Script

Use this simple, bulletproof `start_fly.sh`:

```bash
#!/bin/bash
set -e

# Environment
export LEPTOS_SITE_ADDR="${LEPTOS_SITE_ADDR:-0.0.0.0:3000}"
export LEPTOS_SITE_ROOT="${LEPTOS_SITE_ROOT:-site}"
export API_PORT="${API_PORT:-3001}"
export API_URL="http://127.0.0.1:${API_PORT}"

echo "=== Starting XFTradesmen ==="
echo "LEPTOS_SITE_ADDR: $LEPTOS_SITE_ADDR"

# Start backend (if exists)
if [ -f "/app/api" ]; then
    PORT=$API_PORT /app/api &
    sleep 2
fi

# Start frontend (foreground, with exec for signal handling)
exec /app/frontend-leptos
```

> [!IMPORTANT]
> Use `exec` for the main process - this ensures proper signal handling for graceful shutdown.

---

#### Step 3: Suppress Warning Clutter

Add these to prevent false-positive warnings from blocking your view of real errors:

**Component modules with Leptos props:**
```rust
// At top of file (after doc comments)
#![allow(dead_code)]
```

Apply to:
- `design_system.rs` (intentional API exports)
- `error_boundary.rs` (Leptos props used by macro)
- `seo.rs` (Leptos props used by macro)

**Re-export modules:**
```rust
#[allow(unused_imports)]
pub use error_boundary::*;
```

---

#### Step 4: Dockerfile.production Checklist

```dockerfile
# Stage 1: Chef - prepare dependencies
FROM rust:1.83-slim-bookworm AS chef
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl && rm -rf /var/lib/apt/lists/*
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
    && cargo binstall cargo-chef -y --force

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Builder
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Install Node.js for CSS build
RUN apt-get update && apt-get install -y curl git build-essential \
    && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Install WASM target and cargo-leptos
RUN rustup toolchain install nightly \
    && rustup default nightly \
    && rustup target add wasm32-unknown-unknown
RUN cargo binstall cargo-leptos -y --force

COPY . .

# Build CSS first, then Leptos
RUN npm ci
RUN npm run build:css
RUN cargo build --release --bin api
RUN cargo leptos build --release

# Stage 3: Runtime (minimal)
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates openssl postgresql-client && rm -rf /var/lib/apt/lists/*
RUN useradd -r -s /bin/false appuser
RUN mkdir -p /app/site && chown -R appuser:appuser /app

COPY --from=builder /app/target/release/api /app/api
COPY --from=builder /app/target/release/frontend-leptos /app/frontend-leptos
COPY --from=builder /app/target/site /app/site
COPY start_fly.sh /app/start_fly.sh
RUN chmod +x /app/start_fly.sh && chown -R appuser:appuser /app

EXPOSE 3000
USER appuser
CMD ["/app/start_fly.sh"]
```

---

#### Step 5: fly.toml Checklist

```toml
app = "xftradesmen"
primary_region = "lhr"
kill_signal = "SIGINT"
kill_timeout = "5s"

[build]
dockerfile = "Dockerfile.production"

[env]
LEPTOS_SITE_ROOT = "site"
LEPTOS_SITE_ADDR = "0.0.0.0:3000"
RUST_LOG = "info"

[http_service]
internal_port = 3000
force_https = true
auto_stop_machines = false
auto_start_machines = true
min_machines_running = 1

[[http_service.checks]]
path = "/health"
grace_period = "15s"
interval = "30s"
timeout = "5s"

[[vm]]
memory = "1gb"
cpu_kind = "shared"
cpus = 1
```

---

#### Step 6: GitHub Actions Workflow

`.github/workflows/fly-deploy.yml`:
```yaml
name: Deploy to Fly.io

on:
  push:
    branches: [main]
  workflow_dispatch:

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Fly.io CLI
        uses: superfly/flyctl-actions/setup-flyctl@master
      
      - name: Deploy to Fly.io
        run: |
          flyctl deploy --remote-only --ha=false 2>&1 | tee deploy.log
          exit ${PIPESTATUS[0]}
        env:
          FLY_API_TOKEN: ${{ secrets.FLY_API_TOKEN }}
      
      - name: Save logs on failure
        if: failure()
        run: |
          grep -i "error\|failed\|fatal\|panic" deploy.log > errors.txt || true
      
      - name: Upload logs
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: deploy-logs
          path: |
            deploy.log
            errors.txt
```

---

#### Step 7: Pre-Deployment Verification Script

Create `test_deploy.bat`:
```batch
@echo off
echo === PRE-DEPLOYMENT CHECK ===

echo [1/5] Rust toolchain...
rustc --version || (echo FAIL: Rust not found && exit /b 1)
rustup target list --installed | findstr wasm32 >nul || (echo FAIL: WASM target missing && exit /b 1)
echo OK

echo [2/5] cargo-leptos...
cargo leptos --version >nul 2>&1 || (echo FAIL: cargo-leptos not installed && exit /b 1)
echo OK

echo [3/5] Node.js...
node --version || (echo FAIL: Node.js not found && exit /b 1)
echo OK

echo [4/5] Building CSS...
call npm run build:css || (echo FAIL: CSS build failed && exit /b 1)
echo OK

echo [5/5] Checking Leptos build (SSR)...
cargo check -p frontend-leptos --features ssr || (echo FAIL: SSR check failed && exit /b 1)
echo OK

echo.
echo === ALL CHECKS PASSED ===
echo Ready to deploy!
```

---

### Debugging Failed Deployments

#### 1. Check GitHub Actions Logs
```powershell
# List recent runs
gh run list --repo <owner>/<repo> --limit 5

# Download logs from failed run
gh run download <run-id> --name deploy-logs
```

#### 2. Check Fly.io Logs
```bash
# Live logs
fly logs

# SSH into running container
fly ssh console

# Check what's listening
fly ssh console -C "netstat -tlnp"
```

#### 3. Common Error Patterns

| Log Pattern | Meaning | Fix |
|-------------|---------|-----|
| `queries overflow the depth limit` | Recursion limit too low | Increase `#![recursion_limit]` |
| `not listening on expected address` | App crashed on startup | Check `start_fly.sh`, remove failing commands |
| `wasm-bindgen version mismatch` | CLI != Cargo.toml version | Pin versions: `wasm-bindgen = "=0.2.106"` |
| `health check failed` | `/health` endpoint not responding | Verify health route exists, app is listening |

---

### Quick Deploy Commands

```bash
# Full clean deploy
git add -A && git commit -m "Deploy" && git push origin main

# Monitor deployment
gh run list --repo trilltino/handyman --limit 3

# Check site health
curl https://xftradesmen.fly.dev/health

# View Fly.io status
fly status

# Restart machines
fly machines restart
```

---

### Deployment Timeline

Typical build times:
- Docker context transfer: ~2 min
- Cargo chef (cached deps): ~1 min (first build ~8 min)
- CSS build: ~30 sec
- API binary: ~2-5 min (first build ~12 min)
- Leptos WASM + SSR: ~3-5 min
- **Total: 5-10 min (cached) / 15-25 min (fresh)**

---

## Phase 19: Advanced SEO Implementation (Google Guidelines)

> [!IMPORTANT]
> This section implements Google's official SEO Starter Guide recommendations for Leptos SSR applications. SSR is crucial because search engines can crawl and index your pre-rendered HTML.

### Why SSR Matters for SEO

| Rendering | SEO Impact |
|-----------|------------|
| CSR (Client-Side) | ❌ Search engines see empty HTML, rely on JavaScript execution |
| SSR (Server-Side) | ✅ Full HTML sent on first request, immediately indexable |
| Static Generation | ✅ Pre-built HTML, fastest for static content |

Leptos SSR gives you the best of both worlds: full SEO visibility with dynamic hydration.

---

### 1. Title Tags (Most Important SEO Element)

Google recommendation: Each page needs a unique, descriptive title (50-60 chars).

**Leptos Implementation:**
```rust
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    view! {
        // Global title formatter - appends site name to all pages
        <Title formatter=|text| format!("{text} | XFTradesmen")/>
        
        // Other meta tags...
    }
}

// Per-page title override
#[component]
pub fn PricingPage() -> impl IntoView {
    view! {
        <Title text="Pricing Plans - Start from £29/month"/>
        // Page content...
    }
}
```

**Best Practices:**
- ✅ Unique title per page
- ✅ Include primary keyword near the beginning
- ✅ Keep under 60 characters (truncated in search results)
- ❌ Avoid generic titles like "Home" or "Page 1"
- ❌ Don't stuff keywords

---

### 2. Meta Descriptions (Drive Click-Through Rate)

Google recommendation: 150-160 characters, compelling summary.

**Leptos Implementation:**
```rust
#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <Title text="Contact Us"/>
        <Meta 
            name="description" 
            content="Get a free quote for your handyman project. 
                     Same-day response, fully insured professionals in Coventry. 
                     Call 07833 263486 or fill in our online form."
        />
        // Page content...
    }
}
```

**Best Practices:**
- ✅ Summarize page content accurately
- ✅ Include call-to-action ("Get a quote", "Learn more")
- ✅ Include phone number for local businesses
- ❌ Don't use same description on multiple pages
- ❌ Don't just list keywords

---

### 3. URL Structure (Readable & Hierarchical)

Google recommendation: Use descriptive words, create logical hierarchy.

**Leptos Router Implementation:**
```rust
use leptos_router::*;

#[component]
pub fn AppRouter() -> impl IntoView {
    view! {
        <Routes fallback=|| "404 Not Found">
            // Good: Descriptive, hierarchical
            <Route path="/" view=HomePage/>
            <Route path="/services" view=ServicesPage/>
            <Route path="/services/plumbing" view=PlumbingPage/>
            <Route path="/services/electrical" view=ElectricalPage/>
            <Route path="/pricing" view=PricingPage/>
            <Route path="/contact" view=ContactPage/>
            
            // Good: Location-based for local SEO
            <Route path="/handyman-coventry" view=HandymanHome/>
            <Route path="/handyman-coventry/services" view=HandymanServices/>
        </Routes>
    }
}
```

**URL Do's and Don'ts:**

| ✅ Good | ❌ Bad |
|---------|--------|
| `/services/plumbing` | `/page?id=123&cat=5` |
| `/handyman-coventry` | `/h1` |
| `/pricing` | `/pricing-plans-cheap-best-deals` |

---

### 4. Heading Hierarchy (Content Structure)

Google recommendation: One H1 per page, logical H2-H6 hierarchy.

**Leptos Implementation:**
```rust
#[component]
pub fn ServicesPage() -> impl IntoView {
    view! {
        <article>
            // Only ONE h1 per page
            <h1>"Professional Handyman Services in Coventry"</h1>
            
            // Logical hierarchy
            <section>
                <h2>"Home Repairs"</h2>
                <p>"We fix everything from..."</p>
                
                <h3>"Minor Repairs"</h3>
                <p>"Door handles, hinges..."</p>
                
                <h3>"Major Repairs"</h3>
                <p>"Drywall, flooring..."</p>
            </section>
            
            <section>
                <h2>"Assembly Services"</h2>
                <p>"Flat-pack furniture..."</p>
            </section>
        </article>
    }
}
```

---

### 5. Image Optimization

Google recommendation: Descriptive filenames, alt text, proper formats.

**Leptos Implementation:**
```rust
// Good image implementation
view! {
    <img 
        src="/images/plumber-fixing-leak-coventry.webp"  // Descriptive filename
        alt="Professional plumber fixing kitchen sink leak in Coventry home"  // Descriptive alt
        width="800"
        height="600"
        loading="lazy"  // Lazy load for performance
    />
}

// For hero images (LCP - Largest Contentful Paint)
view! {
    <img 
        src="/images/hero-handyman-team.webp"
        alt="XFTradesmen professional handyman team ready to help"
        width="1200"
        height="630"
        fetchpriority="high"  // Prioritize loading
    />
}
```

**Image Checklist:**
- ✅ Use descriptive filenames: `plumber-coventry.webp` not `IMG_001.jpg`
- ✅ Alt text describes the image AND context
- ✅ Use WebP format for 30% smaller files
- ✅ Include dimensions to prevent layout shift
- ✅ Lazy load below-the-fold images

---

### 6. Internal Linking & Anchor Text

Google recommendation: Descriptive anchor text, logical link structure.

**Leptos Implementation:**
```rust
// Good: Descriptive anchor text
view! {
    <p>
        "Need help with your bathroom? Check out our "
        <a href="/services/plumbing">"plumbing services"</a>
        " or "
        <a href="/contact">"get a free quote"</a>
        "."
    </p>
}

// Bad: Generic anchor text
view! {
    <p>
        "For plumbing services, "
        <a href="/services/plumbing">"click here"</a>  // ❌ Not descriptive
        "."
    </p>
}
```

---

### 7. Structured Data (JSON-LD)

Google recommendation: Schema.org markup for rich snippets.

**Leptos Local Business Schema:**
```rust
#[component]
pub fn LocalBusinessSchema() -> impl IntoView {
    view! {
        <script type="application/ld+json">
            {r#"{
                "@context": "https://schema.org",
                "@type": "LocalBusiness",
                "name": "XFTradesmen",
                "description": "Professional handyman services in Coventry",
                "url": "https://xftradesmen.fly.dev",
                "telephone": "+44-7833-263486",
                "email": "hello@xftradesmen.com",
                "address": {
                    "@type": "PostalAddress",
                    "addressLocality": "Coventry",
                    "addressRegion": "West Midlands",
                    "postalCode": "CV1",
                    "addressCountry": "GB"
                },
                "geo": {
                    "@type": "GeoCoordinates", 
                    "latitude": 52.4068,
                    "longitude": -1.5197
                },
                "openingHoursSpecification": {
                    "@type": "OpeningHoursSpecification",
                    "dayOfWeek": ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"],
                    "opens": "08:00",
                    "closes": "18:00"
                },
                "priceRange": "££",
                "sameAs": [
                    "https://facebook.com/xftradesmen",
                    "https://instagram.com/xftradesmen"
                ]
            }"#}
        </script>
    }
}
```

**Service Schema:**
```rust
#[component]
pub fn ServiceSchema(
    name: &'static str,
    description: &'static str,
    url: &'static str,
) -> impl IntoView {
    view! {
        <script type="application/ld+json">
            {format!(r#"{{
                "@context": "https://schema.org",
                "@type": "Service",
                "name": "{}",
                "description": "{}",
                "url": "{}",
                "provider": {{
                    "@type": "LocalBusiness",
                    "name": "XFTradesmen"
                }},
                "areaServed": {{
                    "@type": "City",
                    "name": "Coventry"
                }}
            }}"#, name, description, url)}
        </script>
    }
}
```

---

### 8. robots.txt & Sitemap

**robots.txt** (place in `public/` folder):
```
User-agent: *
Allow: /

# Block admin/internal pages
Disallow: /admin/
Disallow: /api/
Disallow: /_internal/

# Sitemap location
Sitemap: https://xftradesmen.fly.dev/sitemap.xml
```

**XML Sitemap** (generate dynamically or static):
```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>https://xftradesmen.fly.dev/</loc>
    <lastmod>2024-12-24</lastmod>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
  </url>
  <url>
    <loc>https://xftradesmen.fly.dev/services</loc>
    <changefreq>monthly</changefreq>
    <priority>0.8</priority>
  </url>
  <url>
    <loc>https://xftradesmen.fly.dev/pricing</loc>
    <changefreq>monthly</changefreq>
    <priority>0.8</priority>
  </url>
  <url>
    <loc>https://xftradesmen.fly.dev/contact</loc>
    <changefreq>monthly</changefreq>
    <priority>0.7</priority>
  </url>
</urlset>
```

---

### 9. Open Graph & Social Meta

**Leptos Implementation:**
```rust
#[component]
pub fn SocialMeta(
    title: &'static str,
    description: &'static str,
    image: &'static str,
    url: &'static str,
) -> impl IntoView {
    view! {
        // Open Graph (Facebook, LinkedIn)
        <Meta property="og:title" content=title/>
        <Meta property="og:description" content=description/>
        <Meta property="og:image" content=image/>
        <Meta property="og:url" content=url/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:site_name" content="XFTradesmen"/>
        
        // Twitter Cards
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content=title/>
        <Meta name="twitter:description" content=description/>
        <Meta name="twitter:image" content=image/>
    }
}
```

---

### 10. Mobile-First & Core Web Vitals

**Viewport Meta (required):**
```rust
view! {
    <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
}
```

**Performance Checklist:**

| Metric | Target | How to Achieve |
|--------|--------|----------------|
| LCP (Largest Contentful Paint) | < 2.5s | Optimize hero images, use CDN |
| FID (First Input Delay) | < 100ms | Minimize JavaScript, use SSR |
| CLS (Cumulative Layout Shift) | < 0.1 | Set image dimensions, avoid layout shifts |

**Leptos SSR Advantage:**
- Server renders full HTML first → LCP is fast
- Hydration is progressive → FID stays low
- Page structure is stable → CLS is minimal

---

### 11. Complete SEO Component

**Reusable SeoHead Component:**
```rust
use leptos::prelude::*;
use leptos_meta::*;

#[derive(Clone)]
pub struct PageMetadata {
    pub title: String,
    pub description: String,
    pub canonical_url: Option<String>,
    pub og_image: Option<String>,
    pub no_index: bool,
}

#[component]
pub fn SeoHead(metadata: PageMetadata) -> impl IntoView {
    let og_image = metadata.og_image.unwrap_or_else(|| 
        "https://xftradesmen.fly.dev/og-default.png".to_string()
    );
    
    view! {
        <Title text=metadata.title.clone()/>
        <Meta name="description" content=metadata.description.clone()/>
        
        // Canonical URL (prevents duplicate content)
        {metadata.canonical_url.map(|url| view! {
            <Link rel="canonical" href=url/>
        })}
        
        // Robots directive
        {if metadata.no_index {
            Some(view! { <Meta name="robots" content="noindex, nofollow"/> })
        } else {
            None
        }}
        
        // Open Graph
        <Meta property="og:title" content=metadata.title.clone()/>
        <Meta property="og:description" content=metadata.description.clone()/>
        <Meta property="og:image" content=og_image.clone()/>
        <Meta property="og:type" content="website"/>
        
        // Twitter
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content=metadata.title/>
        <Meta name="twitter:description" content=metadata.description/>
        <Meta name="twitter:image" content=og_image/>
    }
}
```

**Usage:**
```rust
#[component]
pub fn PricingPage() -> impl IntoView {
    view! {
        <SeoHead metadata=PageMetadata {
            title: "Pricing - Affordable Handyman Services".to_string(),
            description: "Transparent pricing starting at £29/month. No hidden fees.".to_string(),
            canonical_url: Some("https://xftradesmen.fly.dev/pricing".to_string()),
            og_image: Some("https://xftradesmen.fly.dev/og-pricing.png".to_string()),
            no_index: false,
        }/>
        
        // Page content...
    }
}
```

---

### SEO Verification Tools

| Tool | Purpose | URL |
|------|---------|-----|
| Google Search Console | Monitor indexing, search performance | https://search.google.com/search-console |
| Google Rich Results Test | Validate structured data | https://search.google.com/test/rich-results |
| PageSpeed Insights | Core Web Vitals analysis | https://pagespeed.web.dev |
| Lighthouse | Full SEO audit in Chrome DevTools | Built into Chrome |
| Facebook Sharing Debugger | Test Open Graph tags | https://developers.facebook.com/tools/debug |

---

### SEO Quick Wins Checklist

- [ ] Every page has unique `<Title>` with primary keyword
- [ ] Every page has unique meta description (150-160 chars)
- [ ] Only ONE `<h1>` per page
- [ ] All images have descriptive alt text
- [ ] Internal links use descriptive anchor text
- [ ] Canonical URLs set on all pages
- [ ] robots.txt allows crawling important pages
- [ ] sitemap.xml submitted to Google Search Console
- [ ] LocalBusiness schema on homepage
- [ ] Open Graph tags for social sharing
- [ ] Mobile viewport meta tag set
- [ ] HTTPS enforced (fly.toml `force_https = true`)

---

## Phase 20: Common Bugs & Gotchas

> [!CAUTION]
> These are the most common issues that trip up Leptos developers. Study these carefully!

### 1. Reactivity Issues

#### Don't write to signals from effects

```rust
// ❌ BAD: Creates inefficient update chains
let (a, set_a) = signal(0);
let (b, set_b) = signal(false);

Effect::new(move |_| {
    if a.get() > 5 {
        set_b.set(true);  // ⚠️ Triggers more effects!
    }
});

// ✅ GOOD: Derive from signals
let (a, set_a) = signal(0);
let b = move || a.get() > 5;  // No signal needed - just derive!
```

#### Nested signal updates cause panics

```rust
// ❌ Clicking twice will panic!
let resources = RwSignal::new(HashMap::new());

let update = move |id: usize| {
    resources.update(|r| {
        r.entry(id)
            .or_insert_with(|| RwSignal::new(0))
            .update(|v| *v += 1)  // ⚠️ Inner update triggers effect that reads outer!
    });
};

// ✅ GOOD: Use batch() to defer effects
let update = move |id: usize| {
    batch(move || {
        resources.update(|r| {
            r.entry(id)
                .or_insert_with(|| RwSignal::new(0))
                .update(|v| *v += 1)
        });
    });
};
```

---

### 2. Input Value Binding

#### Use `prop:value` NOT `value` for reactive inputs

```rust
// ❌ BAD: Stops updating after first keystroke
let (name, set_name) = signal("".to_string());
view! {
    <input value=name on:input=move |ev| set_name.set(event_target_value(&ev))/>
}

// ✅ GOOD: Use prop:value for reactive binding
let (name, set_name) = signal("".to_string());
view! {
    <input prop:value=name on:input=move |ev| set_name.set(event_target_value(&ev))/>
}
```

**Why?** `value` sets the HTML attribute (initial default). `prop:value` sets the DOM property (current value).

---

### 3. Workspace Resolver

#### Always set `resolver = "2"` in workspace Cargo.toml

```toml
# ❌ BAD: Missing resolver causes WASM build failures
[workspace]
members = ["frontend", "backend"]

# ✅ GOOD: Explicitly set resolver
[workspace]
members = ["frontend", "backend"]
resolver = "2"  # Required for 2021 edition feature resolution!
```

**Symptom:** Seeing `mio` fail to build for WASM target.

---

### 4. wasm-bindgen Version Mismatch

```bash
# Error you'll see:
# it looks like the Rust project used to create this WASM file 
# was linked against version 0.2.X of wasm-bindgen

# Fix: Pin version in Cargo.toml AND install matching CLI
cargo install wasm-bindgen-cli --version 0.2.106
```

```toml
# Cargo.toml - pin exact version
wasm-bindgen = "=0.2.106"
```

---

### 5. CSS Not Loading

| Symptom | Cause | Fix |
|---------|-------|-----|
| Unstyled page | CSS not built | Run `npm run build:css` before cargo-leptos |
| Partially styled | Wrong href | Check `<Stylesheet href="/correct-path.css"/>` |
| Tailwind missing | Wrong content paths | Check `tailwind.config.js` content array |

---

### 6. Hydration Mismatch

**Symptom:** Console errors about "hydration mismatch" or page flickering.

**Causes:**
1. Server and client render different HTML
2. `Date::now()` or random values used during render
3. External data differing between SSR and hydration

**Fix:**
```rust
// Use Suspense for async data
view! {
    <Suspense fallback=|| view! { <p>"Loading..."</p> }>
        {move || resource.get().map(|data| view! { <p>{data}</p> })}
    </Suspense>
}
```

---

### 7. Server Function Errors

```rust
// ❌ BAD: Forgetting #[server] annotation or wrong path
async fn fetch_data() -> Result<String, ServerFnError> { ... }

// ✅ GOOD: Proper server function
#[server(FetchData)]
pub async fn fetch_data() -> Result<String, ServerFnError> {
    // This runs ONLY on the server
    Ok("data".to_string())
}
```

---

## Phase 21: Complete Project Template

### Cargo.toml Template (Frontend)

```toml
[package]
name = "your-app"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[[bin]]
name = "your-app"
path = "src/main.rs"

[dependencies]
# Core Leptos
leptos = { version = "0.7", features = ["tracing"] }
leptos_meta = { version = "0.7" }
leptos_router = { version = "0.7" }
leptos_axum = { version = "0.7", optional = true }

# SSR dependencies (only with ssr feature)
axum = { version = "0.8", optional = true }
tokio = { version = "1", features = ["rt-multi-thread", "macros"], optional = true }
tower = { version = "0.4", optional = true }
tower-http = { version = "0.5", features = ["fs"], optional = true }
tracing = { version = "0.1", optional = true }

# WASM dependencies
console_error_panic_hook = "0.1"
wasm-bindgen = "=0.2.106"

# Shared
serde = { version = "1", features = ["derive"] }
thiserror = "2"
http = "1"

[features]
default = ["ssr"]
hydrate = ["leptos/hydrate"]
ssr = [
    "dep:axum",
    "dep:tokio",
    "dep:tower",
    "dep:tower-http",
    "dep:leptos_axum",
    "dep:tracing",
    "leptos/ssr",
    "leptos_meta/ssr",
    "leptos_router/ssr",
]

[package.metadata.leptos]
output-name = "your-app"
site-root = "target/site"
site-pkg-dir = "pkg"
assets-dir = "public"
site-addr = "127.0.0.1:3000"
reload-port = 3001
tailwind-input-file = "input.css"
browserquery = "defaults"
watch = false
env = "DEV"
bin-features = ["ssr"]
bin-default-features = false
lib-features = ["hydrate"]
lib-default-features = false
```

---

### lib.rs Template

```rust
//! Your App - Frontend Library
#![recursion_limit = "1024"]

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_router::components::*;

pub mod components;
pub mod pages;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    view! {
        <Html attr:lang="en-gb"/>
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Stylesheet id="leptos" href="/pkg/your-app.css"/>
        <Title formatter=|text| format!("{text} | Your App")/>
        
        <Router>
            <Routes fallback=|| view! { <NotFound/> }>
                <Route path=path!("/") view=pages::Home/>
                <Route path=path!("/about") view=pages::About/>
                <Route path=path!("/contact") view=pages::Contact/>
            </Routes>
        </Router>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <Title text="404 - Not Found"/>
        <main class="container">
            <h1>"Page Not Found"</h1>
            <a href="/">"Go Home"</a>
        </main>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
```

---

### main.rs Template (SSR)

```rust
//! Your App - SSR Entry Point
#![recursion_limit = "1024"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use your_app::App;

    // Load configuration
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    // Generate routes
    let routes = generate_route_list(App);

    // Build Axum router
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // Start server
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on http://{}", addr);
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(feature = "ssr")]
fn shell(options: LeptosOptions) -> impl IntoView {
    use leptos::prelude::*;
    use leptos_meta::*;
    use your_app::App;
    
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[cfg(not(feature = "ssr"))]
fn main() {}
```

---

### Directory Structure

```
your-app/
├── Cargo.toml              # Dependencies and Leptos config
├── Leptos.toml             # (Optional) Alternative config location
├── Dockerfile.production   # Multi-stage production build
├── fly.toml                # Fly.io deployment config
├── start_fly.sh            # Startup script for Fly.io
├── package.json            # CSS build scripts
├── input.css               # Tailwind input
├── public/                 # Static assets
│   ├── favicon.ico
│   ├── robots.txt
│   └── sitemap.xml
└── src/
    ├── lib.rs              # App component, hydrate function
    ├── main.rs             # SSR server entry point
    ├── components/         # Reusable UI components
    │   ├── mod.rs
    │   ├── layout.rs       # Navbar, Footer
    │   ├── seo.rs          # SeoHead component
    │   └── design_system.rs
    └── pages/              # Route pages
        ├── mod.rs
        ├── home.rs
        ├── about.rs
        └── contact.rs
```

---

### Quick Start Commands

```bash
# 1. Create new project (from template)
cargo leptos new --git leptos-rs/start

# 2. Install dependencies
npm install

# 3. Build CSS
npm run build:css

# 4. Run development server
cargo leptos watch

# 5. Build for production
cargo leptos build --release

# 6. Deploy to Fly.io
fly deploy
```

---

## Final Checklist: Production-Ready Leptos App

### Code Quality
- [ ] `#![recursion_limit = "1024"]` in lib.rs AND main.rs
- [ ] `resolver = "2"` in workspace Cargo.toml
- [ ] `wasm-bindgen` version pinned and matching CLI
- [ ] No signals written from effects
- [ ] `prop:value` used for all input binding
- [ ] All `#[server]` functions properly annotated

### Configuration
- [ ] Leptos.toml configured with correct paths
- [ ] Feature flags properly split (ssr vs hydrate)
- [ ] CSS build integrated (Tailwind or vanilla)

### SEO
- [ ] Unique `<Title>` per page
- [ ] Meta descriptions on all pages
- [ ] Structured data (JSON-LD) on homepage
- [ ] Open Graph tags for social sharing
- [ ] robots.txt and sitemap.xml in public folder

### Deployment
- [ ] Dockerfile.production with cargo-chef
- [ ] fly.toml configured with health checks
- [ ] start_fly.sh using `exec` for main process
- [ ] Environment secrets set in Fly.io
- [ ] HTTPS enforced

### Performance
- [ ] Images optimized (WebP, lazy loading)
- [ ] CSS minified
- [ ] `<Suspense>` for async data
- [ ] Core Web Vitals passing

---

> **This guide was created from:**
> - Leptos official repository examples (tailwind_axum, todo_app_sqlite_axum)
> - Leptos ARCHITECTURE.md and COMMON_BUGS.md
> - Axum ECOSYSTEM.md and examples
> - Google SEO Starter Guide
> - Real deployment debugging experience on Fly.io
