//! Reusable UI components.
//!
//! Contains primitive UI elements used throughout the application:
//! - `Button`: Styled navigation links
//! - `Badge`: Status indicators
//! - `Card`: Content containers
//! - `FeatureList`: Check-marked feature lists
//! - `Section`: Page layout sections

pub mod badge;
pub mod button;
pub mod card;
pub mod feature_list;
pub mod section;

pub use button::{Button, ButtonVariant};
pub use feature_list::FeatureList;
