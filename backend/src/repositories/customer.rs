//! # Customer Repository
//!
//! ## Purpose
//! Database access layer for Customer entity.
//! Handles customer creation with duplicate prevention (email-based).
//!
//! ## Key Feature: create_or_get
//! Smart creation that prevents duplicate customers:
//! 1. Check if customer with email already exists
//! 2. If exists: Return existing customer
//! 3. If not: Create new customer and return it
//!
//! ## Relation to Entire Program
//! - **Called By**: BookingRepository (when creating bookings)
//! - **Database**: Queries `customers` table
//! - **Business Logic**: Reuses existing customers instead of creating duplicates

use crate::models::{Customer, NewCustomer};  // Customer models
use anyhow::{Context, Result};               // Error handling with context
use bb8_postgres::tokio_postgres::Row;       // Database row type
use tokio_postgres::Client;                  // PostgreSQL async client

/// Customer repository - handles all database operations for customers table
pub struct CustomerRepository;

impl CustomerRepository {
    /// Create a new customer or return existing one by email
    /// Called by: BookingRepository::create() when processing booking
    ///
    /// # Smart Deduplication
    /// Prevents duplicate customers with same email by:
    /// 1. Checking if customer exists with this email
    /// 2. Returning existing customer if found
    /// 3. Creating new customer only if not found
    ///
    /// # Why This Matters
    /// - Prevents data duplication (same customer multiple bookings)
    /// - Maintains customer history across bookings
    /// - Email is unique identifier for customers
    pub async fn create_or_get(client: &Client, new_customer: &NewCustomer) -> Result<Customer> {
        // Try to find existing customer by email
        if let Some(customer) = Self::find_by_email(client, &new_customer.email).await? {
            return Ok(customer);  // Return existing customer
        }

        // No existing customer found - create new one
        let row = client
            .query_one(
                "INSERT INTO customers (full_name, email, phone)
                 VALUES ($1, $2, $3)
                 RETURNING id, full_name, email, phone, created_at, updated_at",
                &[&new_customer.full_name, &new_customer.email, &new_customer.phone],
            )
            .await
            .context("Failed to insert customer")?;

        Ok(Self::row_to_customer(row))
    }

    /// Find customer by email
    /// Used by: create_or_get to check for existing customers
    pub async fn find_by_email(client: &Client, email: &str) -> Result<Option<Customer>> {
        let rows = client
            .query(
                "SELECT id, full_name, email, phone, created_at, updated_at
                 FROM customers
                 WHERE email = $1",
                &[&email],
            )
            .await
            .context("Failed to query customer by email")?;

        // Return first row (email is unique) or None if not found
        Ok(rows.into_iter().next().map(Self::row_to_customer))
    }

    /// Find customer by ID
    /// Used for: Looking up customer details for a booking
    pub async fn find_by_id(client: &Client, id: i32) -> Result<Option<Customer>> {
        let rows = client
            .query(
                "SELECT id, full_name, email, phone, created_at, updated_at
                 FROM customers
                 WHERE id = $1",
                &[&id],
            )
            .await
            .context("Failed to query customer by ID")?;

        // Return first row or None if not found
        Ok(rows.into_iter().next().map(Self::row_to_customer))
    }

    /// Helper: Convert database row to Customer struct
    /// Maps column indices to Customer fields
    fn row_to_customer(row: Row) -> Customer {
        Customer {
            id: row.get(0),           // Column 0: id
            full_name: row.get(1),    // Column 1: full_name
            email: row.get(2),        // Column 2: email
            phone: row.get(3),        // Column 3: phone
            created_at: row.get(4),   // Column 4: created_at
            updated_at: row.get(5),   // Column 5: updated_at
        }
    }
}
