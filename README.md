# XF Tradesmen

Professional website platform for UK tradespeople. Built with Rust, Leptos, and Axum.

## Quick Start

```bash
# Prerequisites
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos
npm install

# Development
npm run watch:css  # Terminal 1
cargo leptos watch  # Terminal 2

# Open http://localhost:3001
```

## Project Structure

```
xfhandyman/
├── backend/
│   ├── apps/api/          # Axum HTTP server
│   └── libs/
│       ├── lib-core/      # Business logic, models
│       ├── lib-web/       # HTTP utilities
│       └── lib-utils/     # Shared utilities
├── frontend-leptos/       # Leptos SSR + WASM
│   ├── src/
│   │   ├── components/    # UI components
│   │   └── pages/         # Route pages
│   └── public/            # Static assets
├── shared/                # Cross-stack types
├── migrations/            # SQL migrations
└── IDM/                   # Documentation
```

## Documentation

See `IDM/` folder for comprehensive documentation:

- [00_overview.md](IDM/00_overview.md) - Project overview
- [01_backend_architecture.md](IDM/01_backend_architecture.md) - Backend patterns
- [02_frontend_architecture.md](IDM/02_frontend_architecture.md) - Leptos patterns
- [03_shared_types.md](IDM/03_shared_types.md) - Shared types & validation
- [04_database_schema.md](IDM/04_database_schema.md) - Database schema
- [05_api_design.md](IDM/05_api_design.md) - API endpoints
- [06_deployment.md](IDM/06_deployment.md) - Deployment guide

## Commands

| Command | Description |
|---------|-------------|
| `cargo leptos watch` | Dev server with hot reload |
| `cargo leptos build --release` | Production build |
| `cargo test` | Run all tests |
| `npm run build:css` | Build Tailwind CSS |

## Environment Variables

```bash
DATABASE_URL=postgres://...
SMTP_USERNAME=...
SMTP_PASSWORD=...
STRIPE_PUBLIC_KEY=pk_...
STRIPE_SECRET_KEY=sk_...
```

## License

MIT OR Apache-2.0
