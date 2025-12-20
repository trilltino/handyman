//! # Backend Main Entry Point - Handyman Marketplace API Server
//!
//! ## Purpose
//! This is the main entry point for the handyman marketplace backend API server.
//! It initializes and configures the Axum web server with all necessary middleware,
//! database connections, routes, and CORS settings.
//!
//! ## How It Works
//! 1. **Module Organization**: Imports all application modules (db, models, handlers, etc.)
//! 2. **Server Initialization**: Sets up async Tokio runtime
//! 3. **Database Connection**: Creates PostgreSQL connection pool using bb8
//! 4. **Route Configuration**: Defines public and protected API endpoints
//! 5. **Middleware Stack**: Applies authentication, logging, and response mapping
//! 6. **CORS Setup**: Configures cross-origin requests from frontend (port 8080)
//! 7. **Server Launch**: Binds to 127.0.0.1:3000 and starts listening
//!
//! ## Relation to Entire Program
//! - **Central Hub**: All HTTP requests flow through this server
//! - **Connects**: Frontend → This Server → Database (PostgreSQL)
//! - **Orchestrates**: Booking creation, contact forms
//! - **Future**: Will handle profiles, job matching, payments, reviews
//!
//! ## Key Routes
//! - `POST /api/booking` - Create service booking
//! - `POST /api/contact` - Contact form submission
//!
//! ## Middleware Order (Reverse execution)
//! 1. Response mapping (mw_response_map) - Transforms responses
//! 2. Logging and tracing
//! 4. CORS - Validates cross-origin requests from frontend

mod db;              // Database connection pool management (src/db/)
mod models;          // Data models: Customer, Booking, Contact (src/models/)
mod repositories;    // Database CRUD operations (src/repositories/)
mod handlers;        // HTTP request handlers (src/handlers/)
mod error;           // Custom error types and error handling (src/error.rs)
mod config;          // Configuration from environment variables (src/config.rs)
mod middleware;      // HTTP middleware for response mapping (src/middleware/)

use axum::{
    middleware as axum_middleware,
    routing::{get, post},
    Router,
    http::Method,
};
use tower_http::cors::CorsLayer;
use tower_cookies::CookieManagerLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::net::SocketAddr;

use db::create_pool;
use handlers::{root, health_check, handle_contact, handle_booking};
use middleware::mw_response_map;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    // Initialize tracing with JSON logs for production, pretty for dev
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "backend=debug,tower_http=debug,axum=trace,lib_core=debug".into());

    let registry = tracing_subscriber::registry().with(filter);

    if cfg!(debug_assertions) {
        // Local usage: Pretty print
        registry.with(tracing_subscriber::fmt::layer().pretty()).init();
    } else {
        // Production usage: JSON logs
        registry.with(tracing_subscriber::fmt::layer().json()).init();
    }

    // Load environment variables
    dotenvy::dotenv().ok();
    
    // Get config to validate environment
    let _ = config::config();

    // Create database connection pool
    let database_url = &config::config().DATABASE_URL;
    tracing::info!("Creating database connection pool...");
    let pool = create_pool(database_url).await?;
    tracing::info!("Database connection pool created successfully");


    // Build our application with routes
    let app = Router::new()
        .route("/", get(root))
        .route("/api/health", get(health_check))
        // Public API routes
        .route("/api/contact", post(handle_contact))
        .route("/api/booking", post(handle_booking))
        // Middleware layers (order matters - added in reverse order of execution)
        .layer(axum_middleware::map_response(mw_response_map))
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(tower_http::trace::DefaultMakeSpan::new().include_headers(true))
                .on_request(tower_http::trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        .layer(tower_http::request_id::SetRequestIdLayer::x_request_id(tower_http::request_id::MakeRequestUuid))
        .layer(
            CorsLayer::new()
                .allow_origin("http://127.0.0.1:8080".parse::<axum::http::HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                ])
                .allow_credentials(true),
        )
        // Share database pool with all handlers
        .with_state(pool);

    // Run the server
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr_str = format!("127.0.0.1:{}", port);
    let addr: SocketAddr = addr_str.parse().expect("Invalid address format");

    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
