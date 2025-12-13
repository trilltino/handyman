//! Base traits and types for Model Controllers
//!
//! Provides common CRUD operations and traits that all BMCs can implement.
//!
//! # Purpose
//!
//! This module defines the foundational traits and types used across all
//! Backend Model Controllers (BMCs). It promotes code reuse and consistency.
//!
//! # Example
//!
//! ```rust,no_run
//! use lib_core::model::base::DbBmc;
//!
//! pub struct MyEntityBmc;
//!
//! impl DbBmc for MyEntityBmc {
//!     const TABLE: &'static str = "my_entities";
//! }
//! ```

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region:    --- Traits

/// Marker trait for Backend Model Controllers
///
/// Implement this trait for all BMC structs to define their database table.
///
/// # Example
///
/// ```rust
/// use lib_core::model::base::DbBmc;
///
/// pub struct ContactBmc;
///
/// impl DbBmc for ContactBmc {
///     const TABLE: &'static str = "contact_submissions";
/// }
/// ```
#[allow(dead_code)]
pub trait DbBmc {
    /// The name of the database table this BMC operates on
    const TABLE: &'static str;
}

// endregion: --- Traits

// region:    --- Common Field Types

/// Common timestamp fields for entities
///
/// Use this for entities that track creation and modification times.
///
/// # Example
///
/// ```rust
/// use lib_core::model::base::TimestampFields;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize)]
/// pub struct MyEntity {
///     pub id: i64,
///     pub name: String,
///     #[serde(flatten)]
///     pub timestamps: TimestampFields,
/// }
/// ```
#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct TimestampFields {
    /// When the entity was created
    pub created_at: time::OffsetDateTime,
    /// When the entity was last updated
    pub updated_at: time::OffsetDateTime,
}

/// Common audit fields for entities with creator/modifier tracking
///
/// Use this for entities that need to track who created and modified them.
///
/// # Example
///
/// ```rust
/// use lib_core::model::base::AuditFields;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize)]
/// pub struct MyEntity {
///     pub id: i64,
///     pub name: String,
///     #[serde(flatten)]
///     pub audit: AuditFields,
/// }
/// ```
#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct AuditFields {
    /// ID of the user who created this entity
    pub cid: i64,
    /// When the entity was created
    pub ctime: time::OffsetDateTime,
    /// ID of the user who last modified this entity
    pub mid: i64,
    /// When the entity was last modified
    pub mtime: time::OffsetDateTime,
}

// endregion: --- Common Field Types
