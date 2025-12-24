//! # Pagination Support
//!
//! Common pagination types and utilities for list endpoints.
//!
//! ## Example
//!
//! ```rust,no_run
//! use lib_core::model::pagination::{Pagination, PaginatedResult};
//!
//! let pagination = Pagination::new(1, 20);
//! let results: PaginatedResult<Customer> = CustomerBmc::list_paginated(&mm, pagination).await?;
//! println!("Page {} of {}", results.page, results.total_pages);
//! ```

use serde::{Deserialize, Serialize};

/// Pagination parameters for list queries.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Pagination {
    /// Current page number (1-indexed)
    pub page: u32,
    /// Number of items per page
    pub per_page: u32,
    /// Optional sorting field
    pub sort_by: Option<String>,
    /// Sort direction: "asc" or "desc"
    pub sort_dir: Option<String>,
}

impl Pagination {
    /// Creates new pagination with page and per_page.
    #[must_use]
    pub fn new(page: u32, per_page: u32) -> Self {
        Self {
            page: page.max(1),                  // Minimum page 1
            per_page: per_page.min(100).max(1), // 1-100 items
            sort_by: None,
            sort_dir: None,
        }
    }

    /// Calculates the SQL OFFSET for this pagination.
    #[must_use]
    #[inline]
    pub fn offset(&self) -> u32 {
        (self.page.saturating_sub(1)) * self.per_page
    }

    /// Calculates the SQL LIMIT for this pagination.
    #[must_use]
    #[inline]
    pub fn limit(&self) -> u32 {
        self.per_page
    }

    /// Creates pagination for first page with default size (20).
    #[must_use]
    pub fn first_page() -> Self {
        Self::new(1, 20)
    }
}

/// Paginated result wrapper containing items and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResult<T> {
    /// Items for the current page
    pub items: Vec<T>,
    /// Current page number
    pub page: u32,
    /// Items per page
    pub per_page: u32,
    /// Total number of items across all pages
    pub total_items: u64,
    /// Total number of pages
    pub total_pages: u32,
    /// Whether there is a next page
    pub has_next: bool,
    /// Whether there is a previous page
    pub has_prev: bool,
}

impl<T> PaginatedResult<T> {
    /// Creates a new paginated result from items and counts.
    #[must_use]
    pub fn new(items: Vec<T>, pagination: &Pagination, total_items: u64) -> Self {
        let total_pages = ((total_items as f64) / (pagination.per_page as f64)).ceil() as u32;
        Self {
            items,
            page: pagination.page,
            per_page: pagination.per_page,
            total_items,
            total_pages: total_pages.max(1),
            has_next: pagination.page < total_pages,
            has_prev: pagination.page > 1,
        }
    }

    /// Returns true if the result is empty.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns the number of items on this page.
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_offset() {
        let p = Pagination::new(1, 20);
        assert_eq!(p.offset(), 0);

        let p = Pagination::new(2, 20);
        assert_eq!(p.offset(), 20);

        let p = Pagination::new(3, 10);
        assert_eq!(p.offset(), 20);
    }

    #[test]
    fn test_pagination_bounds() {
        let p = Pagination::new(0, 20);
        assert_eq!(p.page, 1); // Minimum 1

        let p = Pagination::new(1, 1000);
        assert_eq!(p.per_page, 100); // Maximum 100
    }

    #[test]
    fn test_paginated_result() {
        let pagination = Pagination::new(2, 10);
        let result: PaginatedResult<i32> = PaginatedResult::new(vec![1, 2, 3], &pagination, 25);

        assert_eq!(result.page, 2);
        assert_eq!(result.total_pages, 3);
        assert!(result.has_prev);
        assert!(result.has_next);
    }
}
