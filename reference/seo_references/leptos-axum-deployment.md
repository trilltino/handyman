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

---

## Phase 16: Fly.io Deployment

### fly.toml Essentials
```toml
app = "xftradesmen"
primary_region = "lhr"

[http_service]
  internal_port = 8080
  force_https = true

[[http_service.checks]]
  path = "/api/health"
  interval = "30s"
  timeout = "5s"
```

### Environment Secrets
```bash
fly secrets set DATABASE_URL="postgres://..."
fly secrets set JWT_SECRET="..."
fly secrets set SMTP_USERNAME="..."
fly secrets set SMTP_PASSWORD="..."
```

### Deploy Command
```bash
fly deploy
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
```

