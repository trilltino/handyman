# lib-web

**Axum middleware stack and web utilities for the Handyman Marketplace.**

## Purpose

`lib-web` provides all HTTP-level concerns:

- **Authentication Middleware**: Extract and validate JWT tokens
- **Request Stamping**: Assign unique IDs to requests for tracing
- **Context Resolution**: Create `Ctx` from JWT claims
- **Error Mapping**: Convert application errors to HTTP responses
- **Logging**: Structured request/response logging

This library bridges the gap between HTTP (Axum) and domain logic (lib-core).

---

## Middleware Stack

Requests flow through middleware in this order:

```
1. mw_req_stamp     → Assign request ID, start timer
2. mw_ctx_resolver  → Extract JWT, create Ctx (optional)
3. mw_ctx_require   → Enforce authentication (protected routes)
4. Handler          → Execute business logic
5. mw_res_map       → Map errors to HTTP responses, log
```

### Architecture Diagram

```
HTTP Request
     │
     ▼
┌────────────────────────────┐
│   mw_req_stamp             │  Assign req_id, start timer
│   - Generate UUID          │
│   - Store in extensions    │
└────────────┬───────────────┘
             ▼
┌────────────────────────────┐
│   mw_ctx_resolver          │  Extract JWT from cookie
│   - Parse auth_token       │  (OPTIONAL - doesn't fail)
│   - Validate JWT           │
│   - Create Ctx             │
│   - Store in extensions    │
└────────────┬───────────────┘
             ▼
┌────────────────────────────┐
│   mw_ctx_require           │  Require authentication
│   - Check Ctx exists       │  (PROTECTED ROUTES ONLY)
│   - Return 401 if missing  │
└────────────┬───────────────┘
             ▼
┌────────────────────────────┐
│   Route Handler            │  Business logic
│   - Extract Ctx            │
│   - Call BMC methods       │
│   - Return result          │
└────────────┬───────────────┘
             ▼
┌────────────────────────────┐
│   mw_res_map               │  Map errors, log response
│   - Convert ModelError     │
│   - Log with req_id        │
│   - Return HTTP response   │
└────────────┬───────────────┘
             ▼
HTTP Response
```

---

## Middleware Reference

### 1. Request Stamping (`mw_req_stamp`)

**Purpose**: Assign unique ID to each request for distributed tracing.

**What it does**:
- Generates UUID for request
- Stores in request extensions
- Starts timer for request duration
- Logs request start

**Usage**:
```rust
use axum::Router;
use lib_web::middleware::mw_req_stamp;

let app = Router::new()
    // Apply globally to all routes
    .layer(middleware::from_fn(mw_req_stamp))
    .route("/api/users", get(handler));
```

**Access request ID in handler**:
```rust
use lib_web::ReqStamp;

async fn handler(Extension(req_stamp): Extension<ReqStamp>) -> String {
    format!("Request ID: {}", req_stamp.id())
}
```

### 2. Context Resolver (`mw_ctx_resolver`)

**Purpose**: Extract JWT token and create `Ctx` if present.

**What it does**:
- Reads `auth_token` cookie
- Validates JWT signature and expiration
- Creates `Ctx` from JWT claims
- Stores `Ctx` in request extensions
- **Does NOT fail** if token missing (optional auth)

**Usage**:
```rust
use lib_web::middleware::mw_ctx_resolver;

let app = Router::new()
    .layer(middleware::from_fn(mw_ctx_resolver))
    .route("/api/profile", get(get_profile));  // Ctx available if logged in
```

**Extract Ctx in handler** (optional):
```rust
use lib_core::Ctx;

async fn handler(ctx: Option<Extension<Ctx>>) -> String {
    match ctx {
        Some(Extension(ctx)) => {
            format!("Logged in as user {}", ctx.user_id())
        },
        None => {
            "Not logged in".to_string()
        }
    }
}
```

### 3. Context Require (`mw_ctx_require`)

**Purpose**: Enforce authentication for protected routes.

**What it does**:
- Checks if `Ctx` exists in extensions
- Returns **401 Unauthorized** if missing
- Allows request to proceed if `Ctx` present

**Usage**:
```rust
use lib_web::middleware::mw_ctx_require;

// Protected routes (require authentication)
let protected = Router::new()
    .route("/api/bookings", get(list_bookings))
    .route("/api/dashboard", get(dashboard))
    .layer(middleware::from_fn(mw_ctx_require));  // <-- Enforce auth

// Public routes (no auth required)
let public = Router::new()
    .route("/api/auth/login", post(login))
    .route("/health", get(health_check));

// Combine
let app = Router::new()
    .merge(public)
    .merge(protected);
```

**Extract Ctx in protected handler** (guaranteed to exist):
```rust
use lib_core::Ctx;

async fn list_bookings(
    Extension(ctx): Extension<Ctx>,  // <-- No Option<>
    Extension(mm): Extension<ModelManager>,
) -> Result<Json<Vec<Booking>>, Error> {
    let bookings = BookingBmc::list(&ctx, &mm, None, None).await?;
    Ok(Json(bookings))
}
```

### 4. Response Mapper (`mw_res_map`)

**Purpose**: Convert application errors to HTTP responses and log.

**What it does**:
- Intercepts responses from handlers
- Maps `ModelError` → HTTP status codes
- Logs response with request ID and duration
- Formats error messages for clients

**Usage**:
```rust
use lib_web::middleware::mw_res_map;

let app = Router::new()
    .layer(middleware::from_fn(mw_res_map))  // <-- Apply last
    .route("/api/users/:id", get(get_user));
```

**Error mapping**:
| Application Error | HTTP Status | Response Body |
|-------------------|-------------|---------------|
| `ModelError::NotFound` | 404 Not Found | `{"error": "User 123 not found"}` |
| `ModelError::Unauthorized` | 401 Unauthorized | `{"error": "Unauthorized"}` |
| `ModelError::ValidationError` | 400 Bad Request | `{"error": "Invalid email"}` |
| `ModelError::Database` | 500 Internal Server Error | `{"error": "Internal server error"}` |

**Logging**:
```
INFO request completed req_id=abc-123 path=/api/users/456 method=GET status=200 duration=45ms
WARN request failed req_id=def-456 path=/api/users/999 method=GET status=404 duration=12ms error="User 999 not found"
```

---

## Complete Example

### Setup Middleware Stack

```rust
use axum::{
    Router,
    middleware,
    routing::{get, post},
};
use lib_web::middleware::{
    mw_req_stamp,
    mw_ctx_resolver,
    mw_ctx_require,
    mw_res_map,
};

#[tokio::main]
async fn main() {
    // Initialize model manager
    let mm = ModelManager::new().await.unwrap();

    // Build protected routes
    let protected_routes = Router::new()
        .route("/api/bookings", get(list_bookings))
        .route("/api/profile", get(get_profile))
        .layer(middleware::from_fn(mw_ctx_require));  // Require auth

    // Build public routes
    let public_routes = Router::new()
        .route("/api/auth/login", post(login))
        .route("/api/auth/register", post(register))
        .route("/health", get(health_check));

    // Combine and apply global middleware
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(Extension(mm))
        // Global middleware (order matters!)
        .layer(middleware::from_fn(mw_res_map))       // 4. Map errors (outermost)
        .layer(middleware::from_fn(mw_ctx_resolver))  // 2. Resolve Ctx
        .layer(middleware::from_fn(mw_req_stamp));    // 1. Stamp request (innermost)

    // Start server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

### Handler Examples

**Public handler** (no auth required):
```rust
async fn health_check() -> &'static str {
    "OK"
}
```

**Optional auth handler**:
```rust
async fn get_homepage(
    ctx: Option<Extension<Ctx>>,
) -> Html<String> {
    let html = match ctx {
        Some(Extension(ctx)) => {
            format!("<h1>Welcome back, user {}</h1>", ctx.user_id())
        },
        None => {
            "<h1>Welcome! Please log in</h1>".to_string()
        }
    };
    Html(html)
}
```

**Protected handler** (auth required):
```rust
async fn get_profile(
    Extension(ctx): Extension<Ctx>,  // <-- Guaranteed by mw_ctx_require
    Extension(mm): Extension<ModelManager>,
) -> Result<Json<User>, AppError> {
    let user = UserBmc::get(&ctx, &mm, ctx.user_id()).await?;
    Ok(Json(user))
}
```

---

## Error Handling

### Custom Error Type

Create an app-specific error type that implements `IntoResponse`:

```rust
use axum::{
    response::{Response, IntoResponse},
    http::StatusCode,
    Json,
};
use serde_json::json;

pub enum AppError {
    NotFound(String),
    Unauthorized,
    BadRequest(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

// Convert ModelError to AppError
impl From<ModelError> for AppError {
    fn from(err: ModelError) -> Self {
        match err {
            ModelError::NotFound { entity, id } => {
                AppError::NotFound(format!("{} {} not found", entity, id))
            },
            ModelError::Unauthorized => AppError::Unauthorized,
            ModelError::ValidationError { field, reason } => {
                AppError::BadRequest(format!("{}: {}", field, reason))
            },
            _ => AppError::Internal(err.to_string()),
        }
    }
}
```

---

## Configuration

### Environment Variables

```bash
# JWT secret (required for mw_ctx_resolver)
JWT_SECRET=your-super-secret-jwt-key-at-least-64-chars

# Log level
RUST_LOG=info,lib_web=debug

# CORS (optional)
CORS_ALLOWED_ORIGINS=http://localhost:3000,https://example.com
```

---

## Testing

### Testing Middleware

```rust
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;  // for oneshot()

#[tokio::test]
async fn test_protected_route_without_auth() {
    let app = Router::new()
        .route("/protected", get(|| async { "OK" }))
        .layer(middleware::from_fn(mw_ctx_require));

    let response = app
        .oneshot(Request::builder().uri("/protected").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_protected_route_with_valid_token() {
    // Setup app
    let app = /* ... */;

    // Generate valid JWT
    let token = generate_test_token(user_id, tradesman_id, "admin");

    // Make request with cookie
    let response = app
        .oneshot(
            Request::builder()
                .uri("/protected")
                .header("Cookie", format!("auth_token={}", token))
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
```

---

## Best Practices

### ✅ DO

- Apply `mw_req_stamp` first (innermost) for request tracking
- Apply `mw_res_map` last (outermost) to catch all errors
- Use `mw_ctx_require` only on protected routes (not globally)
- Extract `Ctx` as `Extension<Ctx>` in handlers
- Log errors with request ID from `ReqStamp`
- Return semantic HTTP status codes

### ❌ DON'T

- Don't apply `mw_ctx_require` globally (breaks public routes)
- Don't bypass middleware for "quick hacks"
- Don't ignore authentication failures
- Don't return sensitive info in error messages
- Don't block the async runtime in middleware

---

## Troubleshooting

### "401 Unauthorized" on all requests

**Cause**: `mw_ctx_require` applied globally instead of only to protected routes.

**Fix**: Split routes into public and protected:
```rust
let protected = Router::new()
    .route("/api/bookings", get(handler))
    .layer(middleware::from_fn(mw_ctx_require));  // Only here

let public = Router::new()
    .route("/api/auth/login", post(login));  // No auth

let app = Router::new().merge(public).merge(protected);
```

### JWT validation failing

**Cause**: Mismatched `JWT_SECRET` or expired token.

**Check**:
```bash
# Ensure JWT_SECRET is set
echo $JWT_SECRET

# Decode token (without validation) to check expiration
curl -X POST https://jwt.io/api/decode -d '{"token":"YOUR_TOKEN"}'
```

### Request ID not showing in logs

**Cause**: `mw_req_stamp` not applied or applied in wrong order.

**Fix**: Apply as first (innermost) middleware:
```rust
.layer(middleware::from_fn(mw_req_stamp))  // <-- First
```

---

## Dependencies

- **axum** - Web framework
- **tower** - Middleware primitives
- **tower-http** - HTTP middleware (CORS, etc.)
- **tower-cookies** - Cookie parsing
- **lib-core** - Ctx and models
- **lib-auth** - JWT validation
- **uuid** - Request IDs
- **tracing** - Structured logging

---

## Roadmap

### Planned Features
- [ ] Rate limiting middleware
- [ ] CSRF protection
- [ ] Request size limits
- [ ] Compression middleware (gzip, brotli)
- [ ] Metrics middleware (Prometheus)
- [ ] Distributed tracing (OpenTelemetry)

### Security Enhancements
- [ ] IP-based rate limiting
- [ ] Suspicious activity detection
- [ ] Request fingerprinting
- [ ] Bot detection

---

## License

Copyright © 2025 Handyman Marketplace. All rights reserved.
