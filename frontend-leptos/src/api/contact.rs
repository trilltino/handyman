//! Contact API client for submitting forms to backend.

//! Contact form API client.
//!
//! Handles contact form submission to backend.

use shared::validation::Validate;
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

    if let Err(e) = form.validate() {
        return Err(e.into());
    }

    #[cfg(feature = "ssr")]
    {
        use reqwest::Client;
        let client = Client::new();
        let response = client
            .post("http://localhost:8080/api/contact")
            .json(&form)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.status().is_success() {
            Ok("Message sent successfully! We'll get back to you soon.".to_string())
        } else {
            Err(format!("Failed to send message. Please try again."))
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        use gloo_net::http::Request;
        Request::post("/api/contact")
            .json(&form)
            .map_err(|e| format!("Failed to send: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        Ok("Message sent successfully! We'll get back to you soon.".to_string())
    }
}
