//! Contact form submission handler.
//!
//! This module handles contact form submissions from the frontend,
//! including validation, database storage, and email notifications.
//!
//! # Security
//!
//! - Input validation via `ContactForm::validate()`
//! - Email sent asynchronously to prevent blocking
//! - Database errors logged but not exposed to client

use axum::extract::{Json, State};
use lib_core::email::email_service;
use lib_core::model::contact::{ContactBmc, ContactForCreate};
use lib_core::model::ModelManager;
use lib_web::Error;
use serde_json::{json, Value};
use tracing::{error, info};
use shared::{ApiResponse, ContactForm};

/// Handles contact form submissions.
#[utoipa::path(
    post,
    path = "/api/contact",
    tag = "contact",
    request_body = ContactForm,
    responses(
        (status = 200, description = "Contact form submitted successfully", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn api_contact_handler(
    State(mm): State<ModelManager>,
    Json(payload): Json<ContactForm>,
) -> Result<Json<ApiResponse<Value>>, Error> {
    payload.validate()
        .map_err(|e| Error::ValidationError(e))?;

    let contact = ContactForCreate {
        name: payload.name,
        email: payload.email,
        subject: None,
        message: payload.message,
        ip_address: None,
        user_agent: None,
    };

    // Save the contact to database
    let id = ContactBmc::create(&mm, contact.clone())
        .await
        .map_err(|e| {
            error!("Failed to save contact form: {}", e);
            Error::Model(e)
        })?;

    // Send notification email asynchronously (non-blocking)
    let email_payload = contact.clone();
    tokio::spawn(async move {
        if let Err(e) = send_contact_email(&email_payload).await {
            error!("Failed to send contact notification email: {}", e);
        }
    });

    info!("Contact form submitted successfully, id: {}", id);
    Ok(Json(ApiResponse::success(
        "Contact form submitted successfully",
        json!({ "id": id }),
    )))
}

async fn send_contact_email(
    contact: &ContactForCreate,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let email_service = match email_service().as_ref() {
        Ok(service) => service,
        Err(e) => {
            error!("Email service not configured: {}", e);
            return Ok(());
        }
    };

    email_service
        .send_contact_notification(
            &contact.name,
            &contact.email,
            contact.subject.as_deref(),
            &contact.message,
        )
        .await?;

    info!("Contact notification email sent for: {}", contact.email);
    Ok(())
}
