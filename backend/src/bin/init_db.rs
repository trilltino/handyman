//! # Database Initialization Script
//!
//! ## Purpose
//! Initialize the handyman database schema by running schema.sql.
//! Creates all tables, indexes, and constraints.
//!
//! ## Usage
//! ```bash
//! cargo run --bin init_db
//! ```
//!
//! ## What It Does
//! 1. Loads DATABASE_URL from .env file
//! 2. Connects to PostgreSQL database
//! 3. Executes schema.sql (creates tables)
//! 4. Reports success
//!
//! ## Tables Created
//! - users: User accounts with authentication
//! - customers: Customer contact information
//! - bookings: Service booking requests
//! - contact_messages: Contact form submissions
//!
//! ## Relation to Entire Program
//! - **Run Once**: During initial setup or after database reset
//! - **Creates Schema**: Required before running backend server
//! - **Idempotent**: Safe to run multiple times (DROP IF EXISTS)

use tokio_postgres::{NoTls, Error};  // PostgreSQL async client

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get DATABASE_URL or use default
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/handyman".to_string());

    println!("Connecting to database...");
    // Connect to database
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    // Spawn connection handler
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    println!("Running schema...");
    // Load schema.sql from compile-time include
    let schema = include_str!("../../schema.sql");

    // Execute all SQL statements in schema
    client.batch_execute(schema).await?;

    println!("[OK] Database initialized successfully!");
    println!("Tables created:");
    println!("  - users");
    println!("  - customers");
    println!("  - bookings");
    println!("  - contact_messages");

    Ok(())
}
