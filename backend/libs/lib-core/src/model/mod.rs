//! Model Layer - Backend Model Controllers (BMCs)
//!
//! This module provides the data access layer for the application.

// region:    --- Modules

mod base;
pub mod contact;
mod error;
mod store;

pub use self::error::{Error, Result};

use crate::model::store::dbx::Dbx;
use crate::model::store::new_db_pool;

// endregion: --- Modules

// region:    --- ModelManager

/// Central resource manager for the application
#[derive(Clone)]
pub struct ModelManager {
    dbx: Dbx,
}

impl ModelManager {
    /// Create a new ModelManager with database connection
    pub async fn new() -> Result<Self> {
        let db_pool = new_db_pool()
            .await
            .map_err(|ex| Error::CantCreateModelManagerProvider(ex.to_string()))?;
        let dbx = Dbx::new(db_pool, false)?;
        Ok(ModelManager { dbx })
    }

    /// Create a new ModelManager with transaction support
    pub fn new_with_txn(&self) -> Result<ModelManager> {
        let dbx = Dbx::new(self.dbx.db().clone(), true)?;
        Ok(ModelManager { dbx })
    }

    /// Get the database executor
    pub fn dbx(&self) -> &Dbx {
        &self.dbx
    }
}

// endregion: --- ModelManager
