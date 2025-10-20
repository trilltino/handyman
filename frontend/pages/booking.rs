//! # Booking Page
//!
//! ## Purpose
//! Service booking form for customers to request handyman services.
//! Collects booking details, customer info, and initiates payment.
//!
//! ## How It Works
//! 1. User fills form (work type, location, description, contact info)
//! 2. Work type selection auto-calculates price
//! 3. Form submission calls api::create_booking()
//! 4. Backend creates booking and customer records
//! 5. (Future) Redirects to Stripe checkout for payment
//!
//! ## Form Fields
//! - Work Type (dropdown): Plumbing, Electrical, Carpentry, etc.
//! - Location (text): Service address
//! - Description (textarea): Work details
//! - Name, Email, Phone: Customer contact info
//!
//! ## Pricing
//! - Plumbing: €150, Electrical: €180, Carpentry: €120
//! - Painting: €100, General Repair: €80, Other: €100
//!
//! ## Relation to Entire Program
//! - **Route**: `/booking`
//! - **API**: POST /api/bookings
//! - **Future**: Stripe payment integration

use yew::prelude::*;                              // Yew framework
use web_sys::{HtmlInputElement, HtmlSelectElement}; // DOM element types
use stylist::css;                                  // CSS-in-Rust styling

use crate::api;     // API client for booking submission
use crate::stripe;  // Stripe payment (future)

#[derive(Clone, PartialEq)]
pub enum WorkType {
    Plumbing,
    Electrical,
    Carpentry,
    Painting,
    GeneralRepair,
    Other,
}

impl WorkType {
    fn as_str(&self) -> &str {
        match self {
            WorkType::Plumbing => "Plumbing",
            WorkType::Electrical => "Electrical",
            WorkType::Carpentry => "Carpentry",
            WorkType::Painting => "Painting",
            WorkType::GeneralRepair => "General Repair",
            WorkType::Other => "Other",
        }
    }

    fn all() -> Vec<Self> {
        vec![
            WorkType::Plumbing,
            WorkType::Electrical,
            WorkType::Carpentry,
            WorkType::Painting,
            WorkType::GeneralRepair,
            WorkType::Other,
        ]
    }
}

#[function_component]
pub fn Booking() -> Html {
    let location = use_state(String::new);
    let work_type = use_state(|| WorkType::Plumbing);
    let description = use_state(String::new);
    let name = use_state(String::new);
    let email = use_state(String::new);
    let phone = use_state(String::new);
    let status = use_state(|| None::<String>);
    let is_error = use_state(|| false);
    let is_submitting = use_state(|| false);

    let on_location_change = {
        let location = location.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            location.set(input.value());
        })
    };

    let on_work_type_change = {
        let work_type = work_type.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            let value = match select.value().as_str() {
                "Plumbing" => WorkType::Plumbing,
                "Electrical" => WorkType::Electrical,
                "Carpentry" => WorkType::Carpentry,
                "Painting" => WorkType::Painting,
                "General Repair" => WorkType::GeneralRepair,
                _ => WorkType::Other,
            };
            work_type.set(value);
        })
    };

    let on_description_change = {
        let description = description.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            description.set(input.value());
        })
    };

    let on_name_change = {
        let name = name.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_phone_change = {
        let phone = phone.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            phone.set(input.value());
        })
    };

    let on_test_stripe = {
        Callback::from(move |_: MouseEvent| {
            stripe::test_stripe_button();
            // Test with publishable key (replace with your actual key)
            let pk = "pk_test_YOUR_KEY_HERE";
            stripe::open_stripe_checkout(pk, 15000);
        })
    };

    let on_submit = {
        let location = location.clone();
        let work_type = work_type.clone();
        let description = description.clone();
        let name = name.clone();
        let email = email.clone();
        let phone = phone.clone();
        let status = status.clone();
        let is_error = is_error.clone();
        let is_submitting = is_submitting.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let location_val = (*location).clone();
            let work_type_val = (*work_type).as_str().to_string();
            let description_val = (*description).clone();
            let name_val = (*name).clone();
            let email_val = (*email).clone();
            let phone_val = (*phone).clone();
            let status = status.clone();
            let is_error = is_error.clone();
            let is_submitting = is_submitting.clone();
            let location = location.clone();
            let description = description.clone();
            let name = name.clone();
            let email = email.clone();
            let phone = phone.clone();

            is_submitting.set(true);
            status.set(None);

            let on_success = {
                let status = status.clone();
                let is_error = is_error.clone();
                let is_submitting = is_submitting.clone();
                let location = location.clone();
                let description = description.clone();
                let name = name.clone();
                let email = email.clone();
                let phone = phone.clone();
                Callback::from(move |response: api::ApiResponse| {
                    status.set(Some(response.message));
                    is_error.set(false);
                    is_submitting.set(false);
                    location.set(String::new());
                    description.set(String::new());
                    name.set(String::new());
                    email.set(String::new());
                    phone.set(String::new());
                })
            };

            let on_error = {
                let status = status.clone();
                let is_error = is_error.clone();
                let is_submitting = is_submitting.clone();
                Callback::from(move |err: String| {
                    status.set(Some(err));
                    is_error.set(true);
                    is_submitting.set(false);
                })
            };

            api::create_booking(
                location_val,
                work_type_val,
                description_val,
                name_val,
                email_val,
                phone_val,
                on_success,
                on_error,
            );
        })
    };

    let css = css!(r#"
        .form-container { max-width: 500px; margin: 50px auto; padding: 40px; background: #fff; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        .form-group { margin-bottom: 20px; }
        .form-group label { display: block; margin-bottom: 8px; font-weight: 500; }
        .form-group input, .form-group textarea, .form-group select { width: 100%; padding: 12px; border: 1px solid #ddd; border-radius: 4px; box-sizing: border-box; }
        .form-group textarea { min-height: 120px; resize: vertical; }
        .submit-button { width: 100%; padding: 14px; background: #3498db; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 16px; margin-bottom: 10px; }
        .submit-button:hover { background: #2980b9; }
        .submit-button:disabled { background: #95a5a6; cursor: not-allowed; }
        .stripe-button { width: 100%; padding: 14px; background: #635bff; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 16px; }
        .stripe-button:hover { background: #4f46d6; }
        .status-message.error { background: #fee; color: #e74c3c; padding: 12px; border-radius: 4px; margin-bottom: 20px; }
        .status-message.success { background: #efe; color: #2ecc71; padding: 12px; border-radius: 4px; margin-bottom: 20px; }
    "#);

    html! {
        <div class={css}>
            <div class="form-container">
                <h2>{ "Book a Service" }</h2>

                if let Some(msg) = (*status).clone() {
                    <div class={if *is_error { "status-message error" } else { "status-message success" }}>
                        { msg }
                    </div>
                }

                <form onsubmit={on_submit}>
                    <div class="form-group">
                        <label for="location">{ "Location/Address" }</label>
                        <input
                            type="text"
                            id="location"
                            value={(*location).clone()}
                            onchange={on_location_change}
                            required=true
                            disabled={*is_submitting}
                            placeholder="Enter your address"
                        />
                    </div>

                    <div class="form-group">
                        <label for="work_type">{ "Type of Work" }</label>
                        <select
                            id="work_type"
                            onchange={on_work_type_change}
                            disabled={*is_submitting}
                        >
                            {
                                WorkType::all().into_iter().map(|wt| {
                                    let value = wt.as_str().to_string();
                                    let label = wt.as_str().to_string();
                                    html! {
                                        <option value={value} selected={*work_type == wt}>
                                            { label }
                                        </option>
                                    }
                                }).collect::<Html>()
                            }
                        </select>
                    </div>

                    <div class="form-group">
                        <label for="description">{ "Description" }</label>
                        <textarea
                            id="description"
                            value={(*description).clone()}
                            onchange={on_description_change}
                            required=true
                            disabled={*is_submitting}
                            placeholder="Describe the work needed..."
                        />
                    </div>

                    <div class="form-group">
                        <label for="name">{ "Your Name" }</label>
                        <input
                            type="text"
                            id="name"
                            value={(*name).clone()}
                            onchange={on_name_change}
                            required=true
                            disabled={*is_submitting}
                        />
                    </div>

                    <div class="form-group">
                        <label for="email">{ "Email" }</label>
                        <input
                            type="email"
                            id="email"
                            value={(*email).clone()}
                            onchange={on_email_change}
                            required=true
                            disabled={*is_submitting}
                        />
                    </div>

                    <div class="form-group">
                        <label for="phone">{ "Phone Number" }</label>
                        <input
                            type="tel"
                            id="phone"
                            value={(*phone).clone()}
                            onchange={on_phone_change}
                            required=true
                            disabled={*is_submitting}
                        />
                    </div>

                    <button
                        type="submit"
                        class="submit-button"
                        disabled={*is_submitting}
                    >
                        { if *is_submitting { "Submitting..." } else { "Book Service" } }
                    </button>
                </form>

                <button
                    type="button"
                    class="stripe-button"
                    onclick={on_test_stripe}
                >
                    { "🔒 Test Stripe Payment ($150)" }
                </button>
            </div>
        </div>
    }
}
