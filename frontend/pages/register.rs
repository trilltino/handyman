//! # Register Page
//!
//! ## Purpose
//! User registration form for creating new accounts.
//! Allows new users to sign up for the handyman marketplace.
//!
//! ## How It Works
//! 1. User enters username, email, and password
//! 2. Form submission calls api::register()
//! 3. Backend creates user with hashed password
//! 4. On success: Sets auth cookie, updates AuthContext, redirects to home
//! 5. On error: Shows error message
//!
//! ## Form Fields
//! - Username (text input, must be unique)
//! - Email (email input, must be unique)
//! - Password (password input, will be hashed)
//! - Submit button
//!
//! ## Relation to Entire Program
//! - **Route**: `/register`
//! - **API**: POST /api/register
//! - **Redirects To**: Home page on success
//! - **Updates**: AuthContext with new user info

use yew::prelude::*;              // Yew framework
use web_sys::HtmlInputElement;    // DOM input element type
use yew_router::prelude::*;       // Routing for redirect
use stylist::css;                 // CSS-in-Rust styling

use crate::api;                   // API client for register request
use crate::contexts::{AuthAction, AuthContext};  // Auth state management
use crate::routes::Route;         // Routes for navigation

#[function_component]
pub fn Register() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let username = use_state(String::new);
    let email = use_state(String::new);
    let password = use_state(String::new);
    let error = use_state(|| None::<String>);
    let is_submitting = use_state(|| false);
    let navigator = use_navigator().unwrap();

    let on_username_change = {
        let username = username.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_submit = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let error = error.clone();
        let is_submitting = is_submitting.clone();
        let navigator = navigator.clone();
        let auth = auth.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let username_val = (*username).clone();
            let email_val = (*email).clone();
            let password_val = (*password).clone();
            let error = error.clone();
            let is_submitting = is_submitting.clone();
            let navigator = navigator.clone();
            let auth = auth.clone();

            is_submitting.set(true);
            error.set(None);

            let on_success = Callback::from(move |response: api::AuthResponse| {
                if let Some(user) = response.user {
                    auth.dispatch(AuthAction::Login(user));
                    navigator.push(&Route::Home);
                }
            });

            let on_error = {
                let error = error.clone();
                let is_submitting = is_submitting.clone();
                Callback::from(move |err: String| {
                    error.set(Some(err));
                    is_submitting.set(false);
                })
            };

            api::register(username_val, email_val, password_val, on_success, on_error);
        })
    };

    let css = css!(r#"
        .form-container { max-width: 500px; margin: 50px auto; padding: 40px; background: #fff; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        .form-group { margin-bottom: 20px; }
        .form-group label { display: block; margin-bottom: 8px; font-weight: 500; }
        .form-group input, .form-group textarea, .form-group select { width: 100%; padding: 12px; border: 1px solid #ddd; border-radius: 4px; box-sizing: border-box; }
        .submit-button { width: 100%; padding: 14px; background: #3498db; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 16px; }
        .submit-button:hover { background: #2980b9; }
        .submit-button:disabled { background: #95a5a6; cursor: not-allowed; }
        .status-message.error { background: #fee; color: #e74c3c; padding: 12px; border-radius: 4px; margin-bottom: 20px; }
        .status-message.success { background: #efe; color: #2ecc71; padding: 12px; border-radius: 4px; margin-bottom: 20px; }
        .form-footer { margin-top: 20px; text-align: center; }
    "#);

    html! {
        <div class={css}>
            <div class="form-container">
                <h2>{ "Register" }</h2>

                if let Some(err) = (*error).clone() {
                    <div class="status-message error">{ err }</div>
                }

                <form onsubmit={on_submit}>
                    <div class="form-group">
                        <label for="username">{ "Username" }</label>
                        <input
                            type="text"
                            id="username"
                            value={(*username).clone()}
                            onchange={on_username_change}
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
                        <label for="password">{ "Password" }</label>
                        <input
                            type="password"
                            id="password"
                            value={(*password).clone()}
                            onchange={on_password_change}
                            required=true
                            disabled={*is_submitting}
                        />
                    </div>

                    <button
                        type="submit"
                        class="submit-button"
                        disabled={*is_submitting}
                    >
                        { if *is_submitting { "Registering..." } else { "Register" } }
                    </button>
                </form>

                <div class="form-footer">
                    { "Already have an account? " }
                    <Link<Route> to={Route::Login}>{ "Login here" }</Link<Route>>
                </div>
            </div>
        </div>
    }
}
