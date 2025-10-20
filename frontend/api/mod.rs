//! # API Client Module
//!
//! ## Purpose
//! Frontend HTTP client for communicating with backend API.
//! Provides async functions for auth, booking, and contact operations.
//!
//! ## API Functions
//! - **register**: Create new user account
//! - **login**: Authenticate user
//! - **logout**: Clear auth session
//! - **create_booking**: Submit booking form
//! - **send_contact**: Submit contact form
//!
//! ## How It Works
//! 1. Functions accept data + callbacks (on_success, on_error)
//! 2. Spawn async task using spawn_local (WASM async)
//! 3. Make HTTP request to backend using gloo
//! 4. Parse JSON response
//! 5. Call success or error callback with result
//!
//! ## Relation to Entire Program
//! - **Used By**: Page components (Login, Register, Booking, Contact)
//! - **Calls**: Backend API at localhost:3000
//! - **Auth**: Cookies automatically sent with requests

use gloo::net::http::Request;        // HTTP client for WASM
use serde::{Deserialize, Serialize};  // JSON serialization
use wasm_bindgen_futures::spawn_local;  // Async execution in WASM
use yew::Callback;                    // Yew callback type

/// Backend API base URL
const API_BASE_URL: &str = "http://localhost:3000/api";

/// Authentication response from backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub success: bool,        // Operation succeeded
    pub message: String,      // Human-readable message
    pub user: Option<UserInfo>, // User info if successful
}

/// User information returned from auth endpoints
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,         // User ID
    pub username: String, // Username
    pub email: String,   // Email address
}

/// Generic API response (booking, contact)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,   // Operation succeeded
    pub message: String, // Human-readable message
}

// Auth API
pub fn register(
    username: String,
    email: String,
    password: String,
    on_success: Callback<AuthResponse>,
    on_error: Callback<String>,
) {
    spawn_local(async move {
        let body = serde_json::json!({
            "username": username,
            "email": email,
            "password": password
        });

        match Request::post(&format!("{}/register", API_BASE_URL))
            .json(&body)
            .unwrap()
            .send()
            .await
        {
            Ok(response) => {
                if response.ok() {
                    match response.json::<AuthResponse>().await {
                        Ok(data) => on_success.emit(data),
                        Err(_) => on_error.emit("Failed to parse response".to_string()),
                    }
                } else {
                    on_error.emit("Registration failed".to_string());
                }
            }
            Err(_) => on_error.emit("Network error".to_string()),
        }
    });
}

pub fn login(
    username: String,
    password: String,
    on_success: Callback<AuthResponse>,
    on_error: Callback<String>,
) {
    spawn_local(async move {
        let body = serde_json::json!({
            "username": username,
            "password": password
        });

        match Request::post(&format!("{}/login", API_BASE_URL))
            .json(&body)
            .unwrap()
            .send()
            .await
        {
            Ok(response) => {
                if response.ok() {
                    match response.json::<AuthResponse>().await {
                        Ok(data) => on_success.emit(data),
                        Err(_) => on_error.emit("Failed to parse response".to_string()),
                    }
                } else {
                    on_error.emit("Login failed. Check your credentials.".to_string());
                }
            }
            Err(_) => on_error.emit("Network error".to_string()),
        }
    });
}

pub fn logout(on_success: Callback<()>, on_error: Callback<String>) {
    spawn_local(async move {
        let body = serde_json::json!({});

        match Request::post(&format!("{}/logout", API_BASE_URL))
            .json(&body)
            .unwrap()
            .send()
            .await
        {
            Ok(response) => {
                if response.ok() {
                    on_success.emit(());
                } else {
                    on_error.emit("Logout failed".to_string());
                }
            }
            Err(_) => on_error.emit("Network error".to_string()),
        }
    });
}

// Booking API
pub fn create_booking(
    location: String,
    work_type: String,
    description: String,
    name: String,
    email: String,
    phone: String,
    on_success: Callback<ApiResponse>,
    on_error: Callback<String>,
) {
    spawn_local(async move {
        let body = serde_json::json!({
            "location": location,
            "work_type": work_type,
            "description": description,
            "name": name,
            "email": email,
            "phone": phone
        });

        web_sys::console::log_1(&format!("Sending booking request: {:?}", body).into());

        match Request::post(&format!("{}/booking", API_BASE_URL))
            .json(&body)
            .unwrap()
            .send()
            .await
        {
            Ok(response) => {
                web_sys::console::log_1(&format!("Response status: {}", response.status()).into());
                if response.ok() {
                    match response.json::<ApiResponse>().await {
                        Ok(data) => {
                            web_sys::console::log_1(&format!("Success: {:?}", data).into());
                            on_success.emit(data)
                        },
                        Err(e) => {
                            web_sys::console::error_1(&format!("Failed to parse response: {:?}", e).into());
                            on_error.emit("Failed to parse response".to_string())
                        },
                    }
                } else {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_else(|_| "No error message".to_string());
                    web_sys::console::error_1(&format!("Booking failed - Status: {}, Body: {}", status, text).into());
                    on_error.emit(format!("Booking failed: {}", text));
                }
            }
            Err(e) => {
                web_sys::console::error_1(&format!("Network error: {:?}", e).into());
                on_error.emit("Network error".to_string())
            },
        }
    });
}

// Contact API
pub fn send_contact(
    name: String,
    email: String,
    message: String,
    on_success: Callback<ApiResponse>,
    on_error: Callback<String>,
) {
    spawn_local(async move {
        let body = serde_json::json!({
            "name": name,
            "email": email,
            "message": message
        });

        match Request::post(&format!("{}/contact", API_BASE_URL))
            .json(&body)
            .unwrap()
            .send()
            .await
        {
            Ok(response) => {
                if response.ok() {
                    match response.json::<ApiResponse>().await {
                        Ok(data) => on_success.emit(data),
                        Err(_) => on_error.emit("Failed to parse response".to_string()),
                    }
                } else {
                    on_error.emit("Message sending failed".to_string());
                }
            }
            Err(_) => on_error.emit("Network error".to_string()),
        }
    });
}
