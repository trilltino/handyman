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

        let opts = RequestInit::new();
        opts.set_method("POST");
        opts.set_mode(RequestMode::Cors);
        opts.set_body(&wasm_bindgen::JsValue::from_str(&body));

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

        let status = resp.status();
        // Log to browser console for debugging
        web_sys::console::log_1(&format!("Contact form response status: {}", status).into());

        // Consider 200-299 as success
        if status >= 200 && status < 300 {
            Ok("Message sent successfully! We'll get back to you soon.".to_string())
        } else {
            Err(format!(
                "Failed to send message (status: {}). Please try again.",
                status
            ))
        }
    }
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use shared::validation::Validate;
    use shared::ContactForm;

    #[test]
    fn test_contact_form_structure() {
        let form = ContactForm {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            message: "Hello, I need help.".to_string(),
        };

        assert_eq!(form.name, "Test User");
        assert_eq!(form.email, "test@example.com");
        assert_eq!(form.message, "Hello, I need help.");
    }

    #[test]
    fn test_contact_form_validation_success() {
        let form = ContactForm {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            message: "I need a quote for plumbing work.".to_string(),
        };

        assert!(form.validate().is_ok());
    }

    #[test]
    fn test_contact_form_validation_empty_name() {
        let form = ContactForm {
            name: "".to_string(),
            email: "test@example.com".to_string(),
            message: "Hello".to_string(),
        };

        assert!(form.validate().is_err());
    }

    #[test]
    fn test_contact_form_validation_invalid_email() {
        let form = ContactForm {
            name: "John Doe".to_string(),
            email: "invalid-email".to_string(),
            message: "Hello".to_string(),
        };

        assert!(form.validate().is_err());
    }

    #[test]
    fn test_contact_form_validation_empty_message() {
        let form = ContactForm {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            message: "".to_string(),
        };

        assert!(form.validate().is_err());
    }

    #[test]
    fn test_form_trims_input() {
        // Simulate the trimming that happens in submit_contact_form
        let name = "  John Doe  ".trim().to_string();
        let email = "  john@example.com  ".trim().to_string();
        let message = "  Hello  ".trim().to_string();

        assert_eq!(name, "John Doe");
        assert_eq!(email, "john@example.com");
        assert_eq!(message, "Hello");
    }
}

// endregion: --- Tests
