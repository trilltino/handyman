//! Reusable UI components.
//!
//! Contains primitive UI elements used throughout the application:
//! - `Button`: Styled navigation links
//! - `Badge`: Status indicators with color variants
//! - `Card`: Content containers (FaqCard, FeatureCard, StatCard)
//! - `FeatureList`: Check-marked feature lists
//! - `Section`: Page layout sections (HeroSection, CtaSection)

pub mod badge;
pub mod button;
pub mod card;
pub mod feature_list;
pub mod section;

// Button exports
pub use button::{Button, ButtonVariant};

// Badge exports
pub use badge::{Badge, BadgeVariant};

// Card exports
pub use card::{FaqCard, StatCard};

// Section exports
pub use section::Section;

// Feature list exports
pub use feature_list::FeatureList;
