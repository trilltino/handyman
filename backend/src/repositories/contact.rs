//! # Contact Repository
//!
//! ## Purpose
//! Database access layer for ContactMessage entity.
//! Handles contact form submissions and queries.
//!
//! ## Operations
//! - **create**: Insert new contact message with "new" status
//! - **list_recent**: Fetch recent contact messages (for admin dashboard)
//!
//! ## Relation to Entire Program
//! - **Called By**: Contact form handler (POST /api/contact)
//! - **Database**: Queries `contact_messages` table
//! - **Future**: Will integrate with email notifications, CRM system

use crate::models::{ContactMessage, NewContactMessage};  // Contact message models
use anyhow::{Context, Result};                           // Error handling with context
use bb8_postgres::tokio_postgres::Row;                   // Database row type
use tokio_postgres::Client;                              // PostgreSQL async client

/// Contact repository - handles all database operations for contact_messages table
pub struct ContactRepository;

impl ContactRepository {
    /// Create a new contact message from form submission
    /// Called by: POST /api/contact handler
    ///
    /// # Initial State
    /// - status: "new" (unread message awaiting response)
    ///
    /// # Future Enhancements
    /// - Send email notification to admin
    /// - Auto-responder email to sender
    pub async fn create(client: &Client, new_message: &NewContactMessage) -> Result<ContactMessage> {
        let row = client
            .query_one(
                "INSERT INTO contact_messages (name, email, message, status)
                 VALUES ($1, $2, $3, 'new')
                 RETURNING id, name, email, message, status, created_at",
                &[&new_message.name, &new_message.email, &new_message.message],
            )
            .await
            .context("Failed to insert contact message")?;

        Ok(Self::row_to_contact_message(row))
    }

    /// List recent contact messages ordered by creation date
    /// Used for: Admin dashboard to view and respond to inquiries
    ///
    /// # Arguments
    /// - limit: Maximum number of messages to return
    pub async fn list_recent(client: &Client, limit: i64) -> Result<Vec<ContactMessage>> {
        let rows = client
            .query(
                "SELECT id, name, email, message, status, created_at
                 FROM contact_messages
                 ORDER BY created_at DESC
                 LIMIT $1",
                &[&limit],
            )
            .await
            .context("Failed to list contact messages")?;

        Ok(rows.into_iter().map(Self::row_to_contact_message).collect())
    }

    /// Helper: Convert database row to ContactMessage struct
    /// Maps column indices to ContactMessage fields
    fn row_to_contact_message(row: Row) -> ContactMessage {
        ContactMessage {
            id: row.get(0),           // Column 0: id
            name: row.get(1),         // Column 1: name
            email: row.get(2),        // Column 2: email
            message: row.get(3),      // Column 3: message
            status: row.get(4),       // Column 4: status
            created_at: row.get(5),   // Column 5: created_at
        }
    }
}
