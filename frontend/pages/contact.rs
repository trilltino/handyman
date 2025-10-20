//! # Contact Page
//!
//! ## Purpose
//! Contact form for visitors to send messages/inquiries.
//! Allows customers to get in touch with questions or feedback.
//!
//! ## How It Works
//! 1. User enters name, email, and message
//! 2. Form submission calls api::send_contact()
//! 3. Backend saves message to database with "new" status
//! 4. Shows success/error message to user
//! 5. (Future) Sends email notification to admin
//!
//! ## Form Fields
//! - Name (text input)
//! - Email (email input)
//! - Message (textarea)
//! - Submit button
//!
//! ## Relation to Entire Program
//! - **Route**: `/contact`
//! - **API**: POST /api/contact
//! - **Public**: No authentication required

use yew::prelude::*;           // Yew framework
use web_sys::HtmlInputElement;  // DOM input element type
use stylist::css;               // CSS-in-Rust styling

use crate::api;  // API client for contact submission

#[function_component]
pub fn Contact() -> Html {
    let name = use_state(String::new);
    let email = use_state(String::new);
    let message = use_state(String::new);
    let status = use_state(|| None::<String>);
    let is_error = use_state(|| false);
    let is_submitting = use_state(|| false);

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

    let on_message_change = {
        let message = message.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            message.set(input.value());
        })
    };

    let on_submit = {
        let name = name.clone();
        let email = email.clone();
        let message = message.clone();
        let status = status.clone();
        let is_error = is_error.clone();
        let is_submitting = is_submitting.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let name_val = (*name).clone();
            let email_val = (*email).clone();
            let message_val = (*message).clone();
            let status = status.clone();
            let is_error = is_error.clone();
            let is_submitting = is_submitting.clone();
            let name = name.clone();
            let email = email.clone();
            let message = message.clone();

            is_submitting.set(true);
            status.set(None);

            let on_success = {
                let status = status.clone();
                let is_error = is_error.clone();
                let is_submitting = is_submitting.clone();
                let name = name.clone();
                let email = email.clone();
                let message = message.clone();
                Callback::from(move |response: api::ApiResponse| {
                    status.set(Some(response.message));
                    is_error.set(false);
                    is_submitting.set(false);
                    // Clear form
                    name.set(String::new());
                    email.set(String::new());
                    message.set(String::new());
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

            api::send_contact(name_val, email_val, message_val, on_success, on_error);
        })
    };

    let css = css!(r#"
        .form-container { max-width: 500px; margin: 50px auto; padding: 40px; background: #fff; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        .form-group { margin-bottom: 20px; }
        .form-group label { display: block; margin-bottom: 8px; font-weight: 500; }
        .form-group input, .form-group textarea, .form-group select { width: 100%; padding: 12px; border: 1px solid #ddd; border-radius: 4px; box-sizing: border-box; }
        .form-group textarea { min-height: 120px; resize: vertical; }
        .submit-button { width: 100%; padding: 14px; background: #3498db; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 16px; }
        .submit-button:hover { background: #2980b9; }
        .submit-button:disabled { background: #95a5a6; cursor: not-allowed; }
        .status-message.error { background: #fee; color: #e74c3c; padding: 12px; border-radius: 4px; margin-bottom: 20px; }
        .status-message.success { background: #efe; color: #2ecc71; padding: 12px; border-radius: 4px; margin-bottom: 20px; }
    "#);

    html! {
        <div class={css}>
            <div class="form-container">
                <h2>{ "Contact Us" }</h2>

                if let Some(msg) = (*status).clone() {
                    <div class={if *is_error { "status-message error" } else { "status-message success" }}>
                        { msg }
                    </div>
                }

                <form onsubmit={on_submit}>
                    <div class="form-group">
                        <label for="name">{ "Name" }</label>
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
                        <label for="message">{ "Message" }</label>
                        <textarea
                            id="message"
                            value={(*message).clone()}
                            onchange={on_message_change}
                            required=true
                            disabled={*is_submitting}
                        />
                    </div>

                    <button
                        type="submit"
                        class="submit-button"
                        disabled={*is_submitting}
                    >
                        { if *is_submitting { "Sending..." } else { "Send Message" } }
                    </button>
                </form>
            </div>
        </div>
    }
}
