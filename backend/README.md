# Backend

Axum-based HTTP API server with layered architecture.

## Structure

```
backend/
├── apps/
│   └── api/             # Main HTTP server
│       ├── main.rs      # Entry point
│       ├── config.rs    # Configuration
│       ├── middleware.rs # HTTP middleware
│       └── web/
│           ├── handlers/ # Request handlers
│           └── routes_*.rs # Route definitions
└── libs/
    ├── lib-core/        # Business logic & models
    ├── lib-web/         # HTTP utilities
    └── lib-utils/       # Shared utilities
```

## Architecture

```
Request → Middleware → Handler → BMC → Database
                          ↓
                     ModelManager
```

## Running

```bash
# Development
cargo run -p api

# Test database connection
cargo run -p api -- --test-db

# Run migrations
cargo run -p api -- --migrate
```

## Key Patterns

- **ModelManager**: Central resource holder (db pool, services)
- **BMC**: Backend Model Controller for each entity
- **ValidatedJson**: Auto-validating JSON extractor

## Environment

```bash
DATABASE_URL=postgres://...
SMTP_USERNAME=...
SMTP_PASSWORD=...
STRIPE_SECRET_KEY=sk_...
```

See [IDM/01_backend_architecture.md](../IDM/01_backend_architecture.md) for details.
