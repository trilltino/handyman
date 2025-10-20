//! # Stripe Payment Integration
//!
//! ## Purpose
//! WASM bindings for Stripe.js payment processing.
//! Handles payment checkout redirects for booking payments.
//!
//! ## How It Works
//! 1. Stripe.js loaded from CDN in index.html
//! 2. WASM bindings create Stripe instance
//! 3. Opens Stripe checkout session with booking details
//! 4. Redirects to Stripe-hosted payment page
//! 5. User completes payment on Stripe
//! 6. Redirects back to success/cancel URL
//!
//! ## Integration Status
//! - ✅ Stripe.js bindings complete
//! - ⚠️ Need to create Stripe Price IDs
//! - ⚠️ Need backend webhook for payment confirmation
//!
//! ## Relation to Entire Program
//! - **Used By**: Booking page after booking creation
//! - **Requires**: Stripe publishable key, Price IDs
//! - **Future**: Webhook handler in backend to confirm payment

use wasm_bindgen::prelude::*;  // WASM-JS bindings
use web_sys::window;            // Browser window object
use js_sys;                     // JavaScript standard library

/// External JavaScript bindings for Stripe.js
#[wasm_bindgen]
extern "C" {
    /// Stripe constructor from Stripe.js global
    #[wasm_bindgen(js_name = Stripe)]
    fn stripe_constructor(key: &str) -> StripeInstance;

    /// Stripe instance type
    pub type StripeInstance;

    /// Redirect to Stripe checkout
    #[wasm_bindgen(method, js_name = redirectToCheckout)]
    pub fn redirect_to_checkout(this: &StripeInstance, options: JsValue) -> js_sys::Promise;
}

pub fn test_stripe_button() {
    // This is a simple test to show Stripe is loaded
    let window = window().expect("should have window");
    let stripe_exists = js_sys::Reflect::has(&window, &JsValue::from_str("Stripe"))
        .unwrap_or(false);

    if stripe_exists {
        web_sys::console::log_1(&"✅ Stripe.js loaded successfully!".into());
    } else {
        web_sys::console::error_1(&"❌ Stripe.js not loaded!".into());
    }
}

pub fn open_stripe_checkout(publishable_key: &str, _price_cents: i32) {
    let publishable_key = publishable_key.to_string();
    let _ = wasm_bindgen_futures::spawn_local(async move {
        let stripe = stripe_constructor(&publishable_key);

        // Create checkout session options
        let options = js_sys::Object::new();

        // Line items
        let line_items = js_sys::Array::new();
        let item = js_sys::Object::new();
        js_sys::Reflect::set(&item, &"price".into(), &"price_YOUR_PRICE_ID".into()).unwrap();
        js_sys::Reflect::set(&item, &"quantity".into(), &1.into()).unwrap();
        line_items.push(&item);

        js_sys::Reflect::set(&options, &"lineItems".into(), &line_items).unwrap();
        js_sys::Reflect::set(&options, &"mode".into(), &"payment".into()).unwrap();
        js_sys::Reflect::set(&options, &"successUrl".into(), &"http://localhost:8080/success".into()).unwrap();
        js_sys::Reflect::set(&options, &"cancelUrl".into(), &"http://localhost:8080/booking".into()).unwrap();

        match wasm_bindgen_futures::JsFuture::from(stripe.redirect_to_checkout(options.into())).await {
            Ok(_) => web_sys::console::log_1(&"Redirected to Stripe checkout".into()),
            Err(e) => web_sys::console::error_1(&e),
        }
    });
}
