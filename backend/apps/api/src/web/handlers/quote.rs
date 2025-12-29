//! Quote API handlers.
//!
//! Handles quote creation, retrieval, and management.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use lib_core::model::ModelManager;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Line item in a quote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteItem {
    pub description: String,
    pub quantity: i32,
    pub unit_price: i32, // In cents
}

/// Quote creation request
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CreateQuoteRequest {
    pub customer_name: String,
    pub customer_email: String,
    pub customer_phone: Option<String>,
    pub title: String,
    pub items: Vec<QuoteItem>,
    pub valid_days: Option<i32>, // Days until expiry, default 30
}

/// Quote response
#[derive(Debug, Serialize)]
pub struct QuoteResponse {
    pub id: String,
    pub title: String,
    pub items: Vec<QuoteItem>,
    pub subtotal: i32,
    pub total: i32,
    pub status: String,
    pub valid_until: Option<String>,
    pub created_at: String,
    pub accept_url: String,
}

/// Quote templates response
#[derive(Debug, Serialize)]
pub struct QuoteTemplateResponse {
    pub id: i32,
    pub name: String,
    pub service_type: String,
    pub items: Vec<QuoteItem>,
}

/// Create a new quote
pub async fn create_quote(
    State(_mm): State<ModelManager>,
    Json(req): Json<CreateQuoteRequest>,
) -> Result<Json<QuoteResponse>, (StatusCode, String)> {
    // Calculate totals
    let subtotal: i32 = req.items.iter().map(|i| i.quantity * i.unit_price).sum();
    let total = subtotal; // No discount for now

    // Generate a unique ID for the quote
    let quote_id = Uuid::new_v4().to_string();

    // TODO: Save to database
    // For now, return a mock response
    let response = QuoteResponse {
        id: quote_id.clone(),
        title: req.title,
        items: req.items,
        subtotal,
        total,
        status: "draft".to_string(),
        valid_until: Some(
            chrono::Utc::now()
                .checked_add_signed(chrono::Duration::days(req.valid_days.unwrap_or(30) as i64))
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or_default(),
        ),
        created_at: chrono::Utc::now().to_rfc3339(),
        accept_url: format!("/quote/{}/accept", quote_id),
    };

    Ok(Json(response))
}

/// Get quote by ID
pub async fn get_quote(
    State(_mm): State<ModelManager>,
    Path(quote_id): Path<String>,
) -> Result<Json<QuoteResponse>, (StatusCode, String)> {
    // TODO: Fetch from database
    // For now, return a mock response
    let response = QuoteResponse {
        id: quote_id.clone(),
        title: "Sample Quote".to_string(),
        items: vec![
            QuoteItem {
                description: "Call-out fee".to_string(),
                quantity: 1,
                unit_price: 3000,
            },
            QuoteItem {
                description: "Labour (1 hour)".to_string(),
                quantity: 1,
                unit_price: 4500,
            },
        ],
        subtotal: 7500,
        total: 7500,
        status: "sent".to_string(),
        valid_until: Some("2025-01-22".to_string()),
        created_at: chrono::Utc::now().to_rfc3339(),
        accept_url: format!("/quote/{}/accept", quote_id),
    };

    Ok(Json(response))
}

/// Get all quote templates
pub async fn get_quote_templates(
    State(_mm): State<ModelManager>,
) -> Result<Json<Vec<QuoteTemplateResponse>>, (StatusCode, String)> {
    // TODO: Fetch from database
    // For now, return mock templates
    let templates = vec![
        QuoteTemplateResponse {
            id: 1,
            name: "Leaky Tap Repair".to_string(),
            service_type: "plumbing".to_string(),
            items: vec![
                QuoteItem {
                    description: "Call-out fee".to_string(),
                    quantity: 1,
                    unit_price: 3000,
                },
                QuoteItem {
                    description: "Labour (1 hour)".to_string(),
                    quantity: 1,
                    unit_price: 4500,
                },
                QuoteItem {
                    description: "Materials".to_string(),
                    quantity: 1,
                    unit_price: 500,
                },
            ],
        },
        QuoteTemplateResponse {
            id: 2,
            name: "Light Fitting Installation".to_string(),
            service_type: "electrical".to_string(),
            items: vec![
                QuoteItem {
                    description: "Call-out fee".to_string(),
                    quantity: 1,
                    unit_price: 3000,
                },
                QuoteItem {
                    description: "Labour (1 hour)".to_string(),
                    quantity: 1,
                    unit_price: 5000,
                },
                QuoteItem {
                    description: "Standard light fitting".to_string(),
                    quantity: 1,
                    unit_price: 2500,
                },
            ],
        },
        QuoteTemplateResponse {
            id: 3,
            name: "IKEA Furniture Assembly (Small)".to_string(),
            service_type: "assembly".to_string(),
            items: vec![QuoteItem {
                description: "Assembly service".to_string(),
                quantity: 1,
                unit_price: 4500,
            }],
        },
        QuoteTemplateResponse {
            id: 4,
            name: "TV Wall Mount".to_string(),
            service_type: "general".to_string(),
            items: vec![QuoteItem {
                description: "TV mounting service".to_string(),
                quantity: 1,
                unit_price: 6500,
            }],
        },
        QuoteTemplateResponse {
            id: 5,
            name: "Door Hanging".to_string(),
            service_type: "carpentry".to_string(),
            items: vec![
                QuoteItem {
                    description: "Call-out fee".to_string(),
                    quantity: 1,
                    unit_price: 3000,
                },
                QuoteItem {
                    description: "Door hanging labour".to_string(),
                    quantity: 1,
                    unit_price: 7500,
                },
            ],
        },
    ];

    Ok(Json(templates))
}

/// Accept a quote (convert to booking)
pub async fn accept_quote(
    State(_mm): State<ModelManager>,
    Path(quote_id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // TODO: Update quote status and create booking
    Ok(Json(serde_json::json!({
        "success": true,
        "quote_id": quote_id,
        "message": "Quote accepted! We'll be in touch to confirm your booking.",
        "booking_id": Uuid::new_v4().to_string()
    })))
}

/// Instant quote calculator
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct InstantQuoteRequest {
    pub service_type: String,
    pub description: Option<String>,
    pub urgency: Option<String>, // "same_day", "within_3_days", "flexible"
}

#[derive(Debug, Serialize)]
pub struct InstantQuoteResponse {
    pub estimate_low: i32,
    pub estimate_high: i32,
    pub urgency_fee: i32,
    pub message: String,
}

/// Get instant quote estimate (public endpoint)
pub async fn get_instant_quote(
    Json(req): Json<InstantQuoteRequest>,
) -> Result<Json<InstantQuoteResponse>, (StatusCode, String)> {
    // Base prices by service type (in cents)
    let (base_low, base_high) = match req.service_type.to_lowercase().as_str() {
        "plumbing" => (4500, 12000),
        "electrical" => (5000, 15000),
        "carpentry" => (6000, 18000),
        "assembly" => (3500, 8500),
        "painting" => (8000, 25000),
        "general" | _ => (4000, 10000),
    };

    // Urgency fee
    let urgency_fee = match req.urgency.as_deref() {
        Some("same_day") => 1500,   // £15 extra
        Some("within_3_days") => 0, // No extra
        Some("flexible") => -500,   // £5 discount
        _ => 0,
    };

    let response = InstantQuoteResponse {
        estimate_low: base_low + urgency_fee,
        estimate_high: base_high + urgency_fee,
        urgency_fee,
        message: format!(
            "Estimated cost for {} work: £{:.2} - £{:.2}",
            req.service_type,
            (base_low + urgency_fee) as f64 / 100.0,
            (base_high + urgency_fee) as f64 / 100.0
        ),
    };

    Ok(Json(response))
}
