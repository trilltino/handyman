//! Development and testing utilities
//!
//! Provides helper functions for testing and development environments.
//!
//! # Purpose
//!
//! This module contains utilities that are useful during development and testing,
//! such as test database initialization, fixture creation, and cleanup helpers.
//!
//! # Example
//!
//! ```rust,no_run
//! use lib_core::_dev_utils;
//!
//! #[tokio::test]
//! async fn test_something() -> Result<()> {
//!     let mm = _dev_utils::init_test().await;
//!     // Use mm for testing...
//!     Ok(())
//! }
//! ```

use crate::model::ModelManager;

/// Initialize test environment
///
/// Creates a ModelManager connected to the test database.
/// Loads environment variables from `.env` file.
///
/// # Panics
///
/// Panics if ModelManager creation fails. This is acceptable in tests.
///
/// # Example
///
/// ```rust,no_run
/// use lib_core::_dev_utils;
///
/// #[tokio::test]
/// async fn my_test() {
///     let mm = _dev_utils::init_test().await;
/// }
/// ```
pub async fn init_test() -> ModelManager {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize logging for tests (optional, helps with debugging)
    let _ = env_logger::builder().is_test(true).try_init();

    // Create ModelManager
    ModelManager::new()
        .await
        .expect("Failed to create test ModelManager - check DATABASE_URL")
}

/// Clean up test data
///
/// Helper function to clean up test records from the database.
/// Call this in test cleanup to avoid polluting the test database.
///
/// # Example
///
/// ```rust,no_run
/// use lib_core::_dev_utils;
/// use lib_core::model::contact::ContactBmc;
///
/// #[tokio::test]
/// async fn test_with_cleanup() {
///     let mm = _dev_utils::init_test().await;
///     
///     // Create test data
///     let id = ContactBmc::create(&mm, /* ... */).await.unwrap();
///     
///     // Test operations...
///     
///     // Cleanup
///     ContactBmc::delete(&mm, id).await.unwrap();
/// }
/// ```
pub async fn cleanup_test(_mm: &ModelManager) {
    // Placeholder for cleanup logic
    // Can be extended to clean up specific test data
}
