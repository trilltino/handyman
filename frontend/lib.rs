//! # Frontend Library Module
//!
//! ## Purpose
//! Module organization and re-exports for the Yew frontend application.
//! Defines application structure and public API.
//!
//! ## Module Structure
//! - **app**: Root App component with router
//! - **routes**: Route definitions (/home, /booking, /contact, etc.)
//! - **pages**: Page components (Home, Booking, Contact, etc.)
//! - **components**: Reusable UI components (Nav, etc.)
//! - **contexts**: React-style contexts (AuthProvider)
//! - **api**: Backend API client functions
//! - **stripe**: Stripe payment integration
//!
//! ## Relation to Entire Program
//! - **WASM Target**: Compiles to WebAssembly for browser
//! - **Single Page App**: Client-side routing with yew-router
//! - **Communicates with**: Backend API at localhost:3000

// Module declarations
mod app;         // Root App component with router setup
mod components;  // Reusable UI components (Nav, etc.)
mod pages;       // Page components (Home, Booking, Contact, etc.)
mod routes;      // Route enum and paths
mod api;         // Backend API client
mod contexts;    // Context providers (Auth, etc.)
pub mod stripe;  // Stripe payment integration (public for pages)

// Re-exports for clean imports
pub use app::*;       // Export App component
pub use routes::Route; // Export Route enum
