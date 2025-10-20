//! # Contact Form Handler
//!
//! ## Purpose
//! Handles contact form submissions from website visitors.
//! Stores inquiries for later response by customer support.
//!
//! ## Endpoint
//! - **POST /api/contact**: Submit contact form
//!
//! ## Process Flow
//! 1. Receive contact form data (name, email, message)
//! 2. Save to database with "new" status
//! 3. (Future) Send email notification to admin
//! 4. (Future) Send auto-reply to sender
//!
//! ## Relation to Entire Program
//! - **Called By**: Frontend contact page form submission
//! - **Calls**: ContactRepository → Database
//! - **Future**: Will integrate email notifications

use axum::{extract::State, http::StatusCode, Json};  // Axum HTTP types
use serde_json::json;                                 // JSON response helper

use crate::db::DbPool;                      // Database connection pool
use crate::models::NewContactMessage;       // Contact message DTO
use crate::repositories::ContactRepository; // Contact database operations

/// Handle contact form submission
/// Route: POST /api/contact
///
/// # Process
/// 1. Extract contact details from request
/// 2. Get database connection
/// 3. Save message with "new" status
/// 4. Return success confirmation
///
/// # Future Enhancements
/// - Send email notification to admin/support team
/// - Send auto-reply confirmation to sender
/// - Integrate with CRM system for ticket tracking
pub async fn handle_contact(
    State(pool): State<DbPool>,
    Json(payload): Json<NewContactMessage>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, String)> {
    tracing::info!("Received contact form from: {}", payload.email);

    // Get connection from pool
    let client = pool
        .get()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Save to database
    match ContactRepository::create(&client, &payload).await {
        Ok(message) => {
            tracing::info!("Contact message saved with ID: {}", message.id);

            // TODO: Send email notification to admin
            // email::send_contact_notification(&message)?;
            // TODO: Send auto-reply to sender
            // email::send_contact_confirmation(&message)?;

            Ok((
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "message": "Thank you for contacting us! We'll get back to you soon.",
                    "id": message.id
                }))
            ))
        }
        Err(err) => {
            tracing::error!("Failed to save contact message: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to save contact message".to_string(),
            ))
        }
    }
}
