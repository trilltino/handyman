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
//! - **Connects**: Frontend (Yew) → This Server → Database (PostgreSQL)
//! - **Orchestrates**: Authentication, booking creation, user management, contact forms
//! - **Future**: Will handle handyman profiles, job matching, payments, reviews
//!
//! ## Key Routes
//! - `POST /api/register` - User registration
//! - `POST /api/login` - User authentication
//! - `POST /api/booking` - Create service booking
//! - `POST /api/contact` - Contact form submission
//! - `GET /api/me` - Get current user (protected)
//!
//! ## Middleware Order (Reverse execution)
//! 1. Response mapping (mw_response_map) - Transforms responses
//! 2. Context resolver (mw_ctx_resolver) - Extracts user context from auth token
//! 3. Cookie manager - Handles HTTP cookies for sessions
//! 4. CORS - Validates cross-origin requests from frontend

mod db;              // Database connection pool management (src/db/)
mod models;          // Data models: User, Customer, Booking, Contact (src/models/)
mod repositories;    // Database CRUD operations (src/repositories/)
mod handlers;        // HTTP request handlers (src/handlers/)
mod error;           // Custom error types and error handling (src/error.rs)
mod config;          // Configuration from environment variables (src/config.rs)
mod ctx;             // Request context with user authentication info (src/ctx.rs)
mod token;           // JWT token generation and validation (src/token.rs)
mod middleware;      // HTTP middleware for auth and response mapping (src/middleware/)

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
use handlers::{root, health_check, handle_contact, handle_booking, handle_register, handle_login, handle_logout, handle_me};
use middleware::{mw_ctx_resolver, mw_ctx_require, mw_response_map};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();
    
    // Get config to validate environment
    let _ = config::config();

    // Create database connection pool
    let database_url = &config::config().DATABASE_URL;
    tracing::info!("Creating database connection pool...");
    let pool = create_pool(database_url).await?;
    tracing::info!("Database connection pool created successfully");

    // Define routes that require authentication
    let routes_protected = Router::new()
        .route("/api/me", get(handle_me))
        .route_layer(axum_middleware::from_fn(mw_ctx_require));

    // Build our application with routes
    let app = Router::new()
        .route("/", get(root))
        .route("/api/health", get(health_check))
        // Auth routes (public)
        .route("/api/register", post(handle_register))
        .route("/api/login", post(handle_login))
        .route("/api/logout", post(handle_logout))
        // Public API routes
        .route("/api/contact", post(handle_contact))
        .route("/api/booking", post(handle_booking))
        // Protected routes
        .merge(routes_protected)
        // Middleware layers (order matters - added in reverse order of execution)
        .layer(axum_middleware::map_response(mw_response_map))
        .layer(axum_middleware::from_fn_with_state(pool.clone(), mw_ctx_resolver))
        .layer(CookieManagerLayer::new())
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
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
