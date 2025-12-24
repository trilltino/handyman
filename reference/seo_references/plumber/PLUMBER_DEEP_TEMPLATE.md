# Plumber Deep Template & SEO Strategy

## 1. Deep SEO Strategy

### Core Keywords
- **High Intent**: "Emergency plumber near me", "Burst pipe repair [City]", "Blocked drain cleaner", "24 hour plumbing service".
- **Service-Based**: "Boiler repair", "Water heater installation", "Leak detection", "Bathroom fitting", "Toilet repair".
- **Problem-Based**: "No hot water", "Low water pressure", "Sink not draining", "Radiator not working".
- **Long-Tail**: "Cost to unclog main sewer line", "Emergency boiler repair price", "Plumber for kitchen renovation".

### Recommended Content Hierarchy
1.  **Home Page**: "24/7 Response", "No Callout Fee" (if applicable), "Gas Safe Registered".
2.  **Service Pages**: /boiler-repair, /blocked-drains, /bathroom-installation.
3.  **Local Landing Pages**: /plumber-[suburb].
4.  **DIY Guides (Blog)**: "How to bleed a radiator", "What to do when a pipe bursts".

### Schema Markup Strategy
Use `Plumber` (subtype of `HomeAndConstructionBusiness`).

```json
{
  "@context": "https://schema.org",
  "@type": "Plumber",
  "name": "FlowMaster Plumbing",
  "image": "https://example.com/plumber-logo.png",
  "@id": "https://example.com",
  "url": "https://example.com",
  "telephone": "+44 123 456 0000",
  "priceRange": "$$",
  "address": {
    "@type": "PostalAddress",
    "streetAddress": "45 Pipe Street",
    "addressLocality": "Manchester",
    "postalCode": "M1 1AA",
    "addressCountry": "UK"
  },
  "geo": {
    "@type": "GeoCoordinates",
    "latitude": 53.4808,
    "longitude": -2.2426
  },
  "openingHoursSpecification": [
    {
      "@type": "OpeningHoursSpecification",
      "dayOfWeek": ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"],
      "opens": "00:00",
      "closes": "23:59",
      "description": "24/7 Emergency Service"
    }
  ],
  "sameAs": [
    "https://facebook.com/flowmaster"
  ]
}
```

---

## 2. Leptos Frontend Scaffold
**File**: `frontend-leptos/src/pages/trades/plumber.rs`

```rust
use leptos::*;
use crate::components::seo::SeoHead;

#[component]
pub fn PlumberLanding() -> impl IntoView {
    view! {
        <SeoHead
            title="Emergency Plumber | Boiler Repair & Drain Cleaning"
            description="Fast response plumbers in [City]. We fix leaks, boilers, and blocked drains. Gas Safe registered. Call now."
            canonical_url="https://handyman.com/trades/plumber"
        />

        <section class="hero bg-blue-900 text-white py-20" style="background-color: #0a101f;"> // Custom deep color
            <div class="container mx-auto px-4">
                <h1 class="text-5xl font-bold text-white mb-6">"Plumbing Disasters Fixed Fast"</h1>
                <p class="text-xl mb-8">"Leaks, Drains, and Boilers. We handle the dirty work."</p>
                <div class="flex gap-4">
                     <a href="tel:+123456789" class="btn-primary bg-deep-red hover:bg-red-700 text-white px-8 py-3 rounded-lg font-bold">"Call Emergency"</a>
                    <a href="/book" class="btn-secondary border border-white text-white px-8 py-3 rounded-lg hover:bg-white hover:text-black">"Book Routine Job"</a>
                </div>
            </div>
        </section>

        // Service Area Map
        <section class="map-section py-16">
            <div class="container mx-auto px-4">
                 <h2 class="text-3xl font-bold mb-8 text-center">"Serving Greater Manchester"</h2>
                 <ServiceMap lat=53.4808 lng=-2.2426 radius=8000.0 />
            </div>
        </section>

        <section class="services py-16 bg-white text-deep-black">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold mb-12 text-center">"Our Plumbing Services"</h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                     <div class="card p-6 shadow-lg">
                        <h3 class="text-2xl font-bold mb-4">"Blocked Drains"</h3>
                        <p>"High pressure jetting to clear blockages instantly."</p>
                    </div>
                     <div class="card p-6 shadow-lg">
                        <h3 class="text-2xl font-bold mb-4">"Boiler Repair"</h3>
                        <p>"Gas Safe engineers to fix heating issues."</p>
                    </div>
                     <div class="card p-6 shadow-lg">
                        <h3 class="text-2xl font-bold mb-4">"Leak Detection"</h3>
                        <p>"Find and fix hidden leaks before they cause damage."</p>
                    </div>
                </div>
            </div>
        </section>
    }
}
```

---

## 3. Axum Backend Scaffold
**File**: `backend/src/api/trades/plumber.rs`

```rust
use axum::{
    routing::{get, post},
    Router, Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PlumbingJob {
    pub id: String,
    pub title: String,
    pub is_emergency: bool,
}

pub fn routes() -> Router {
    Router::new()
        .route("/quote", post(request_quote))
}

#[derive(Deserialize)]
struct QuoteRequest {
    problem_type: String, // e.g., leak, blockage, boiler
    description: String,
}

async fn request_quote(Json(payload): Json<QuoteRequest>) -> Json<serde_json::Value> {
    // Logic to calculate estimated quote range
    let estimate = match payload.problem_type.as_str() {
        "blockage" => "$100 - $250",
        "boiler" => "$150 - $500",
        _ => "Contact for quote",
    };

    Json(serde_json::json!({
        "estimated_cost": estimate,
        "next_step": "We will call you within 5 minutes"
    }))
}
```
