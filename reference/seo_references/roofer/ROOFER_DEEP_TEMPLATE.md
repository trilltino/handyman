# Roofer Deep Template & SEO Strategy

## 1. Deep SEO Strategy

### Core Keywords
- **High Intent**: "Roofing contractors near me", "Emergency roof repair [City]", "Flat roof specialists", "Leaking roof fix", "Chimney repair".
- **Service-Based**: "New roof installation", "Slate roofing", "EPDM protection", "Gutter cleaning and repair", "Fascias and soffits".
- **Long-Tail**: "Cost of new roof 3 bed semi", "Roof leak detection service", "Storm damage roof repair insurance".

### Recommended Content Hierarchy
1.  **Home Page**: "We keep you dry", "25 Year Guarantee".
2.  **Services**: /flat-roofs, /tiling, /repairs.
3.  **Trust Page**: Portfolio + Reviews (crucial for roofing).

### Schema Markup Strategy
Use `RoofingContractor`.

```json
{
  "@context": "https://schema.org",
  "@type": "RoofingContractor",
  "name": "Apex Roofing",
  "description": "Professional roofing services including repairs, new roofs, and flat roofing.",
  "address": {
    "@type": "PostalAddress",
    "addressLocality": "Newcastle"
  }
}
```

---

## 2. Leptos Frontend Scaffold
**File**: `frontend-leptos/src/pages/trades/roofer.rs`

```rust
use leptos::*;
use crate::components::seo::SeoHead;

#[component]
pub fn RooferLanding() -> impl IntoView {
    view! {
        <SeoHead
            title="Local Roofing Contractors | Repairs & New Roofs"
            description="Expert roofing services. From slipped tiles to full re-roofing projects. Guaranteed work. Free quotes."
            canonical_url="https://handyman.com/trades/roofer"
        />

        <section class="hero bg-slate-800 text-white py-20">
            <div class="container mx-auto px-4">
                <h1 class="text-5xl font-bold mb-6">"Protect Your Home from the Elements"</h1>
                <p class="text-xl mb-8">"Reliable roofing repairs and installations."</p>
                <a href="/quote" class="btn-primary">"Get Free Quote"</a>
            </div>
        </section>

        // Map Section
        <section class="py-12">
            <div class="container mx-auto px-4">
               <ServiceMap lat=54.9783 lng=-1.6178 radius=15000.0 />
            </div>
        </section>

        <section class="services py-16">
            <div class="container mx-auto px-4 grid grid-cols-1 md:grid-cols-3 gap-8">
                <div class="card p-6 border">
                    <h3 class="text-2xl font-bold mb-4">"Emergency Repairs"</h3>
                    <p>"Storm damage, leaks, and slipped tiles fixed fast."</p>
                </div>
                <div class="card p-6 border">
                    <h3 class="text-2xl font-bold mb-4">"Flat Roofing"</h3>
                    <p>"EPDM, GRP fibreglass, and felt options."</p>
                </div>
                 <div class="card p-6 border">
                    <h3 class="text-2xl font-bold mb-4">"Chimneys"</h3>
                    <p>"Repointing, capping, and lead work."</p>
                </div>
            </div>
        </section>
    }
}
```

---

## 3. Axum Backend Scaffold
**File**: `backend/src/api/trades/roofer.rs`

```rust
use axum::{
    routing::{post},
    Router, Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct InspectionRequest {
    address: String,
    roof_type: String, // tiled, flat, etc.
}

pub fn routes() -> Router {
    Router::new()
        .route("/request-inspection", post(request_inspection))
}

async fn request_inspection(Json(_payload): Json<InspectionRequest>) -> Json<&'static str> {
    Json("Inspection booked. We will contact you.")
}
```
