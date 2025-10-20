//! # Booking Handler
//!
//! ## Purpose
//! Handles booking form submissions from customers requesting handyman services.
//!
//! ## Endpoint
//! - **POST /api/bookings**: Create new booking
//!
//! ## Process Flow
//! 1. Receive booking form data (location, work type, customer info)
//! 2. Create/find customer by email (prevents duplicates)
//! 3. Calculate price based on work type
//! 4. Create booking in database with Pending status
//! 5. (Future) Create Stripe payment intent
//! 6. (Future) Send confirmation email
//!
//! ## Relation to Entire Program
//! - **Called By**: Frontend booking form submission
//! - **Calls**: BookingRepository → CustomerRepository → Database
//! - **Future**: Will integrate Stripe for payment processing

use axum::{extract::State, http::StatusCode, Json};  // Axum HTTP types
use serde_json::json;                                 // JSON response helper

use crate::db::DbPool;                  // Database connection pool
use crate::models::NewBooking;          // Booking DTO
use crate::repositories::BookingRepository;  // Booking database operations

/// Handle booking submission
/// Route: POST /api/bookings
///
/// # Process
/// 1. Extract booking details from request
/// 2. Get database connection
/// 3. Create booking (also creates/finds customer)
/// 4. Return success with booking ID and price
///
/// # Future Enhancements
/// - Create Stripe PaymentIntent and return client_secret
/// - Send confirmation email to customer
/// - Send notification to handyman matching service
/// - Schedule job in queue for handyman assignment
pub async fn handle_booking(
    State(pool): State<DbPool>,
    Json(payload): Json<NewBooking>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, String)> {
    tracing::info!("Received booking request from: {}", payload.email);

    // Get connection from pool
    let client = pool
        .get()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Save to database (creates customer if needed, calculates price)
    tracing::info!("Attempting to create booking with payload: {:?}", payload);
    match BookingRepository::create(&client, &payload).await {
        Ok(booking) => {
            tracing::info!("Booking created successfully with ID: {}", booking.id);

            // TODO: Create Stripe PaymentIntent here
            // let payment_intent = stripe::PaymentIntent::create(...)?;
            // TODO: Send confirmation email
            // email::send_booking_confirmation(&booking, &customer)?;

            Ok((
                StatusCode::CREATED,
                Json(json!({
                    "success": true,
                    "message": "Booking created successfully! We'll contact you soon.",
                    "booking_id": booking.id,
                    "price_cents": booking.price_cents,
                    // TODO: Return payment client_secret for frontend
                    // "client_secret": payment_intent.client_secret,
                }))
            ))
        }
        Err(err) => {
            tracing::error!("Failed to create booking: {:?}", err);
            tracing::error!("Error chain: {:#}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create booking: {}", err),
            ))
        }
    }
}
