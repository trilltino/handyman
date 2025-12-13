//! Request context for authentication.
//!
//! Contains user authentication information for request processing.

use serde::{Deserialize, Serialize};

/// Request context with authenticated user information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use = "Context should be used for authorization"]
pub struct Ctx {
    user_id: i64,
}

impl Ctx {
    pub fn new(user_id: i64) -> Self {
        Self { user_id }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}
