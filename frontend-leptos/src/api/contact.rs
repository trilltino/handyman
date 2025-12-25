//! Contact API client for submitting forms to backend.
//!
//! Handles contact form submission - works on both server (SSR) and client (WASM).

use shared::ContactForm;

/// Submit a contact form message to the backend.
pub async fn submit_contact_form(
    name: String,
    email: String,
    message: String,
) -> Result<String, String> {
    let form = ContactForm {
        name: name.trim().to_string(),
        email: email.trim().to_string(),
        message: message.trim().to_string(),
    };

    form.validate()?;

    // Use gloo-net for both SSR and WASM compatibility
    #[cfg(feature = "ssr")]
    {
        use reqwest::Client;
        let api_url =
            std::env::var("API_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
        let client = Client::new();
        let response = client
            .post(format!("{}/api/contact", api_url))
            .json(&form)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.status().is_success() {
            Ok("Message sent successfully! We'll get back to you soon.".to_string())
        } else {
            Err("Failed to send message. Please try again.".to_string())
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        use wasm_bindgen::JsCast;
        use web_sys::{Request, RequestInit, RequestMode, Response};

        let body =
            serde_json::to_string(&form).map_err(|e| format!("Serialization error: {}", e))?;

        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.mode(RequestMode::Cors);
        opts.body(Some(&wasm_bindgen::JsValue::from_str(&body)));

        let request = Request::new_with_str_and_init("/api/contact", &opts)
            .map_err(|_| "Failed to create request".to_string())?;

        request
            .headers()
            .set("Content-Type", "application/json")
            .map_err(|_| "Failed to set headers".to_string())?;

        let window = web_sys::window().ok_or("No window object")?;
        let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|_| "Network error".to_string())?;

        let resp: Response = resp_value
            .dyn_into()
            .map_err(|_| "Invalid response".to_string())?;

        if resp.ok() {
            Ok("Message sent successfully! We'll get back to you soon.".to_string())
        } else {
            Err("Failed to send message. Please try again.".to_string())
        }
    }
}
