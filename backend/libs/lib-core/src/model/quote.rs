//! # Quote Model
//!
//! This module defines the quote data model and database operations.
//!
//! ## Structures
//!
//! - [`Quote`] - Complete quote record from database
//! - [`QuoteItem`] - Line item in a quote
//! - [`QuoteForCreate`] - Data required to create a new quote
//! - [`QuoteBmc`] - Business Model Controller for quote operations
//!
//! ## Example
//!
//! ```rust,no_run
//! use lib_core::model::quote::{QuoteBmc, QuoteForCreate, QuoteItem};
//! use lib_core::model::ModelManager;
//!
//! async fn create_quote(mm: &ModelManager) -> Result<i32, Box<dyn std::error::Error>> {
//!     let quote = QuoteForCreate {
//!         customer_id: Some(1),
//!         title: "Plumbing Repair".to_string(),
//!         items: vec![
//!             QuoteItem {
//!                 description: "Call-out fee".to_string(),
//!                 quantity: 1,
//!                 unit_price: 3000,
//!             },
//!         ],
//!         valid_days: Some(30),
//!     };
//!     let id = QuoteBmc::create(mm, quote).await?;
//!     Ok(id)
//! }
//! ```

use crate::model::ModelManager;
use crate::model::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use tracing::instrument;
use utoipa::ToSchema;

/// Line item in a quote.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct QuoteItem {
    /// Description of the item/service
    pub description: String,
    /// Quantity
    pub quantity: i32,
    /// Unit price in cents
    pub unit_price: i32,
}

/// Complete quote record from the database.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Quote {
    /// Auto-generated primary key
    pub id: i32,
    /// Customer ID (foreign key)
    pub customer_id: Option<i32>,
    /// Quote title
    pub title: String,
    /// JSON array of line items
    pub items: serde_json::Value,
    /// Subtotal in cents
    pub subtotal_cents: i32,
    /// Discount in cents
    pub discount_cents: i32,
    /// Total in cents
    pub total_cents: i32,
    /// Quote expiry date
    pub valid_until: Option<time::Date>,
    /// Current status: draft, sent, viewed, accepted, rejected, expired
    pub status: String,
    /// Customer notes when responding
    pub customer_notes: Option<String>,
    /// When the customer accepted
    pub accepted_at: Option<OffsetDateTime>,
    /// Linked booking ID when accepted
    pub booking_id: Option<i32>,
    /// When the quote was created
    pub created_at: Option<OffsetDateTime>,
    /// When the quote was last updated
    pub updated_at: Option<OffsetDateTime>,
}

/// Data required to create a new quote.
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct QuoteForCreate {
    /// Customer ID (optional)
    pub customer_id: Option<i32>,
    /// Quote title
    pub title: String,
    /// Line items
    pub items: Vec<QuoteItem>,
    /// Days until expiry (default 30)
    pub valid_days: Option<i32>,
}

/// Business Model Controller for quote operations.
pub struct QuoteBmc;

impl QuoteBmc {
    /// Creates a new quote in the database.
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    /// * `quote` - Quote data to insert
    ///
    /// # Returns
    ///
    /// The auto-generated ID of the new quote.
    #[must_use = "the returned ID should be used or logged"]
    #[instrument(skip(mm), fields(title = %quote.title))]
    pub async fn create(mm: &ModelManager, quote: QuoteForCreate) -> Result<i32> {
        // Calculate totals
        let subtotal: i32 = quote.items.iter().map(|i| i.quantity * i.unit_price).sum();
        let total = subtotal; // No discount for now

        // Serialize items to JSON
        let items_json = serde_json::to_value(&quote.items)
            .map_err(|e| crate::model::Error::Sqlx(sqlx::Error::Decode(Box::new(e))))?;

        // Calculate valid_until date
        let valid_days = quote.valid_days.unwrap_or(30);

        let row: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO quotes (customer_id, title, items, subtotal_cents, total_cents, valid_until, status)
            VALUES ($1, $2, $3, $4, $5, CURRENT_DATE + $6, 'draft')
            RETURNING id
            "#,
        )
        .bind(quote.customer_id)
        .bind(&quote.title)
        .bind(&items_json)
        .bind(subtotal)
        .bind(total)
        .bind(valid_days)
        .fetch_one(mm.dbx().db())
        .await?;

        Ok(row.0)
    }

    /// Gets a quote by ID.
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    /// * `id` - Quote ID to retrieve
    #[instrument(skip(mm))]
    pub async fn get(mm: &ModelManager, id: i32) -> Result<Quote> {
        let quote = sqlx::query_as::<_, Quote>(
            r#"
            SELECT id, customer_id, title, items, subtotal_cents, discount_cents, total_cents,
                   valid_until, status, customer_notes, accepted_at, booking_id,
                   created_at, updated_at
            FROM quotes
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(mm.dbx().db())
        .await?;

        Ok(quote)
    }

    /// Lists all quotes, ordered by creation date (newest first).
    #[instrument(skip(mm))]
    pub async fn list(mm: &ModelManager) -> Result<Vec<Quote>> {
        let quotes = sqlx::query_as(
            r#"
            SELECT id, customer_id, title, items, subtotal_cents, discount_cents, total_cents,
                   valid_until, status, customer_notes, accepted_at, booking_id,
                   created_at, updated_at
            FROM quotes
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(mm.dbx().db())
        .await?;

        Ok(quotes)
    }

    /// Lists quotes by status.
    #[instrument(skip(mm))]
    pub async fn list_by_status(mm: &ModelManager, status: &str) -> Result<Vec<Quote>> {
        let quotes = sqlx::query_as(
            r#"
            SELECT id, customer_id, title, items, subtotal_cents, discount_cents, total_cents,
                   valid_until, status, customer_notes, accepted_at, booking_id,
                   created_at, updated_at
            FROM quotes
            WHERE status = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(status)
        .fetch_all(mm.dbx().db())
        .await?;

        Ok(quotes)
    }

    /// Lists quotes for a specific customer.
    #[instrument(skip(mm))]
    pub async fn list_by_customer(mm: &ModelManager, customer_id: i32) -> Result<Vec<Quote>> {
        let quotes = sqlx::query_as(
            r#"
            SELECT id, customer_id, title, items, subtotal_cents, discount_cents, total_cents,
                   valid_until, status, customer_notes, accepted_at, booking_id,
                   created_at, updated_at
            FROM quotes
            WHERE customer_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(customer_id)
        .fetch_all(mm.dbx().db())
        .await?;

        Ok(quotes)
    }

    /// Updates a quote's status.
    #[instrument(skip(mm))]
    pub async fn update_status(mm: &ModelManager, id: i32, status: &str) -> Result<()> {
        let rows_affected = sqlx::query(
            r#"
            UPDATE quotes
            SET status = $2, updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(status)
        .execute(mm.dbx().db())
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(crate::model::Error::EntityNotFound {
                entity: "Quote",
                id: id as i64,
            });
        }

        Ok(())
    }

    /// Marks a quote as sent.
    #[instrument(skip(mm))]
    pub async fn send(mm: &ModelManager, id: i32) -> Result<()> {
        Self::update_status(mm, id, "sent").await
    }

    /// Accepts a quote and optionally creates a booking.
    #[instrument(skip(mm))]
    pub async fn accept(
        mm: &ModelManager,
        id: i32,
        booking_id: Option<i32>,
        customer_notes: Option<&str>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE quotes
            SET status = 'accepted',
                accepted_at = CURRENT_TIMESTAMP,
                booking_id = $2,
                customer_notes = $3,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(booking_id)
        .bind(customer_notes)
        .execute(mm.dbx().db())
        .await?;

        Ok(())
    }

    /// Rejects a quote with optional reason.
    #[instrument(skip(mm))]
    pub async fn reject(mm: &ModelManager, id: i32, reason: Option<&str>) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE quotes
            SET status = 'rejected',
                customer_notes = $2,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(reason)
        .execute(mm.dbx().db())
        .await?;

        Ok(())
    }

    /// Deletes a quote.
    #[instrument(skip(mm))]
    pub async fn delete(mm: &ModelManager, id: i32) -> Result<()> {
        let rows_affected = sqlx::query("DELETE FROM quotes WHERE id = $1")
            .bind(id)
            .execute(mm.dbx().db())
            .await?
            .rows_affected();

        if rows_affected == 0 {
            return Err(crate::model::Error::EntityNotFound {
                entity: "Quote",
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

    fn test_items() -> Vec<QuoteItem> {
        vec![
            QuoteItem {
                description: "Call-out fee".to_string(),
                quantity: 1,
                unit_price: 3000,
            },
            QuoteItem {
                description: "Labour (1 hour)".to_string(),
                quantity: 1,
                unit_price: 4500,
            },
        ]
    }

    #[tokio::test]
    async fn test_quote_create_ok() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Execute
        let quote = QuoteForCreate {
            customer_id: None,
            title: "Test Quote Create".to_string(),
            items: test_items(),
            valid_days: Some(30),
        };

        let id = QuoteBmc::create(&mm, quote).await?;

        // Check
        assert!(id > 0, "Should return valid ID");

        // Cleanup
        QuoteBmc::delete(&mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_quote_get_ok() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test quote
        let quote = QuoteForCreate {
            customer_id: None,
            title: "Test Quote Get".to_string(),
            items: test_items(),
            valid_days: Some(30),
        };
        let id = QuoteBmc::create(&mm, quote).await?;

        // Execute
        let quote = QuoteBmc::get(&mm, id).await?;

        // Check
        assert_eq!(quote.id, id);
        assert_eq!(quote.title, "Test Quote Get");
        assert_eq!(quote.status, "draft");
        assert_eq!(quote.subtotal_cents, 7500); // 3000 + 4500
        assert_eq!(quote.total_cents, 7500);

        // Cleanup
        QuoteBmc::delete(&mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_quote_get_err_not_found() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;
        let fx_id = 999999;

        // Execute
        let res = QuoteBmc::get(&mm, fx_id).await;

        // Check
        assert!(res.is_err(), "Should return error for non-existent quote");

        Ok(())
    }

    #[tokio::test]
    async fn test_quote_list_ok() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test quotes
        let quote1 = QuoteForCreate {
            customer_id: None,
            title: "Test List Quote 1".to_string(),
            items: test_items(),
            valid_days: Some(30),
        };
        let quote2 = QuoteForCreate {
            customer_id: None,
            title: "Test List Quote 2".to_string(),
            items: test_items(),
            valid_days: Some(30),
        };

        let id1 = QuoteBmc::create(&mm, quote1).await?;
        let id2 = QuoteBmc::create(&mm, quote2).await?;

        // Execute
        let quotes = QuoteBmc::list(&mm).await?;

        // Check
        assert!(quotes.len() >= 2, "Should have at least 2 quotes");

        // Cleanup
        QuoteBmc::delete(&mm, id1).await?;
        QuoteBmc::delete(&mm, id2).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_quote_list_by_status() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test quote
        let quote = QuoteForCreate {
            customer_id: None,
            title: "Test Status Filter".to_string(),
            items: test_items(),
            valid_days: Some(30),
        };
        let id = QuoteBmc::create(&mm, quote).await?;

        // Execute
        let draft_quotes = QuoteBmc::list_by_status(&mm, "draft").await?;

        // Check
        assert!(
            draft_quotes.iter().any(|q| q.id == id),
            "Should find newly created quote in draft list"
        );

        // Cleanup
        QuoteBmc::delete(&mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_quote_update_status() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test quote
        let quote = QuoteForCreate {
            customer_id: None,
            title: "Test Update Status".to_string(),
            items: test_items(),
            valid_days: Some(30),
        };
        let id = QuoteBmc::create(&mm, quote).await?;

        // Execute
        QuoteBmc::update_status(&mm, id, "sent").await?;

        // Check
        let quote = QuoteBmc::get(&mm, id).await?;
        assert_eq!(quote.status, "sent");

        // Cleanup
        QuoteBmc::delete(&mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_quote_send() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test quote
        let quote = QuoteForCreate {
            customer_id: None,
            title: "Test Send".to_string(),
            items: test_items(),
            valid_days: Some(30),
        };
        let id = QuoteBmc::create(&mm, quote).await?;

        // Execute
        QuoteBmc::send(&mm, id).await?;

        // Check
        let quote = QuoteBmc::get(&mm, id).await?;
        assert_eq!(quote.status, "sent");

        // Cleanup
        QuoteBmc::delete(&mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_quote_accept() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test quote
        let quote = QuoteForCreate {
            customer_id: None,
            title: "Test Accept".to_string(),
            items: test_items(),
            valid_days: Some(30),
        };
        let id = QuoteBmc::create(&mm, quote).await?;

        // Execute
        QuoteBmc::accept(&mm, id, None, Some("Looks good!")).await?;

        // Check
        let quote = QuoteBmc::get(&mm, id).await?;
        assert_eq!(quote.status, "accepted");
        assert!(quote.accepted_at.is_some());
        assert_eq!(quote.customer_notes, Some("Looks good!".to_string()));

        // Cleanup
        QuoteBmc::delete(&mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_quote_reject() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test quote
        let quote = QuoteForCreate {
            customer_id: None,
            title: "Test Reject".to_string(),
            items: test_items(),
            valid_days: Some(30),
        };
        let id = QuoteBmc::create(&mm, quote).await?;

        // Execute
        QuoteBmc::reject(&mm, id, Some("Too expensive")).await?;

        // Check
        let quote = QuoteBmc::get(&mm, id).await?;
        assert_eq!(quote.status, "rejected");
        assert_eq!(quote.customer_notes, Some("Too expensive".to_string()));

        // Cleanup
        QuoteBmc::delete(&mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_quote_delete_ok() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test quote
        let quote = QuoteForCreate {
            customer_id: None,
            title: "Test Delete".to_string(),
            items: test_items(),
            valid_days: Some(30),
        };
        let id = QuoteBmc::create(&mm, quote).await?;

        // Execute
        QuoteBmc::delete(&mm, id).await?;

        // Check - should not be able to get deleted quote
        let res = QuoteBmc::get(&mm, id).await;
        assert!(res.is_err(), "Should not find deleted quote");

        Ok(())
    }

    #[tokio::test]
    async fn test_quote_delete_err_not_found() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;
        let fx_id = 999999;

        // Execute
        let res = QuoteBmc::delete(&mm, fx_id).await;

        // Check
        assert!(
            res.is_err(),
            "Should return error when deleting non-existent quote"
        );

        Ok(())
    }
}

// endregion: --- Tests
