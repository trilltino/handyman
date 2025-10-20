//! # Contexts Module
//!
//! ## Purpose
//! React-style context providers for global state management in Yew.
//! Currently provides authentication state across the application.
//!
//! ## Contexts
//! - **AuthContext**: User authentication state (logged in/out, user info)
//!
//! ## How It Works
//! 1. Provider wraps App component in app.rs
//! 2. Child components use use_context hook to access state
//! 3. Components can dispatch actions to update state
//! 4. All subscribed components re-render on state change
//!
//! ## Relation to Entire Program
//! - **Used By**: All components needing auth state (Nav, Login, etc.)
//! - **Pattern**: Similar to React Context API

pub mod auth;  // Authentication context

// Re-export auth context types
pub use auth::{AuthContext, AuthProvider, AuthAction};
