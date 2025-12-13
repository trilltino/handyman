//! Core Business Logic for Handyman Marketplace
//!
//! This library contains the core domain models, business logic,
//! and data access layer for the handyman marketplace application.
//!
//! # Architecture
//!
//! The core library is organized into several key modules:
//!
//! - **[`model`]** - Data access layer with ModelManager and BMC pattern
//! - **[`ctx`]** - Request context for authentication and authorization
//! - **[`email`]** - Email service for notifications
//! - **[`config`]** - Configuration management
//!
//! ## Design Principles
//!
//! - **Model Layer Normalization**: All data access goes through the Model layer
//! - **ModelManager Pattern**: Central resource manager holding db_pool and other resources
//! - **BMC Pattern**: Backend Model Controllers implement CRUD for each entity
//! - **Transaction Support**: Built-in database transaction support
//! - **Type Safety**: Leverage Rust's type system for correctness
//!
//! # Example
//!
//! ```rust,no_run
//! use lib_core::model::ModelManager;
//! use lib_core::model::contact::{ContactBmc, ContactForCreate};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize ModelManager with database connection
//!     let mm = ModelManager::new().await?;
//!     
//!     // Create a contact submission
//!     let contact = ContactForCreate {
//!         name: "John Doe".to_string(),
//!         email: "john@example.com".to_string(),
//!         message: "Hello!".to_string(),
//!         subject: Some("Question".to_string()),
//!         ip_address: None,
//!         user_agent: None,
//!     };
//!     
//!     // Use BMC to create record
//!     let id = ContactBmc::create(&mm, contact).await?;
//!     println!("Created contact with ID: {}", id);
//!     
//!     // Retrieve the contact
//!     let contact = ContactBmc::get(&mm, id).await?;
//!     println!("Contact: {:?}", contact);
//!     
//!     Ok(())
//! }
//! ```
//!
//! # Transaction Example
//!
//! ```rust,no_run
//! use lib_core::model::ModelManager;
//! use lib_core::model::contact::{ContactBmc, ContactForCreate};
//!
//! async fn create_with_transaction(mm: &ModelManager) -> Result<i64, Box<dyn std::error::Error>> {
//!     // Create transactional ModelManager
//!     let mm_txn = mm.new_with_txn()?;
//!     
//!     // Perform operations within transaction
//!     let contact = ContactForCreate { /* ... */ };
//!     let id = ContactBmc::create(&mm_txn, contact).await?;
//!     
//!     // Commit transaction
//!     mm_txn.dbx().commit().await?;
//!     
//!     Ok(id)
//! }
//! ```
//!
//! # Integration with Frameworks
//!
//! ## Axum
//!
//! ```rust,no_run
//! use axum::{Router, Extension};
//! use lib_core::model::ModelManager;
//!
//! async fn setup_axum() -> Router {
//!     let mm = ModelManager::new().await.expect("Failed to create ModelManager");
//!     
//!     Router::new()
//!         // Add routes...
//!         .layer(Extension(mm))  // Share ModelManager across handlers
//! }
//! ```
//!
//! ## Leptos
//!
//! ```rust,no_run
//! use leptos::prelude::*;
//! use lib_core::model::ModelManager;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     // Provide ModelManager to Leptos context
//!     let mm = expect_context::<ModelManager>();
//!     
//!     view! {
//!         // Use mm in server functions
//!     }
//! }
//! ```

// region:    --- Modules

pub mod config;
pub mod ctx;
pub mod email;
pub mod model;

// Re-export commonly used types for convenience
pub use config::core_config;
pub use ctx::Ctx;
pub use model::ModelManager;

// Development utilities (commented in production)
// #[cfg(test)]
pub mod _dev_utils;

// endregion: --- Modules
