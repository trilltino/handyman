//! Stripe webhook handler for payment events.
//!
//! This module processes incoming Stripe webhook events to handle
//! payment lifecycle events (succeeded, failed, etc.).
//!
//! # Security
//!
//! - Webhook signature verification using `STRIPE_WEBHOOK_SECRET`
//! - Events are verified before processing
//! - Invalid signatures result in 400 Bad Request
//!
//! # Environment Variables
//!
//! - `STRIPE_WEBHOOK_SECRET`: Secret for verifying webhook signatures

use axum::body::Bytes;
use axum::http::HeaderMap;
use lib_web::error::Error;
use serde_json::json;
use shared::ApiResponse;
use std::env;
use tracing::{error, info};

/// Handles incoming Stripe webhook events
#[utoipa::path(
    post,
    path = "/api/webhooks/stripe",
    tag = "payments",
    responses(
        (status = 200, description = "Webhook processed", body = serde_json::Value),
        (status = 400, description = "Invalid signature", body = serde_json::Value),
        (status = 500, description = "Server error", body = serde_json::Value)
    )
)]
pub async fn stripe_webhook_handler(
    headers: HeaderMap,
    body: Bytes,
) -> Result<axum::Json<ApiResponse<serde_json::Value>>, Error> {
    let webhook_secret = env::var("STRIPE_WEBHOOK_SECRET")
        .unwrap_or_else(|_| "whsec_test".to_string());

    let signature = headers
        .get("stripe-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            error!("Missing Stripe signature header");
            Error::ValidationError("Missing Stripe signature".into())
        })?;

    let body_str = std::str::from_utf8(&body)
        .map_err(|_| Error::ValidationError("Invalid body encoding".into()))?;

    let event = stripe::Webhook::construct_event(
        body_str,
        signature,
        &webhook_secret,
    )
    .map_err(|e| {
        error!("Webhook verification failed: {}", e);
        Error::ValidationError("Invalid webhook signature".into())
    })?;

    info!("Received verified webhook event: {:?}", event.type_);

    use stripe::{EventType, EventObject};
    
    match event.type_ {
        EventType::PaymentIntentSucceeded => {
            if let EventObject::PaymentIntent(payment_intent) = event.data.object {
                info!("Payment succeeded. ID: {:?}", payment_intent.id);
            }
        }
        EventType::PaymentIntentPaymentFailed => {
            if let EventObject::PaymentIntent(payment_intent) = event.data.object {
                error!("Payment failed. ID: {:?}", payment_intent.id);
            }
        }
        _ => {
            info!("Unhandled webhook event type: {:?}", event.type_);
        }
    }

    Ok(axum::Json(ApiResponse::success(
        "Webhook processed",
        json!({"received": true}),
    )))
}
