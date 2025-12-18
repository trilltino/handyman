# Electrician Deep Template & SEO Strategy

## 1. Deep SEO Strategy

### Core Keywords
- **High Intent**: "Emergency electrician near me", "24/7 electrician [City]", "Licensed electrician [City]", "Electrical repair same day".
- **Residential**: "Home rewiring", "Fuse box replacement", "LED lighting installation", "EV charger installation home", "Electrical panel upgrade".
- **Commercial**: "Commercial electrical contractor", "Office lighting retrofit", "Industrial wiring".
- **Problem-Based**: "Circuit breaker keeps tripping", "Outlet not working", "Flickering lights repair".
- **Long-Tail**: "Cost to rewire a 3 bedroom house", "Safety switch installation price", "Smart home automation electrician".

### Recommended Content Hierarchy
1.  **Home Page**: Overview of services, trust signals (licenses, insurance), "Emergency Callout" prominent button.
2.  **Service Pages**: Dedicated pages for high-value services (e.g., /ev-charger, /rewiring, /emergency).
3.  **Local Landing Pages**: /electrician-[suburb-name] for every target suburb.
4.  **FAQ Section**: Answers to "How much does X cost?", "Is my wiring unsafe?".

### Schema Markup Strategy
Use `Electrician` (subtype of `HomeAndConstructionBusiness`).

```json
{
  "@context": "https://schema.org",
  "@type": "Electrician",
  "name": "SparkSafe Electrical",
  "image": "https://example.com/logo.png",
  "@id": "https://example.com",
  "url": "https://example.com",
  "telephone": "+44 123 456 7890",
  "priceRange": "$$",
  "address": {
    "@type": "PostalAddress",
    "streetAddress": "123 Voltage Lane",
    "addressLocality": "London",
    "postalCode": "SW1A 1AA",
    "addressCountry": "UK"
  },
  "geo": {
    "@type": "GeoCoordinates",
    "latitude": 51.5074,
    "longitude": -0.1278
  },
  "openingHoursSpecification": [
    {
      "@type": "OpeningHoursSpecification",
      "dayOfWeek": [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday"
      ],
      "opens": "08:00",
      "closes": "18:00"
    },
    {
      "@type": "OpeningHoursSpecification",
      "dayOfWeek": ["Saturday", "Sunday"],
      "opens": "00:00",
      "closes": "23:59",
      "description": "Emergency Callout Available"
    }
  ],
  "sameAs": [
    "https://facebook.com/sparksafe",
    "https://instagram.com/sparksafe"
  ]
}
```

---

## 2. Leptos Frontend Scaffold
**File**: `frontend-leptos/src/pages/trades/electrician.rs`

```rust
use leptos::*;
use crate::components::seo::SeoHead; // Assuming a shared SEO component

#[component]
pub fn ElectricianLanding() -> impl IntoView {
    view! {
        <SeoHead
            title="Expert Electrician Services | 24/7 Emergency Repairs"
            description="Licensed electricians in your area. Fast reliable service for rewiring, fuse boxes, and EV chargers. Book online now."
            canonical_url="https://handyman.com/trades/electrician"
        />

        <section class="hero bg-deep-black text-white py-20">
            <div class="container mx-auto px-4">
                <h1 class="text-5xl font-bold text-deep-red mb-6">"Master Electricians You Can Trust"</h1>
                <p class="text-xl mb-8">"From emergency fixes to full home rewiring. We keep the lights on."</p>
                <div class="flex gap-4">
                    <a href="/book" class="btn-primary">"Book Now"</a>
                    <a href="tel:+123456789" class="btn-secondary">"Call 24/7"</a>
                </div>
            </div>
        </section>

        // New Service Map Integration
        <section class="map-section py-16 bg-gray-50">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold mb-8 text-center text-deep-black">"Our Local Service Area"</h2>
                <ServiceMap lat=51.5074 lng=-0.1278 radius=5000.0 />
            </div>
        </section>

        <section class="services py-16 bg-white text-deep-black">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold mb-12 text-center">"Our Electrical Services"</h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                    // Service Card 1
                    <div class="card p-6 border border-gray-200 rounded-lg shadow-lg hover:shadow-deep-red transition">
                        <h3 class="text-2xl font-bold mb-4">"Emergency Repairs"</h3>
                        <p>"Power outage? Tripped Switch? We arrive within 60 minutes."</p>
                    </div>
                     // Service Card 2
                    <div class="card p-6 border border-gray-200 rounded-lg shadow-lg hover:shadow-deep-red transition">
                        <h3 class="text-2xl font-bold mb-4">"Rewiring"</h3>
                        <p>"Full and partial house rewiring to modern safety standards."</p>
                    </div>
                    // Service Card 3
                    <div class="card p-6 border border-gray-200 rounded-lg shadow-lg hover:shadow-deep-red transition">
                        <h3 class="text-2xl font-bold mb-4">"EV Chargers"</h3>
                        <p>"Installation of home charging stations for your electric vehicle."</p>
                    </div>
                </div>
            </div>
        </section>
    }
}
```

---

## 3. Axum Backend Scaffold
**File**: `backend/src/api/trades/electrician.rs`

```rust
use axum::{
    routing::{get, post},
    Router, Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ElectricalService {
    pub id: String,
    pub name: String,
    pub price_range: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/services", get(get_services))
        .route("/emergency-status", get(get_emergency_availability))
}

async fn get_services() -> Json<Vec<ElectricalService>> {
    let services = vec![
        ElectricalService {
            id: "rewire".into(),
            name: "Home Rewiring".into(),
            price_range: "$2000 - $8000".into(),
        },
        ElectricalService {
            id: "emergency".into(),
            name: "Emergency Callout".into(),
            price_range: "$150 - $400".into(),
        },
    ];
    Json(services)
}

async fn get_emergency_availability() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "available": true,
        "estimated_arrival": "45 mins"
    }))
}
```
