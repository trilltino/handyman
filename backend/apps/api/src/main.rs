//! # Handyman Marketplace HTTP API Server
//!
//! Main entry point for the Axum-based REST API serving the frontend and external clients.
//!
//! ## Startup Sequence
//!
//! 1. Load environment variables from `.env` file
//! 2. Initialize structured logging with `tracing`
//! 3. Load application configuration from environment or defaults
//! 4. Initialize database connection pool via `ModelManager`
//! 5. Build Axum router with all middleware and routes
//! 6. Bind to configured address and listen for requests
//!
//! ## Command-Line Arguments
//!
//! - `--test-db` - Test database connection and exit
//!   ```bash
//!   cargo run -p api -- --test-db
//!   ```
//!
//! ## Environment Variables
//!
//! - `DATABASE_URL` - PostgreSQL connection string
//! - `APP_SERVER__HOST` - Server bind address (default: 127.0.0.1)
//! - `APP_SERVER__PORT` - Server port (default: 8080)
//! - `RUST_LOG` - Log level: debug, info, warn, error
//! - `SMTP_*` - Email configuration
//!
//! ## Development
//!
//! ```bash
//! # Start with debug logging
//! RUST_LOG=debug cargo run -p api
//!
//! # Watch mode
//! cargo watch -x "run -p api"
//! ```
//!
//! ## Production
//!
//! Ensure all required environment variables are set:
//! - DATABASE_URL
//! - SMTP configuration for email
//! - RUST_LOG=info or warn

mod config;
mod middleware;
mod web;

use crate::config::app_config;
use clap::Parser;
use lib_core::model::ModelManager;
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// Handyman Marketplace API Server
#[derive(Parser)]
#[command(name = "api")]
#[command(about = "Handyman Marketplace API Server", long_about = None)]
struct Args {
    /// Run database migrations and exit
    #[arg(long)]
    migrate: bool,

    /// Test database connection and exit
    #[arg(long)]
    test_db: bool,
}

/// Main application entry point.
///
/// Performs complete server initialization including:
/// - Configuration loading
/// - Database connection pool setup
/// - Route registration
/// - Middleware composition
/// - HTTP server binding
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    // Initialize tracing with JSON logs for production, pretty for dev
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "api=debug,tower_http=debug,axum=trace,lib_core=debug".into());

    let registry = tracing_subscriber::registry().with(filter);

    if cfg!(debug_assertions) {
        registry
            .with(tracing_subscriber::fmt::layer().pretty())
            .init();
    } else {
        registry
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    }

    let args = Args::parse();

    // Handle --test-db flag
    if args.test_db {
        return test_database_connection().await;
    }

    // Handle --migrate flag
    if args.migrate {
        return run_migrations().await;
    }

    let config = app_config();
    println!(
        "Server config: {}:{}",
        config.server.host, config.server.port
    );

    // Check Email Configuration
    if std::env::var("SMTP_USERNAME").is_err() || std::env::var("SMTP_PASSWORD").is_err() {
        tracing::warn!("SMTP_USERNAME or SMTP_PASSWORD not set. Emails will NOT be sent.");
    } else {
        tracing::info!("SMTP Configuration detected.");
    }

    let mm = match ModelManager::new().await {
        Ok(mm) => {
            tracing::info!("Database connection successful");
            mm
        }
        Err(e) => {
            tracing::error!("Database connection failed: {}", e);
            tracing::warn!("Starting server without database - some features will not work");
            return Err(e.into());
        }
    };

    let mut app = middleware::apply_middleware(web::routes(mm));

    // Add Request ID and Tracing Middleware
    app = app
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(tower_http::trace::DefaultMakeSpan::new().include_headers(true))
                .on_request(tower_http::trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(
                    tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO),
                ),
        )
        .layer(tower_http::request_id::SetRequestIdLayer::x_request_id(
            tower_http::request_id::MakeRequestUuid,
        ));

    let addr = format!("{}:{}", config.server.host, config.server.port).parse::<SocketAddr>()?;

    tracing::info!("listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Test database connection and exit.
///
/// Used with `--test-db` command-line argument to verify database
/// configuration without starting the server.
///
/// # Exit Codes
///
/// - 0 - Connection successful
/// - 1 - Connection failed
async fn test_database_connection() -> anyhow::Result<()> {
    println!("Testing database connection...");
    match ModelManager::new().await {
        Ok(_) => {
            println!("Database connection successful!");
            Ok(())
        }
        Err(e) => {
            eprintln!("Database connection failed: {}", e);
            std::process::exit(1);
        }
    }
}

/// Run database migrations and exit.
///
/// Used with `--migrate` command-line argument to run pending
/// database migrations before starting the server.
///
/// # Exit Codes
///
/// - 0 - Migrations successful
/// - 1 - Migrations failed
async fn run_migrations() -> anyhow::Result<()> {
    tracing::info!("Running database migrations...");

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to database
    let pool = sqlx::PgPool::connect(&database_url).await?;

    // Run migrations from the migrations folder
    sqlx::migrate!("../../../migrations").run(&pool).await?;

    tracing::info!("Migrations completed successfully");
    Ok(())
}
