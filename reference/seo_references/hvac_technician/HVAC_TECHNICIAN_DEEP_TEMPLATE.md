# HVAC Technician Deep Template & SEO Strategy

## 1. Deep SEO Strategy

### Core Keywords
- **High Intent**: "AC repair near me", "Furnace breakdown", "HVAC maintenance [City]", "Emergency air conditioning repair".
- **Service-Based**: "Heat pump installation", "Ductless mini split", "Commercial HVAC", "Ventilation systems".
- **Long-Tail**: "Energy efficient heating options", "Why is my AC making noise", "Air quality testing".

### Recommended Content Hierarchy
1.  **Home Page**: "Year-Round Comfort", "Commercial & Residential".
2.  **Service Pages**: /air-conditioning, /heating, /maintenance-plans.
3.  **Brands Page**: "We service Daikin, Mitsubishi, etc."

### Schema Markup Strategy
Use `HVACBusiness`.

```json
{
  "@context": "https://schema.org",
  "@type": "HVACBusiness",
  "name": "Climate Control Pros",
  "description": "Expert HVAC installation, repair, and maintenance.",
   "address": {
    "@type": "PostalAddress",
    "addressLocality": "Cardiff"
  }
}
```

---

## 2. Leptos Frontend Scaffold
**File**: `frontend-leptos/src/pages/trades/hvac.rs`

```rust
use leptos::*;
use crate::components::seo::SeoHead;

#[component]
pub fn HvacLanding() -> impl IntoView {
    view! {
        <SeoHead
            title="HVAC Technicians | AC Repair & Heating Installation"
            description="Keep your home comfortable all year round. Expert AC repair, heat pump installation, and maintenance."
            canonical_url="https://handyman.com/trades/hvac"
        />

        <section class="hero bg-blue-100 text-deep-black py-20">
            <div class="container mx-auto px-4">
                <h1 class="text-5xl font-bold mb-6 text-blue-900">"Perfect Climate. Every Room."</h1>
                <p class="text-xl mb-8">"Heating, Cooling, and Ventilation experts."</p>
                <a href="/maintenance" class="btn-primary bg-blue-600 text-white">"Check Maintenance Plans"</a>
            </div>
        </section>

        // Service Map
        <section class="py-12 bg-white">
             <div class="container mx-auto px-4">
                <ServiceMap lat=51.4816 lng=-3.1791 radius=10000.0 />
            </div>
        </section>

        <section class="services py-16">
            <div class="container mx-auto px-4 grid grid-cols-1 md:grid-cols-2 gap-8">
                 <div class="card p-6 shadow-lg bg-white">
                    <h3 class="text-2xl font-bold text-red-600">"Heating"</h3>
                    <p>"Furnaces, Heat Pumps, and Boilers."</p>
                 </div>
                 <div class="card p-6 shadow-lg bg-white">
                    <h3 class="text-2xl font-bold text-blue-600">"Cooling"</h3>
                    <p>"Air Conditioning repairs and installation."</p>
                 </div>
            </div>
        </section>
    }
}
```

---

## 3. Axum Backend Scaffold
**File**: `backend/src/api/trades/hvac.rs`

```rust
use axum::{
    routing::{get},
    Router, Json,
};

pub fn routes() -> Router {
    Router::new()
        .route("/systems", get(supported_systems))
}

async fn supported_systems() -> Json<Vec<&'static str>> {
    Json(vec!["Daikin", "Mitsubishi Electric", "Toshiba", "LG"])
}
```
