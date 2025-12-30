//! Application pages.
//!
//! Contains all route page components organized into subfolders.

pub mod blog;
pub mod examples;
pub mod info;
pub mod legal;
pub mod locations;
pub mod main;

// Re-export all page components for convenient access
pub use info::*;
pub use legal::*;
pub use locations::*;
pub use main::*;
