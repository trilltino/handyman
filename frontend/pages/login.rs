//! # Login Page
//!
//! ## Purpose
//! User login form for authentication.
//! Allows registered users to log in and access protected features.
//!
//! ## How It Works
//! 1. User enters username and password
//! 2. Form submission calls api::login()
//! 3. Backend validates credentials and sets auth cookie
//! 4. On success: Updates AuthContext and redirects to home
//! 5. On error: Shows error message
//!
//! ## Form Fields
//! - Username (text input)
//! - Password (password input)
//! - Submit button
//!
//! ## Relation to Entire Program
//! - **Route**: `/login`
//! - **API**: POST /api/login
//! - **Redirects To**: Home page on success
//! - **Updates**: AuthContext with user info

use yew::prelude::*;              // Yew framework
use web_sys::HtmlInputElement;    // DOM input element type
use yew_router::prelude::*;       // Routing for redirect
use stylist::css;                 // CSS-in-Rust styling

use crate::api;                   // API client for login request
use crate::contexts::{AuthAction, AuthContext};  // Auth state management
use crate::routes::Route;         // Routes for navigation

#[function_component]
pub fn Login() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let username = use_state(String::new);
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

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_submit = {
        let username = username.clone();
        let password = password.clone();
        let error = error.clone();
        let is_submitting = is_submitting.clone();
        let navigator = navigator.clone();
        let auth = auth.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let username_val = (*username).clone();
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

            api::login(username_val, password_val, on_success, on_error);
        })
    };

    let css = css!(r#"
        .form-container {
            max-width: 500px;
            margin: 50px auto;
            padding: 40px;
            background-color: #fff;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
        }
        .form-container h2 {
            text-align: center;
            color: #2c3e50;
            margin-bottom: 30px;
        }
        .form-group {
            margin-bottom: 20px;
        }
        .form-group label {
            display: block;
            margin-bottom: 8px;
            color: #2c3e50;
            font-weight: 500;
        }
        .form-group input {
            width: 100%;
            padding: 12px;
            border: 1px solid #ddd;
            border-radius: 4px;
            box-sizing: border-box;
        }
        .form-group input:focus {
            outline: none;
            border-color: #3498db;
        }
        .submit-button {
            width: 100%;
            padding: 14px;
            background-color: #3498db;
            color: white;
            border: none;
            border-radius: 4px;
            font-size: 16px;
            font-weight: 600;
            cursor: pointer;
        }
        .submit-button:hover:not(:disabled) {
            background-color: #2980b9;
        }
        .submit-button:disabled {
            background-color: #95a5a6;
            cursor: not-allowed;
            opacity: 0.6;
        }
        .status-message {
            padding: 12px;
            border-radius: 4px;
            margin-bottom: 20px;
        }
        .status-message.error {
            background-color: #fee;
            color: #e74c3c;
            border: 1px solid #e74c3c;
        }
        .form-footer {
            text-align: center;
            margin-top: 20px;
            color: #7f8c8d;
        }
        .form-footer a {
            color: #3498db;
            text-decoration: none;
        }
    "#);

    html! {
        <div class={css}>
            <div class="form-container">
                <h2>{ "Login" }</h2>

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
                        { if *is_submitting { "Logging in..." } else { "Login" } }
                    </button>
                </form>

                <div class="form-footer">
                    { "Don't have an account? " }
                    <Link<Route> to={Route::Register}>{ "Register here" }</Link<Route>>
                </div>
            </div>
        </div>
    }
}
