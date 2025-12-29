//! # Booking Model
//!
//! This module defines the booking/job data model and database operations.
//!
//! ## Structures
//!
//! - [`Booking`] - Complete booking record from database
//! - [`BookingForCreate`] - Data required to create a new booking
//! - [`BookingForUpdate`] - Data for updating an existing booking
//! - [`BookingBmc`] - Business Model Controller for booking operations
//!
//! ## Example
//!
//! ```rust,no_run
//! use lib_core::model::booking::{BookingBmc, BookingForCreate};
//! use lib_core::model::ModelManager;
//!
//! async fn create_booking(mm: &ModelManager) -> Result<i32, Box<dyn std::error::Error>> {
//!     let booking = BookingForCreate {
//!         customer_id: Some(1),
//!         service_type: "plumbing".to_string(),
//!         scheduled_date: Some("2025-01-15".to_string()),
//!         scheduled_time: Some("10:00".to_string()),
//!         notes: Some("Leaky tap".to_string()),
//!     };
//!     let id = BookingBmc::create(mm, booking).await?;
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

/// Complete booking record from the database.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Booking {
    /// Auto-generated primary key
    pub id: i32,
    /// Customer ID (foreign key)
    pub customer_id: Option<i32>,
    /// Type of service requested
    pub service_type: String,
    /// Scheduled date for the job
    pub scheduled_date: Option<time::Date>,
    /// Scheduled time for the job
    pub scheduled_time: Option<time::Time>,
    /// Current status: pending, confirmed, completed, cancelled
    pub status: String,
    /// Quote ID if created from a quote
    pub quote_id: Option<i32>,
    /// Estimated duration in minutes
    pub estimated_duration: Option<i32>,
    /// Actual duration in minutes
    pub actual_duration: Option<i32>,
    /// When the job started
    pub started_at: Option<OffsetDateTime>,
    /// When the job was completed
    pub completed_at: Option<OffsetDateTime>,
    /// Customer rating (1-5)
    pub customer_rating: Option<i32>,
    /// Customer review text
    pub customer_review: Option<String>,
    /// When the booking was created
    pub created_at: Option<OffsetDateTime>,
    /// When the booking was last updated
    pub updated_at: Option<OffsetDateTime>,
}

/// Data required to create a new booking.
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct BookingForCreate {
    /// Customer ID (optional for walk-ins)
    pub customer_id: Option<i32>,
    /// Type of service requested
    pub service_type: String,
    /// Scheduled date (YYYY-MM-DD)
    pub scheduled_date: Option<String>,
    /// Scheduled time (HH:MM)
    pub scheduled_time: Option<String>,
    /// Additional notes
    pub notes: Option<String>,
}

/// Data for updating an existing booking.
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct BookingForUpdate {
    /// New status
    pub status: Option<String>,
    /// New scheduled date
    pub scheduled_date: Option<String>,
    /// New scheduled time
    pub scheduled_time: Option<String>,
    /// Estimated duration in minutes
    pub estimated_duration: Option<i32>,
    /// Customer rating (1-5)
    pub customer_rating: Option<i32>,
    /// Customer review text
    pub customer_review: Option<String>,
}

/// Business Model Controller for booking operations.
pub struct BookingBmc;

impl BookingBmc {
    /// Creates a new booking in the database.
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    /// * `booking` - Booking data to insert
    ///
    /// # Returns
    ///
    /// The auto-generated ID of the new booking.
    ///
    /// # Errors
    ///
    /// Returns an error if the database insert fails.
    #[must_use = "the returned ID should be used or logged"]
    #[instrument(skip(mm), fields(service_type = %booking.service_type))]
    pub async fn create(mm: &ModelManager, booking: BookingForCreate) -> Result<i32> {
        let row: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO bookings (customer_id, service_type, status)
            VALUES ($1, $2, 'pending')
            RETURNING id
            "#,
        )
        .bind(booking.customer_id)
        .bind(&booking.service_type)
        .fetch_one(mm.dbx().db())
        .await?;

        Ok(row.0)
    }

    /// Gets a booking by ID.
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    /// * `id` - Booking ID to retrieve
    ///
    /// # Returns
    ///
    /// The booking record if found.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails or booking not found.
    #[instrument(skip(mm))]
    pub async fn get(mm: &ModelManager, id: i32) -> Result<Booking> {
        let booking = sqlx::query_as::<_, Booking>(
            r#"
            SELECT id, customer_id, service_type, scheduled_date, scheduled_time,
                   status, quote_id, estimated_duration, actual_duration,
                   started_at, completed_at, customer_rating, customer_review,
                   created_at, updated_at
            FROM bookings
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(mm.dbx().db())
        .await?;

        Ok(booking)
    }

    /// Lists all bookings, ordered by creation date (newest first).
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    ///
    /// # Returns
    ///
    /// A vector of all bookings.
    #[instrument(skip(mm))]
    pub async fn list(mm: &ModelManager) -> Result<Vec<Booking>> {
        let bookings = sqlx::query_as(
            r#"
            SELECT id, customer_id, service_type, scheduled_date, scheduled_time,
                   status, quote_id, estimated_duration, actual_duration,
                   started_at, completed_at, customer_rating, customer_review,
                   created_at, updated_at
            FROM bookings
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(mm.dbx().db())
        .await?;

        Ok(bookings)
    }

    /// Lists bookings by status.
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    /// * `status` - Status to filter by (pending, confirmed, completed, cancelled)
    #[instrument(skip(mm))]
    pub async fn list_by_status(mm: &ModelManager, status: &str) -> Result<Vec<Booking>> {
        let bookings = sqlx::query_as(
            r#"
            SELECT id, customer_id, service_type, scheduled_date, scheduled_time,
                   status, quote_id, estimated_duration, actual_duration,
                   started_at, completed_at, customer_rating, customer_review,
                   created_at, updated_at
            FROM bookings
            WHERE status = $1
            ORDER BY scheduled_date ASC, scheduled_time ASC
            "#,
        )
        .bind(status)
        .fetch_all(mm.dbx().db())
        .await?;

        Ok(bookings)
    }

    /// Updates a booking's status.
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    /// * `id` - Booking ID to update
    /// * `status` - New status
    #[instrument(skip(mm))]
    pub async fn update_status(mm: &ModelManager, id: i32, status: &str) -> Result<()> {
        let rows_affected = sqlx::query(
            r#"
            UPDATE bookings
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
                entity: "Booking",
                id: id as i64,
            });
        }

        Ok(())
    }

    /// Marks a booking as completed.
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    /// * `id` - Booking ID to complete
    /// * `actual_duration` - Actual time taken in minutes
    #[instrument(skip(mm))]
    pub async fn complete(mm: &ModelManager, id: i32, actual_duration: i32) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE bookings
            SET status = 'completed',
                completed_at = CURRENT_TIMESTAMP,
                actual_duration = $2,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(actual_duration)
        .execute(mm.dbx().db())
        .await?;

        Ok(())
    }

    /// Deletes a booking.
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    /// * `id` - Booking ID to delete
    #[instrument(skip(mm))]
    pub async fn delete(mm: &ModelManager, id: i32) -> Result<()> {
        let rows_affected = sqlx::query("DELETE FROM bookings WHERE id = $1")
            .bind(id)
            .execute(mm.dbx().db())
            .await?
            .rows_affected();

        if rows_affected == 0 {
            return Err(crate::model::Error::EntityNotFound {
                entity: "Booking",
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
    async fn test_booking_create_ok() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Execute
        let booking = BookingForCreate {
            customer_id: None,
            service_type: "test_plumbing".to_string(),
            scheduled_date: Some("2025-01-15".to_string()),
            scheduled_time: Some("10:00".to_string()),
            notes: Some("Leaky tap in kitchen".to_string()),
        };

        let id = BookingBmc::create(&mm, booking).await?;

        // Check
        assert!(id > 0, "Should return valid ID");

        // Cleanup
        BookingBmc::delete(&mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_booking_get_ok() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test booking
        let booking = BookingForCreate {
            customer_id: None,
            service_type: "test_electrical".to_string(),
            scheduled_date: None,
            scheduled_time: None,
            notes: None,
        };
        let id = BookingBmc::create(&mm, booking).await?;

        // Execute
        let booking = BookingBmc::get(&mm, id).await?;

        // Check
        assert_eq!(booking.id, id);
        assert_eq!(booking.service_type, "test_electrical");
        assert_eq!(booking.status, "pending");

        // Cleanup
        BookingBmc::delete(&mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_booking_get_err_not_found() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;
        let fx_id = 999999;

        // Execute
        let res = BookingBmc::get(&mm, fx_id).await;

        // Check
        assert!(res.is_err(), "Should return error for non-existent booking");

        Ok(())
    }

    #[tokio::test]
    async fn test_booking_list_ok() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test bookings
        let booking1 = BookingForCreate {
            customer_id: None,
            service_type: "test_list_1".to_string(),
            scheduled_date: None,
            scheduled_time: None,
            notes: None,
        };
        let booking2 = BookingForCreate {
            customer_id: None,
            service_type: "test_list_2".to_string(),
            scheduled_date: None,
            scheduled_time: None,
            notes: None,
        };

        let id1 = BookingBmc::create(&mm, booking1).await?;
        let id2 = BookingBmc::create(&mm, booking2).await?;

        // Execute
        let bookings = BookingBmc::list(&mm).await?;

        // Check
        assert!(bookings.len() >= 2, "Should have at least 2 bookings");

        // Cleanup
        BookingBmc::delete(&mm, id1).await?;
        BookingBmc::delete(&mm, id2).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_booking_list_by_status() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test booking
        let booking = BookingForCreate {
            customer_id: None,
            service_type: "test_status_filter".to_string(),
            scheduled_date: None,
            scheduled_time: None,
            notes: None,
        };
        let id = BookingBmc::create(&mm, booking).await?;

        // Execute
        let pending_bookings = BookingBmc::list_by_status(&mm, "pending").await?;

        // Check
        assert!(
            pending_bookings.iter().any(|b| b.id == id),
            "Should find newly created booking in pending list"
        );

        // Cleanup
        BookingBmc::delete(&mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_booking_update_status() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test booking
        let booking = BookingForCreate {
            customer_id: None,
            service_type: "test_update_status".to_string(),
            scheduled_date: None,
            scheduled_time: None,
            notes: None,
        };
        let id = BookingBmc::create(&mm, booking).await?;

        // Execute
        BookingBmc::update_status(&mm, id, "confirmed").await?;

        // Check
        let booking = BookingBmc::get(&mm, id).await?;
        assert_eq!(booking.status, "confirmed");

        // Cleanup
        BookingBmc::delete(&mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_booking_complete() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test booking
        let booking = BookingForCreate {
            customer_id: None,
            service_type: "test_complete".to_string(),
            scheduled_date: None,
            scheduled_time: None,
            notes: None,
        };
        let id = BookingBmc::create(&mm, booking).await?;

        // Execute
        BookingBmc::complete(&mm, id, 120).await?;

        // Check
        let booking = BookingBmc::get(&mm, id).await?;
        assert_eq!(booking.status, "completed");
        assert_eq!(booking.actual_duration, Some(120));
        assert!(booking.completed_at.is_some());

        // Cleanup
        BookingBmc::delete(&mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_booking_delete_ok() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test booking
        let booking = BookingForCreate {
            customer_id: None,
            service_type: "test_delete".to_string(),
            scheduled_date: None,
            scheduled_time: None,
            notes: None,
        };
        let id = BookingBmc::create(&mm, booking).await?;

        // Execute
        BookingBmc::delete(&mm, id).await?;

        // Check - should not be able to get deleted booking
        let res = BookingBmc::get(&mm, id).await;
        assert!(res.is_err(), "Should not find deleted booking");

        Ok(())
    }

    #[tokio::test]
    async fn test_booking_delete_err_not_found() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;
        let fx_id = 999999;

        // Execute
        let res = BookingBmc::delete(&mm, fx_id).await;

        // Check
        assert!(
            res.is_err(),
            "Should return error when deleting non-existent booking"
        );

        Ok(())
    }
}

// endregion: --- Tests
