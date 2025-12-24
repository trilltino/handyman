//! # API Integration Tests
//!
//! Tests for the REST API endpoints.

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use serde_json::{json, Value};
use tower::ServiceExt;

/// Test fixtures for consistent test data.
pub mod fixtures {
    use serde_json::json;

    /// Sample contact form data.
    pub fn contact_form() -> serde_json::Value {
        json!({
            "name": "John Doe",
            "email": "john@example.com",
            "message": "This is a test message."
        })
    }

    /// Sample invalid contact form (missing email).
    pub fn invalid_contact_form() -> serde_json::Value {
        json!({
            "name": "John Doe",
            "email": "",
            "message": "Test"
        })
    }

    /// Sample quote request.
    pub fn quote_request() -> serde_json::Value {
        json!({
            "customer_id": 1,
            "title": "Plumbing Repair",
            "items": [
                {
                    "description": "Call-out fee",
                    "quantity": 1,
                    "unit_price": 3000
                },
                {
                    "description": "Labour (1 hour)",
                    "quantity": 1,
                    "unit_price": 4500
                }
            ],
            "valid_days": 30
        })
    }

    /// Sample booking request.
    pub fn booking_request() -> serde_json::Value {
        json!({
            "customer_id": 1,
            "service_type": "plumbing",
            "scheduled_date": "2025-01-15",
            "scheduled_time": "10:00",
            "notes": "Leaky tap in kitchen"
        })
    }

    /// Sample customer.
    pub fn customer() -> serde_json::Value {
        json!({
            "name": "Jane Smith",
            "email": "jane@example.com",
            "phone": "+44 7833 263486",
            "notes": "Preferred customer"
        })
    }
}

/// Test helper to make JSON POST requests.
pub async fn post_json(app: &Router, uri: &str, body: Value) -> (StatusCode, Value) {
    let request = Request::builder()
        .method("POST")
        .uri(uri)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap_or(json!({}));

    (status, body)
}

/// Test helper to make GET requests.
pub async fn get_json(app: &Router, uri: &str) -> (StatusCode, Value) {
    let request = Request::builder()
        .method("GET")
        .uri(uri)
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap_or(json!({}));

    (status, body)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require a running database and proper app setup.
    // For now, they serve as a template for integration testing.

    #[tokio::test]
    async fn test_fixtures_are_valid_json() {
        let contact = fixtures::contact_form();
        assert!(contact.get("name").is_some());
        assert!(contact.get("email").is_some());
        assert!(contact.get("message").is_some());
    }

    #[tokio::test]
    async fn test_invalid_contact_has_empty_email() {
        let invalid = fixtures::invalid_contact_form();
        let email = invalid.get("email").and_then(|v| v.as_str());
        assert_eq!(email, Some(""));
    }

    #[tokio::test]
    async fn test_quote_fixture_has_items() {
        let quote = fixtures::quote_request();
        let items = quote.get("items").and_then(|v| v.as_array());
        assert!(items.is_some());
        assert_eq!(items.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_booking_fixture_has_service_type() {
        let booking = fixtures::booking_request();
        let service = booking.get("service_type").and_then(|v| v.as_str());
        assert_eq!(service, Some("plumbing"));
    }
}
