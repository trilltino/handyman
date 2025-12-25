//! Stripe API client.
//!
//! Fetches Stripe configuration from backend.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeConfig {
    pub product_id: String,
    pub public_key: String,
}

pub async fn get_stripe_config() -> Result<StripeConfig, String> {
    #[cfg(feature = "ssr")]
    {
        use reqwest::Client;
        let client = Client::new();
        client
            .get("http://127.0.0.1:8080/api/stripe-config")
            .send()
            .await
            .map_err(|e| format!("Failed to fetch Stripe config: {}", e))?
            .json()
            .await
            .map_err(|e| format!("Failed to parse Stripe config: {}", e))
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err("Stripe config requires server-side rendering".to_string())
    }
}
