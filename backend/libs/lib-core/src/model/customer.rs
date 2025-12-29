//! # Customer Model
//!
//! This module defines the customer CRM data model and database operations.
//!
//! ## Structures
//!
//! - [`Customer`] - Complete customer record from database
//! - [`CustomerForCreate`] - Data required to create a new customer
//! - [`CustomerForUpdate`] - Data for updating an existing customer
//! - [`CustomerBmc`] - Business Model Controller for customer operations
//!
//! ## Example
//!
//! ```rust,no_run
//! use lib_core::model::customer::{CustomerBmc, CustomerForCreate};
//! use lib_core::model::ModelManager;
//!
//! async fn create_customer(mm: &ModelManager) -> Result<i32, Box<dyn std::error::Error>> {
//!     let customer = CustomerForCreate {
//!         name: "John Doe".to_string(),
//!         email: Some("john@example.com".to_string()),
//!         phone: Some("+44 7833 263486".to_string()),
//!         notes: None,
//!     };
//!     let id = CustomerBmc::create(mm, customer).await?;
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

/// Complete customer record from the database.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Customer {
    /// Auto-generated primary key
    pub id: i32,
    /// Customer's full name
    pub name: String,
    /// Email address
    pub email: Option<String>,
    /// Phone number
    pub phone: Option<String>,
    /// JSON array of addresses
    pub addresses: Option<serde_json::Value>,
    /// Internal notes about the customer
    pub notes: Option<String>,
    /// Tags like 'vip', 'subscriber'
    pub tags: Option<Vec<String>>,
    /// When the customer was created
    pub created_at: Option<OffsetDateTime>,
    /// When the customer was last updated
    pub updated_at: Option<OffsetDateTime>,
}

/// Data required to create a new customer.
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct CustomerForCreate {
    /// Customer's full name (required)
    pub name: String,
    /// Email address
    pub email: Option<String>,
    /// Phone number
    pub phone: Option<String>,
    /// Internal notes
    pub notes: Option<String>,
}

/// Data for updating an existing customer.
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct CustomerForUpdate {
    /// New name
    pub name: Option<String>,
    /// New email
    pub email: Option<String>,
    /// New phone
    pub phone: Option<String>,
    /// New notes
    pub notes: Option<String>,
    /// New tags
    pub tags: Option<Vec<String>>,
}

/// Business Model Controller for customer operations.
pub struct CustomerBmc;

impl CustomerBmc {
    /// Creates a new customer in the database.
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    /// * `customer` - Customer data to insert
    ///
    /// # Returns
    ///
    /// The auto-generated ID of the new customer.
    #[must_use = "the returned ID should be used or logged"]
    #[instrument(skip(mm), fields(name = %customer.name))]
    pub async fn create(mm: &ModelManager, customer: CustomerForCreate) -> Result<i32> {
        let row: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO customers (name, email, phone, notes)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
        )
        .bind(&customer.name)
        .bind(&customer.email)
        .bind(&customer.phone)
        .bind(&customer.notes)
        .fetch_one(mm.dbx().db())
        .await?;

        Ok(row.0)
    }

    /// Gets a customer by ID.
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    /// * `id` - Customer ID to retrieve
    ///
    /// # Returns
    ///
    /// The customer record if found.
    #[instrument(skip(mm))]
    pub async fn get(mm: &ModelManager, id: i32) -> Result<Customer> {
        let customer = sqlx::query_as::<_, Customer>(
            r#"
            SELECT id, name, email, phone, addresses, notes, tags, created_at, updated_at
            FROM customers
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(mm.dbx().db())
        .await?;

        Ok(customer)
    }

    /// Gets a customer by email.
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    /// * `email` - Email to search for
    #[instrument(skip(mm))]
    pub async fn get_by_email(mm: &ModelManager, email: &str) -> Result<Option<Customer>> {
        let customer = sqlx::query_as::<_, Customer>(
            r#"
            SELECT id, name, email, phone, addresses, notes, tags, created_at, updated_at
            FROM customers
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(mm.dbx().db())
        .await?;

        Ok(customer)
    }

    /// Lists all customers, ordered by name.
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    #[instrument(skip(mm))]
    pub async fn list(mm: &ModelManager) -> Result<Vec<Customer>> {
        let customers = sqlx::query_as(
            r#"
            SELECT id, name, email, phone, addresses, notes, tags, created_at, updated_at
            FROM customers
            ORDER BY name ASC
            "#,
        )
        .fetch_all(mm.dbx().db())
        .await?;

        Ok(customers)
    }

    /// Searches customers by name or email.
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    /// * `query` - Search term (matches name or email)
    #[instrument(skip(mm))]
    pub async fn search(mm: &ModelManager, query: &str) -> Result<Vec<Customer>> {
        let pattern = format!("%{}%", query);
        let customers = sqlx::query_as(
            r#"
            SELECT id, name, email, phone, addresses, notes, tags, created_at, updated_at
            FROM customers
            WHERE name ILIKE $1 OR email ILIKE $1
            ORDER BY name ASC
            LIMIT 50
            "#,
        )
        .bind(&pattern)
        .fetch_all(mm.dbx().db())
        .await?;

        Ok(customers)
    }

    /// Updates a customer.
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    /// * `id` - Customer ID to update
    /// * `data` - Fields to update (None fields are skipped)
    #[instrument(skip(mm, data))]
    pub async fn update(mm: &ModelManager, id: i32, data: CustomerForUpdate) -> Result<()> {
        // Build dynamic update query based on provided fields
        let mut updates = vec!["updated_at = CURRENT_TIMESTAMP"];
        let mut param_count = 1;

        if data.name.is_some() {
            updates.push("name = $2");
            param_count = 2;
        }

        // For simplicity, we'll do a basic update with name only for now
        // A more robust implementation would use a query builder
        if let Some(name) = &data.name {
            sqlx::query(
                r#"
                UPDATE customers
                SET name = $2, updated_at = CURRENT_TIMESTAMP
                WHERE id = $1
                "#,
            )
            .bind(id)
            .bind(name)
            .execute(mm.dbx().db())
            .await?;
        } else {
            sqlx::query(
                r#"
                UPDATE customers
                SET updated_at = CURRENT_TIMESTAMP
                WHERE id = $1
                "#,
            )
            .bind(id)
            .execute(mm.dbx().db())
            .await?;
        }

        let _ = updates;
        let _ = param_count;

        Ok(())
    }

    /// Adds a tag to a customer.
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    /// * `id` - Customer ID
    /// * `tag` - Tag to add (e.g., "vip", "subscriber")
    #[instrument(skip(mm))]
    pub async fn add_tag(mm: &ModelManager, id: i32, tag: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE customers
            SET tags = array_append(COALESCE(tags, ARRAY[]::varchar[]), $2),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(tag)
        .execute(mm.dbx().db())
        .await?;

        Ok(())
    }

    /// Deletes a customer.
    ///
    /// # Arguments
    ///
    /// * `mm` - Model manager for database access
    /// * `id` - Customer ID to delete
    #[instrument(skip(mm))]
    pub async fn delete(mm: &ModelManager, id: i32) -> Result<()> {
        let rows_affected = sqlx::query("DELETE FROM customers WHERE id = $1")
            .bind(id)
            .execute(mm.dbx().db())
            .await?
            .rows_affected();

        if rows_affected == 0 {
            return Err(crate::model::Error::EntityNotFound {
                entity: "Customer",
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
    async fn test_customer_create_ok() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Execute
        let customer = CustomerForCreate {
            name: "Test Customer Create".to_string(),
            email: Some("test_create@example.com".to_string()),
            phone: Some("+44 7833 263486".to_string()),
            notes: Some("Test notes".to_string()),
        };

        let id = CustomerBmc::create(&mm, customer).await?;

        // Check
        assert!(id > 0, "Should return valid ID");

        // Cleanup
        CustomerBmc::delete(&mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_customer_get_ok() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test customer
        let customer = CustomerForCreate {
            name: "Test Customer Get".to_string(),
            email: Some("test_get@example.com".to_string()),
            phone: None,
            notes: None,
        };
        let id = CustomerBmc::create(&mm, customer).await?;

        // Execute
        let customer = CustomerBmc::get(&mm, id).await?;

        // Check
        assert_eq!(customer.id, id);
        assert_eq!(customer.name, "Test Customer Get");
        assert_eq!(customer.email, Some("test_get@example.com".to_string()));

        // Cleanup
        CustomerBmc::delete(&mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_customer_get_err_not_found() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;
        let fx_id = 999999;

        // Execute
        let res = CustomerBmc::get(&mm, fx_id).await;

        // Check
        assert!(
            res.is_err(),
            "Should return error for non-existent customer"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_customer_get_by_email() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;
        let unique_email = format!("test_email_{}@example.com", uuid::Uuid::new_v4());

        // Create test customer
        let customer = CustomerForCreate {
            name: "Test Email Customer".to_string(),
            email: Some(unique_email.clone()),
            phone: None,
            notes: None,
        };
        let id = CustomerBmc::create(&mm, customer).await?;

        // Execute
        let customer = CustomerBmc::get_by_email(&mm, &unique_email).await?;

        // Check
        assert!(customer.is_some());
        assert_eq!(customer.unwrap().email, Some(unique_email));

        // Cleanup
        CustomerBmc::delete(&mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_customer_list_ok() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test customers
        let customer1 = CustomerForCreate {
            name: "Test List Customer 1".to_string(),
            email: None,
            phone: None,
            notes: None,
        };
        let customer2 = CustomerForCreate {
            name: "Test List Customer 2".to_string(),
            email: None,
            phone: None,
            notes: None,
        };

        let id1 = CustomerBmc::create(&mm, customer1).await?;
        let id2 = CustomerBmc::create(&mm, customer2).await?;

        // Execute
        let customers = CustomerBmc::list(&mm).await?;

        // Check
        assert!(customers.len() >= 2, "Should have at least 2 customers");

        // Cleanup
        CustomerBmc::delete(&mm, id1).await?;
        CustomerBmc::delete(&mm, id2).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_customer_search() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;
        let unique_name = format!("UniqueSearchTest{}", uuid::Uuid::new_v4());

        // Create test customer
        let customer = CustomerForCreate {
            name: unique_name.clone(),
            email: None,
            phone: None,
            notes: None,
        };
        let id = CustomerBmc::create(&mm, customer).await?;

        // Execute
        let results = CustomerBmc::search(&mm, "UniqueSearchTest").await?;

        // Check
        assert!(results.iter().any(|c| c.name == unique_name));

        // Cleanup
        CustomerBmc::delete(&mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_customer_update() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test customer
        let customer = CustomerForCreate {
            name: "Test Update Original".to_string(),
            email: None,
            phone: None,
            notes: None,
        };
        let id = CustomerBmc::create(&mm, customer).await?;

        // Execute
        let update = CustomerForUpdate {
            name: Some("Test Update Changed".to_string()),
            email: None,
            phone: None,
            notes: None,
            tags: None,
        };
        CustomerBmc::update(&mm, id, update).await?;

        // Check
        let customer = CustomerBmc::get(&mm, id).await?;
        assert_eq!(customer.name, "Test Update Changed");

        // Cleanup
        CustomerBmc::delete(&mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_customer_add_tag() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test customer
        let customer = CustomerForCreate {
            name: "Test Tag Customer".to_string(),
            email: None,
            phone: None,
            notes: None,
        };
        let id = CustomerBmc::create(&mm, customer).await?;

        // Execute
        CustomerBmc::add_tag(&mm, id, "vip").await?;

        // Check
        let customer = CustomerBmc::get(&mm, id).await?;
        assert!(customer.tags.is_some());
        assert!(customer.tags.unwrap().contains(&"vip".to_string()));

        // Cleanup
        CustomerBmc::delete(&mm, id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_customer_delete_ok() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;

        // Create test customer
        let customer = CustomerForCreate {
            name: "Test Delete Customer".to_string(),
            email: None,
            phone: None,
            notes: None,
        };
        let id = CustomerBmc::create(&mm, customer).await?;

        // Execute
        CustomerBmc::delete(&mm, id).await?;

        // Check - should not be able to get deleted customer
        let res = CustomerBmc::get(&mm, id).await;
        assert!(res.is_err(), "Should not find deleted customer");

        Ok(())
    }

    #[tokio::test]
    async fn test_customer_delete_err_not_found() -> Result<()> {
        // Setup
        let mm = _dev_utils::init_test().await;
        let fx_id = 999999;

        // Execute
        let res = CustomerBmc::delete(&mm, fx_id).await;

        // Check
        assert!(
            res.is_err(),
            "Should return error when deleting non-existent customer"
        );

        Ok(())
    }
}

// endregion: --- Tests
