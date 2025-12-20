# Observability & Telemetry Plan for XFHandyman

This document outlines a comprehensive strategy for implementing observability and telemetry for the XFHandyman application. The goal is to gain deep visibility into the system's performance, reliability, and user experience, enabling proactive debugging and optimized operations.

## 1. Core Philosophy: The Three Pillars

We will leverage the three pillars of observability:
1.  **Logs**: Discrete events (e.g., "Incoming request", "Database error").
2.  **Metrics**: Aggregable numerical data (e.g., "Requests per second", "Memory usage").
3.  **Traces**: The journey of a request across services (e.g., Frontend → Backend → Database).

**Goal**: Move from *"The system is down"* to *"The database pool is exhausted because of high latency in the `booking` query starting at 14:02 UTC."*

---

## 2. Structured Logging (The Foundation)

Current state: Basic `tracing-subscriber` with `fmt::layer`.

### Recommendation: JSON in Production
Human-readable logs are great for local dev, but machines (like Grafana Loki or AWS CloudWatch) parse JSON best.

-   **Action**: Update `main.rs` to switch between pretty logs (Dev) and JSON logs (Prod).
-   **Crates**: `tracing-subscriber` with `json` feature.

```rust
// In main.rs (Concept)
let subscriber = tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env());

if cfg!(debug_assertions) {
    subscriber.pretty().init();
} else {
    subscriber.json().init(); // JSON for production
}
```

### Request Correlation
Ensure every request has a unique `x-request-id`. This should be generated at the edge (or by the first middleware) and passed to all logs and downstream calls.

-   **Action**: Use `tower_http::request_id::MakeRequestUuid` middleware.

---

## 3. Metrics (Prometheus Integration)

Current state: `metrics` and `metrics-exporter-prometheus` are in `Cargo.toml` but not fully utilized.

### Implementation Plan
The **RED Method** (Rate, Errors, Duration) is the industry standard for HTTP APIs.

1.  **Expose Metrics Endpoint**:
    -   Mount a `/metrics` route in Axum that scrapes the internal registry.
    -   Secure this endpoint so only internal tools (or Fly.io metrics scraper) can access it.

2.  **Instrument Handlers**:
    -   Use `metrics` crate macros or middleware to auto-record:
        -   `http_requests_total` (counter, labelled by status, method, route)
        -   `http_request_duration_seconds` (histogram)

3.  **Database Connection Metrics**:
    -   BB8 exposes simple stats (state). We should periodically gauge:
        -   `db_pool_size`
        -   `db_connections_active`
        -   `db_connections_idle`

**Action**: Initialize `PrometheusBuilder` in `main.rs` and add a `axum_prometheus` layer (or manual middleware).

---

## 4. Distributed Tracing (OpenTelemetry)

Tracing allows us to visualize the latency waterfall of a single user request.

### Integration
1.  **Crates**: Add `opentelemetry`, `opentelemetry-otlp`, `tracing-opentelemetry`.
2.  **Setup**:
    -   Configure an OTLP exporter (to send data to Honeycomb, Jaeger, or Grafana Tempo).
    -   Fly.io creates basic traces, but application-level tracing is crucial for database queries.
3.  **Instrumentation**:
    -   Add `#[tracing::instrument]` to key async functions (e.g., `handle_booking`, `Repositories::create_booking`).
    -   Ensure `sqlx` instrumentation is enabled (it usually is by default at `INFO` level).

---

## 5. Error Tracking (Sentry)

Logs are for digging; Error Trackers are for alerting. Sentry provides stack traces, user context, and release tracking.

### Backend (Axum)
-   **Crate**: `sentry` + `sentry-tracing`.
-   **Strategy**:
    -   Initialize Sentry in `main.rs`.
    -   Capture `panic!`s and `error` level logs automatically.
    -   Ensure `anyhow::Result` errors in handlers are captured before converting to HTTP responses.

### Frontend (Leptos)
-   **Context**: Client-side errors are invisible to backend logs.
-   **Strategy**:
    -   Use Sentry specific WASM/JS SDK or Rust bindings if available/stable.
    -   Create an `ErrorBoundary` in the Leptos app root to catch panics and send them to Sentry.

---

## 6. Frontend Telemetry (Real User Monitoring)

User experience is not just about errors, but also performance.

1.  **Web Vitals**: Measure LCP (Largest Contentful Paint), CLS (Cumulative Layout Shift) in the browser.
2.  **Route Change Tracking**: Log navigation events in the SPA.
3.  **API Latency from User Perspective**: Log how long `fetch` requests take from the browser's point of view.

**Implementation**:
-   Inject a small JS snippet or use a Rust-based `gloo` wrapper to report these metrics to the backend `/api/telemetry` or directly to a provider (like PostHog or Google Analytics).

---

## 7. Health Checks & Probes

Current state: Basic `/api/health`.

### Enhancement
The health check must be "Deep". It shouldn't just return 200 OK; it should check dependencies.

-   **Database**: Run a simple `SELECT 1` on the pool.
-   **External APIs**: Check connectivity to Stripe/Email services (optional/cached).

**Response Format**:
```json
{
  "status": "healthy", // or "degraded"
  "checks": {
    "database": "ok",
    "stripe": "ok",
    "memory": "ok"
  },
  "version": "0.1.0"
}
```

---

## 8. Dashboarding Strategy (Grafana/Fly Metrics)

Fly.io provides built-in Grafana. We should leverage custom dashboards.

**Key Dashboard Panels**:
1.  **Traffic**: Req/sec broken down by Status Code (2xx, 4xx, 5xx).
2.  **Latency**: p50, p95, p99 Histograms.
3.  **Saturation**: CPU Load, Memory Usage, DB Pool exhaustion.
4.  **Business Metrics**:
    -   "Bookings created per hour"
    -   "Contact forms submitted"

---

## 9. Implementation Roadmap

### Phase 1: Foundations (Immediate)
-   [ ] Switch to JSON logging in Production.
-   [ ] Add Request ID middleware.
-   [ ] Implement Deep Health Check.

### Phase 2: Metrics (Next)
-   [ ] Enable `metrics-exporter-prometheus`.
-   [ ] Add middleware to measure Request Duration.

### Phase 3: External Observability (Soon)
-   [ ] Set up Sentry for Backend/Frontend.
-   [ ] Configure OpenTelemetry OTLP exporter.

### Phase 4: Business Intelligence
-   [ ] Create custom business metric counters.
-   [ ] Build "Handyman Operations" dashboard.
