//! # Booking Repository
//!
//! ## Purpose
//! Database access layer for Booking entity.
//! Handles booking creation (with customer), payment updates, queries.
//!
//! ## Key Operations
//! - **create**: Creates booking AND customer (reuses existing customers by email)
//! - **update_payment**: Updates Stripe payment info after payment processing
//! - **find_by_id**: Fetch specific booking
//! - **list_recent**: List recent bookings (for admin dashboard, etc.)
//!
//! ## Relation to Entire Program
//! - **Called By**: Booking handler (POST /api/bookings), payment handlers
//! - **Calls**: CustomerRepository to handle customer creation
//! - **Database**: Queries `bookings` and `customers` tables
//! - **Stripe Integration**: Stores payment_intent_id and payment_method_id

use crate::models::{Booking, NewBooking, BookingStatus, PaymentStatus};  // Booking models and enums
use crate::repositories::CustomerRepository;  // Customer repository for creating/finding customers
use crate::models::NewCustomer;               // Customer DTO
use anyhow::{Context, Result};                // Error handling with context
use bb8_postgres::tokio_postgres::Row;        // Database row type
use tokio_postgres::Client;                   // PostgreSQL async client

/// Booking repository - handles all database operations for bookings table
pub struct BookingRepository;

impl BookingRepository {
    /// Create a new booking with customer
    /// Called by: POST /api/bookings handler
    ///
    /// # Multi-Step Process
    /// 1. Extract customer info from booking form
    /// 2. Create or get existing customer (prevents duplicates)
    /// 3. Calculate price based on work type
    /// 4. Insert booking with Pending status
    /// 5. Return created booking
    ///
    /// # Initial State
    /// - status: Pending (awaiting handyman assignment)
    /// - payment_status: Pending (awaiting payment processing)
    pub async fn create(client: &Client, new_booking: &NewBooking) -> Result<Booking> {
        // Step 1: Create customer DTO from booking form data
        let new_customer = NewCustomer::new(
            new_booking.name.clone(),
            new_booking.email.clone(),
            new_booking.phone.clone(),
        );

        // Step 2: Create or get existing customer by email
        let customer = CustomerRepository::create_or_get(client, &new_customer).await?;

        // Step 3: Calculate price in cents based on work type
        let price_cents = new_booking.price_cents();

        // Step 4: Insert booking into database
        let row = client
            .query_one(
                "INSERT INTO bookings (
                    customer_id, location, work_type, description,
                    price_cents, status, payment_status
                 )
                 VALUES ($1, $2, $3, $4, $5, $6, $7)
                 RETURNING id, customer_id, location, work_type, description,
                           price_cents, status, payment_status, payment_intent_id,
                           payment_method_id, scheduled_date, created_at, updated_at",
                &[
                    &customer.id,
                    &new_booking.location,
                    &new_booking.work_type,
                    &new_booking.description,
                    &price_cents,
                    &BookingStatus::Pending.as_str(),      // Initial status
                    &PaymentStatus::Pending.as_str(),      // Initial payment status
                ],
            )
            .await
            .context("Failed to insert booking")?;

        Ok(Self::row_to_booking(row))
    }

    /// Update booking payment information after Stripe processing
    /// Called by: Payment confirmation handler after successful Stripe payment
    ///
    /// # Arguments
    /// - booking_id: Which booking to update
    /// - payment_intent_id: Stripe payment intent ID (pi_xxx)
    /// - payment_method_id: Stripe payment method ID (pm_xxx)
    /// - payment_status: New payment status (Paid, Failed, etc.)
    pub async fn update_payment(
        client: &Client,
        booking_id: i32,
        payment_intent_id: &str,
        payment_method_id: Option<&str>,
        payment_status: PaymentStatus,
    ) -> Result<Booking> {
        let row = client
            .query_one(
                "UPDATE bookings
                 SET payment_intent_id = $1,
                     payment_method_id = $2,
                     payment_status = $3
                 WHERE id = $4
                 RETURNING id, customer_id, location, work_type, description,
                           price_cents, status, payment_status, payment_intent_id,
                           payment_method_id, scheduled_date, created_at, updated_at",
                &[&payment_intent_id, &payment_method_id, &payment_status.as_str(), &booking_id],
            )
            .await
            .context("Failed to update booking payment")?;

        Ok(Self::row_to_booking(row))
    }

    /// Find booking by ID
    /// Used for: Fetching specific booking details, payment confirmation
    pub async fn find_by_id(client: &Client, id: i32) -> Result<Option<Booking>> {
        let rows = client
            .query(
                "SELECT id, customer_id, location, work_type, description,
                        price_cents, status, payment_status, payment_intent_id,
                        payment_method_id, scheduled_date, created_at, updated_at
                 FROM bookings
                 WHERE id = $1",
                &[&id],
            )
            .await
            .context("Failed to query booking by ID")?;

        Ok(rows.into_iter().next().map(Self::row_to_booking))
    }

    /// List recent bookings ordered by creation date
    /// Used for: Admin dashboard, recent activity display
    ///
    /// # Arguments
    /// - limit: Maximum number of bookings to return
    pub async fn list_recent(client: &Client, limit: i64) -> Result<Vec<Booking>> {
        let rows = client
            .query(
                "SELECT id, customer_id, location, work_type, description,
                        price_cents, status, payment_status, payment_intent_id,
                        payment_method_id, scheduled_date, created_at, updated_at
                 FROM bookings
                 ORDER BY created_at DESC
                 LIMIT $1",
                &[&limit],
            )
            .await
            .context("Failed to list recent bookings")?;

        Ok(rows.into_iter().map(Self::row_to_booking).collect())
    }

    /// Helper: Convert database row to Booking struct
    /// Maps column indices to Booking fields (13 total columns)
    fn row_to_booking(row: Row) -> Booking {
        Booking {
            id: row.get(0),                      // Column 0: id
            customer_id: row.get(1),             // Column 1: customer_id
            location: row.get(2),                // Column 2: location
            work_type: row.get(3),               // Column 3: work_type
            description: row.get(4),             // Column 4: description
            price_cents: row.get(5),             // Column 5: price_cents
            status: row.get(6),                  // Column 6: status
            payment_status: row.get(7),          // Column 7: payment_status
            payment_intent_id: row.get(8),       // Column 8: payment_intent_id
            payment_method_id: row.get(9),       // Column 9: payment_method_id
            scheduled_date: row.get(10),         // Column 10: scheduled_date
            created_at: row.get(11),             // Column 11: created_at
            updated_at: row.get(12),             // Column 12: updated_at
        }
    }
}
