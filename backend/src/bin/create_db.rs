//! # Database Creation Script
//!
//! ## Purpose
//! Create the 'handyman' PostgreSQL database.
//! Drops existing database if it exists and creates fresh one.
//!
//! ## Usage
//! ```bash
//! cargo run --bin create_db
//! ```
//!
//! ## What It Does
//! 1. Connects to PostgreSQL 'postgres' database (default)
//! 2. Drops 'handyman' database if exists
//! 3. Creates fresh 'handyman' database
//!
//! ## Setup Sequence
//! 1. Run this script first: `cargo run --bin create_db`
//! 2. Then run init_db: `cargo run --bin init_db`
//! 3. Finally start backend: `cargo run --bin backend`
//!
//! ## Relation to Entire Program
//! - **First Step**: Creates database before schema initialization
//! - **Development Tool**: Useful for resetting database during development
//! - **WARNING**: Destroys all existing data!

use tokio_postgres::{NoTls, Error};  // PostgreSQL async client

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Connect to 'postgres' database (not 'handyman')
    // This allows us to create/drop the 'handyman' database
    let postgres_url = "postgresql://postgres:Ab13cba46def79_@localhost:5432/postgres";

    println!("Connecting to PostgreSQL...");
    let (client, connection) = tokio_postgres::connect(&postgres_url, NoTls).await?;

    // Spawn connection handler
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    println!("Dropping existing database if exists...");
    // Drop existing database (ignores error if doesn't exist)
    let _ = client.execute("DROP DATABASE IF EXISTS handyman", &[]).await;

    println!("Creating database 'handyman'...");
    // Create fresh database
    client.execute("CREATE DATABASE handyman", &[]).await?;

    println!("[OK] Database 'handyman' created successfully!");

    Ok(())
}
