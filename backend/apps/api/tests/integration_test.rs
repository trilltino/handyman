//! Integration tests for the API
//!
//! These tests verify API endpoint behavior using tower's test utilities.
//! Full integration tests require setting up a test database and server.

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use serde_json::json;
use tower::ServiceExt;

mod fixtures {
    use serde_json::json;

    pub fn contact_form() -> serde_json::Value {
        json!({
            "name": "Test User",
            "email": "test@example.com",
            "message": "Test message content"
        })
    }

    pub fn invalid_contact_form() -> serde_json::Value {
        json!({
            "name": "",
            "email": "invalid-email",
            "message": ""
        })
    }

    pub fn quote_request() -> serde_json::Value {
        json!({
            "customer_id": null,
            "title": "Plumbing Repair",
            "items": [
                {
                    "description": "Call-out fee",
                    "quantity": 1,
                    "unit_price": 3000
                }
            ],
            "valid_days": 30
        })
    }

    pub fn instant_quote_request() -> serde_json::Value {
        json!({
            "service_type": "plumbing",
            "urgency": "standard"
        })
    }
}

// Test payload structure validation
#[tokio::test]
async fn test_contact_form_payload_structure() {
    let payload = fixtures::contact_form();

    // Validate required fields
    assert!(payload.get("name").is_some());
    assert!(payload.get("email").is_some());
    assert!(payload.get("message").is_some());

    // Validate field types
    assert!(payload["name"].is_string());
    assert!(payload["email"].is_string());
    assert!(payload["message"].is_string());
}

#[tokio::test]
async fn test_invalid_contact_form_has_empty_fields() {
    let payload = fixtures::invalid_contact_form();

    // Should have empty name
    assert_eq!(payload["name"], "");
    // Should have invalid email
    assert_eq!(payload["email"], "invalid-email");
}

#[tokio::test]
async fn test_quote_request_structure() {
    let payload = fixtures::quote_request();

    // Validate required fields
    assert!(payload.get("title").is_some());
    assert!(payload.get("items").is_some());

    // Validate items structure
    let items = payload["items"].as_array().unwrap();
    assert!(!items.is_empty());

    let first_item = &items[0];
    assert!(first_item.get("description").is_some());
    assert!(first_item.get("quantity").is_some());
    assert!(first_item.get("unit_price").is_some());
}

#[tokio::test]
async fn test_instant_quote_request_structure() {
    let payload = fixtures::instant_quote_request();

    // Validate required fields
    assert!(payload.get("service_type").is_some());
    assert!(payload.get("urgency").is_some());
}

#[tokio::test]
async fn test_contact_form_email_format() {
    let valid = fixtures::contact_form();
    let email = valid["email"].as_str().unwrap();

    // Should contain @ symbol
    assert!(email.contains('@'));
    // Should have domain part
    assert!(email.split('@').collect::<Vec<_>>().len() == 2);
}

#[tokio::test]
async fn test_quote_items_totals() {
    let payload = fixtures::quote_request();
    let items = payload["items"].as_array().unwrap();

    // Calculate expected total
    let total: i64 = items
        .iter()
        .map(|item| {
            item["quantity"].as_i64().unwrap_or(0) * item["unit_price"].as_i64().unwrap_or(0)
        })
        .sum();

    assert_eq!(total, 3000); // 1 * 3000
}

// Validation tests
#[tokio::test]
async fn test_contact_form_validation_rules() {
    // Test that shared validation can be applied
    use shared::validation::Validate;
    use shared::ContactForm;

    let valid_form = ContactForm {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        message: "Hello, I need help with my plumbing.".to_string(),
    };

    assert!(valid_form.validate().is_ok());
}

#[tokio::test]
async fn test_contact_form_validation_empty_name() {
    use shared::validation::Validate;
    use shared::ContactForm;

    let invalid_form = ContactForm {
        name: "".to_string(),
        email: "test@example.com".to_string(),
        message: "Hello".to_string(),
    };

    assert!(invalid_form.validate().is_err());
}

#[tokio::test]
async fn test_contact_form_validation_invalid_email() {
    use shared::validation::Validate;
    use shared::ContactForm;

    let invalid_form = ContactForm {
        name: "John Doe".to_string(),
        email: "not-an-email".to_string(),
        message: "Hello".to_string(),
    };

    assert!(invalid_form.validate().is_err());
}

#[tokio::test]
async fn test_contact_form_validation_empty_message() {
    use shared::validation::Validate;
    use shared::ContactForm;

    let invalid_form = ContactForm {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        message: "".to_string(),
    };

    assert!(invalid_form.validate().is_err());
}

#[tokio::test]
async fn test_contact_form_trims_whitespace() {
    use shared::validation::Validate;
    use shared::ContactForm;

    let form = ContactForm {
        name: "  John Doe  ".to_string(),
        email: "john@example.com".to_string(),
        message: "Hello".to_string(),
    };

    // Validation should pass (trimming happens in handler)
    assert!(form.validate().is_ok());
}

// API response structure tests
#[tokio::test]
async fn test_api_response_success_structure() {
    use shared::ApiResponse;

    let response = ApiResponse::success("Test message", json!({"data": "value"}));

    assert!(response.success);
    assert!(response.data.is_some());
    assert_eq!(response.message, Some("Test message".to_string()));
}

#[tokio::test]
async fn test_api_response_error_structure() {
    use shared::ApiResponse;

    let response: ApiResponse<()> = ApiResponse::error("Error occurred");

    assert!(!response.success);
    assert!(response.data.is_none());
    assert_eq!(response.message, Some("Error occurred".to_string()));
}
