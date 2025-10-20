//! # Components Module
//!
//! ## Purpose
//! Reusable UI components used across multiple pages.
//! Follows component-based architecture for code reusability.
//!
//! ## Components
//! - **Nav**: Navigation bar with responsive menu and auth state
//!
//! ## Relation to Entire Program
//! - **Used By**: App component (Nav), pages (future: forms, cards, modals)
//! - **Pattern**: Presentational components that receive props

pub mod nav;  // Navigation bar component

pub use nav::Nav;
