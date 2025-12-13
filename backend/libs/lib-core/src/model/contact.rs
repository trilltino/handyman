//! # Contact Model
//!
//! This module defines the contact form data model and database operations.
//!
//! ## Structures
//!
//! - [`Contact`] - Complete contact record from database
//! - [`ContactForCreate`] - Data required to create a new contact
//! - [`ContactBmc`] - Business Model Controller for contact operations
//!
//! ## Usage
//!
//! ```rust
//! use lib_core::model::contact::{ContactBmc, ContactForCreate};
//! use lib_core::model::ModelManager;
//!
//! async fn save_contact(mm: &ModelManager) {
//!     let contact = ContactForCreate {
//!         name: "John Doe".to_string(),
//!         email: "john@example.com".to_string(),
//!         subject: Some("Question".to_string()),
//!         message: "Hello!".to_string(),
//!         ip_address: None,
//!         user_agent: None,
//!     };
//!     let id = ContactBmc::create(mm, contact).await.unwrap();
//! }
//! ```

use crate::model::ModelManager;
use crate::model::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::PrimitiveDateTime;
use utoipa::ToSchema;

/// Complete contact record from the database.
///
/// This struct represents a full contact submission including
/// auto-generated fields like `id` and `submitted_at`.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Contact {
    /// Auto-generated primary key
    pub id: i32,
    /// Sender's name
    pub name: String,
    /// Sender's email address
    pub email: String,
    /// Optional message subject
    pub subject: Option<String>,
    /// Message content
    pub message: String,
    /// Timestamp when submitted (auto-generated)
    pub submitted_at: Option<PrimitiveDateTime>,
    /// IP address of the submitter (optional)
    pub ip_address: Option<String>,
    /// User agent string (optional)
    pub user_agent: Option<String>,
}

/// Data required to create a new contact submission.
///
/// This struct is used for API requests and doesn't include
/// auto-generated fields like `id` or `submitted_at`.
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct ContactForCreate {
    /// Sender's name (required)
    pub name: String,
    /// Sender's email address (required)
    pub email: String,
    /// Optional message subject
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Message content (required)
    pub message: String,
    /// Optional IP address of submitter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// Optional user agent string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

/// Business Model Controller for contact operations.
///
/// Provides database operations for contact form submissions.
pub struct ContactBmc;

impl ContactBmc {
    /// Creates a new contact submission in the database.
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    /// * `contact` - Contact data to insert
    ///
    /// # Returns
    ///
    /// The auto-generated ID of the new contact record.
    ///
    /// # Errors
    ///
    /// Returns an error if the database insert fails.
    pub async fn create(mm: &ModelManager, contact: ContactForCreate) -> Result<i32> {
        let row: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO contact_submissions (name, email, subject, message, ip_address, user_agent)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#,
        )
        .bind(contact.name)
        .bind(contact.email)
        .bind(contact.subject)
        .bind(contact.message)
        .bind(contact.ip_address)
        .bind(contact.user_agent)
        .fetch_one(mm.dbx().db())
        .await?;

        Ok(row.0)
    }

    /// Gets a contact submission by ID.
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    /// * `id` - Contact ID to retrieve
    ///
    /// # Returns
    ///
    /// The contact record if found.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails or contact not found.
    pub async fn get(mm: &ModelManager, id: i32) -> Result<Contact> {
        let contact = sqlx::query_as::<_, Contact>(
            r#"
            SELECT id, name, email, subject, message, submitted_at, ip_address, user_agent
            FROM contact_submissions
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(mm.dbx().db())
        .await?;

        Ok(contact)
    }

    /// Lists all contact submissions from the database.
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    ///
    /// # Returns
    ///
    /// A vector of all contact submissions, ordered by submission time (newest first).
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    pub async fn list(mm: &ModelManager) -> Result<Vec<Contact>> {
        let contacts = sqlx::query_as(
            r#"
            SELECT id, name, email, subject, message, submitted_at, ip_address, user_agent
            FROM contact_submissions
            ORDER BY submitted_at DESC
            "#,
        )
        .fetch_all(mm.dbx().db())
        .await?;

        Ok(contacts)
    }

    /// Deletes a contact submission from the database.
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    /// * `id` - Contact ID to delete
    ///
    /// # Returns
    ///
    /// Unit type on success.
    ///
    /// # Errors
    ///
    /// Returns an error if the database delete fails or contact not found.
    pub async fn delete(mm: &ModelManager, id: i32) -> Result<()> {
        let rows_affected = sqlx::query("DELETE FROM contact_submissions WHERE id = $1")
            .bind(id)
            .execute(mm.dbx().db())
            .await?
            .rows_affected();

        if rows_affected == 0 {
            return Err(crate::model::Error::EntityNotFound {
                entity: "Contact",
                id: id as i64,
            });
        }

        Ok(())
    }
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;

    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;
        let fx_name = "test_create_ok contact 01";
        let fx_email = "test_create@example.com";

        // Execute
        let contact = ContactForCreate {
            name: fx_name.to_string(),
            email: fx_email.to_string(),
            message: "Test message for create".to_string(),
            subject: Some("Test Subject".to_string()),
            ip_address: Some("127.0.0.1".to_string()),
            user_agent: Some("test-agent".to_string()),
        };

        let id = ContactBmc::create(&mm, contact).await?;

        // Check
        assert!(id > 0, "Should return valid ID");

        // Cleanup
        ContactBmc::delete(&mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_get_ok() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;
        let fx_name = "test_get_ok contact 01";
        let fx_email = "test_get@example.com";

        // Create test contact
        let contact = ContactForCreate {
            name: fx_name.to_string(),
            email: fx_email.to_string(),
            message: "Test message for get".to_string(),
            subject: Some("Get Test".to_string()),
            ip_address: None,
            user_agent: None,
        };
        let id = ContactBmc::create(&mm, contact).await?;

        // Execute
        let contact = ContactBmc::get(&mm, id).await?;

        // Check
        assert_eq!(contact.name, fx_name);
        assert_eq!(contact.email, fx_email);
        assert_eq!(contact.subject, Some("Get Test".to_string()));

        // Cleanup
        ContactBmc::delete(&mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;
        let fx_id = 999999;

        // Execute
        let res = ContactBmc::get(&mm, fx_id).await;

        // Check
        assert!(res.is_err(), "Should return error for non-existent contact");

        Ok(())
    }

    #[tokio::test]
    async fn test_list_ok() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test contacts
        let contact1 = ContactForCreate {
            name: "test_list contact 01".to_string(),
            email: "test_list_01@example.com".to_string(),
            message: "First test message".to_string(),
            subject: None,
            ip_address: None,
            user_agent: None,
        };
        let contact2 = ContactForCreate {
            name: "test_list contact 02".to_string(),
            email: "test_list_02@example.com".to_string(),
            message: "Second test message".to_string(),
            subject: None,
            ip_address: None,
            user_agent: None,
        };

        let id1 = ContactBmc::create(&mm, contact1).await?;
        let id2 = ContactBmc::create(&mm, contact2).await?;

        // Execute
        let contacts = ContactBmc::list(&mm).await?;

        // Check
        assert!(contacts.len() >= 2, "Should have at least 2 contacts");

        // Cleanup
        ContactBmc::delete(&mm, id1).await?;
        ContactBmc::delete(&mm, id2).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_ok() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test contact
        let contact = ContactForCreate {
            name: "test_delete contact 01".to_string(),
            email: "test_delete@example.com".to_string(),
            message: "Test message for delete".to_string(),
            subject: None,
            ip_address: None,
            user_agent: None,
        };
        let id = ContactBmc::create(&mm, contact).await?;

        // Execute
        ContactBmc::delete(&mm, id).await?;

        // Check - should not be able to get deleted contact
        let res = ContactBmc::get(&mm, id).await;
        assert!(res.is_err(), "Should not find deleted contact");

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_err_not_found() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;
        let fx_id = 999999;

        // Execute
        let res = ContactBmc::delete(&mm, fx_id).await;

        // Check
        assert!(
            res.is_err(),
            "Should return error when deleting non-existent contact"
        );

        Ok(())
    }
}

// endregion: --- Tests
