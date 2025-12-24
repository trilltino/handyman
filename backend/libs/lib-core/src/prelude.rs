//! # lib-core Prelude
//!
//! Common imports for working with the core library.
//!
//! ## Usage
//!
//! ```rust
//! use lib_core::prelude::*;
//! ```
//!
//! This imports the most commonly used types and functions.

pub use crate::model::booking::{Booking, BookingBmc, BookingForCreate};
pub use crate::model::contact::{Contact, ContactBmc, ContactForCreate};
pub use crate::model::customer::{Customer, CustomerBmc, CustomerForCreate};
pub use crate::model::quote::{Quote, QuoteBmc, QuoteForCreate, QuoteItem};
pub use crate::model::ModelManager;
pub use crate::model::{Error as ModelError, Result as ModelResult};
pub use crate::Ctx;
