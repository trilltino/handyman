# Locksmith Deep Template & SEO Strategy

## 1. Deep SEO Strategy

### Core Keywords
- **High Intent**: "Locksmith near me", "Emergency locksmith [City]", "Locked out of house", "Change locks price".
- **Service-Based**: "uPVC door lock repair", "Snap safe locks", "Car key cutting", "Safe opening".
- **Long-Tail**: "Lost house keys what to do", "Emergency boarding up service", "High security anti-snap locks".

### Recommended Content Hierarchy
1.  **Home Page**: "Arrive in 30 Mins", "No Damage Entry".
2.  **Services**: /emergency-entry, /lock-replacement, /auto-locksmith.
3.  **Pricing Page**: Transparent pricing builds trust.

### Schema Markup Strategy
Use `Locksmith`.

```json
{
  "@context": "https://schema.org",
  "@type": "Locksmith",
  "name": "SecureFast Locksmiths",
  "description": "24/7 emergency locksmith. Non-destructive entry and lock upgrades.",
  "openingHours": "Mo-Su 00:00-23:59",
  "address": {
    "@type": "PostalAddress",
    "addressLocality": "Edinburgh"
  }
}
```

---

## 2. Leptos Frontend Scaffold
**File**: `frontend-leptos/src/pages/trades/locksmith.rs`

```rust
use leptos::*;
use crate::components::seo::SeoHead;

#[component]
pub fn LocksmithLanding() -> impl IntoView {
    view! {
        <SeoHead
            title="24/7 Emergency Locksmith | Locked Out? Call Now"
            description="Fast response locksmiths in [City]. Non-destructive entry, lock changes, and security upgrades. Arrive in 30 mins."
            canonical_url="https://handyman.com/trades/locksmith"
        />

        <section class="hero bg-deep-black text-white py-20">
            <div class="container mx-auto px-4">
                <h1 class="text-5xl font-bold text-red-600 mb-6">"Locked Out?"</h1>
                <p class="text-3xl mb-8">"We'll get you back in. Fast."</p>
                <a href="tel:000000000" class="btn-primary bg-red-600 hover:bg-red-700 text-2xl px-10 py-4 rounded-full animate-pulse">"Call Now: 0123 456 789"</a>
            </div>
        </section>

        <section class="features py-16 text-center">
            <div class="container mx-auto px-4 grid grid-cols-1 md:grid-cols-3 gap-8">
                 <div>
                    <i class="icon-clock text-4xl mb-2 block"></i>
                    <h3 class="font-bold">"30 Min Response"</h3>
                 </div>
                 <div>
                    <i class="icon-shield text-4xl mb-2 block"></i>
                    <h3 class="font-bold">"No Damage Entry"</h3>
                 </div>
                 <div>
                    <i class="icon-check text-4xl mb-2 block"></i>
                    <h3 class="font-bold">"DBS Checked"</h3>
                 </div>
            </div>
        </section>

        // 30 Min Response Zone
        <section class="py-12 bg-gray-50">
             <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold mb-6 text-center">"30 Minute Response Zone"</h2>
                <ServiceMap lat=55.9533 lng=-3.1883 radius=5000.0 />
            </div>
        </section>
    }
}
```

---

## 3. Axum Backend Scaffold
**File**: `backend/src/api/trades/locksmith.rs`

```rust
use axum::{
    routing::{get},
    Router, Json,
};

pub fn routes() -> Router {
    Router::new()
        .route("/emergency-rates", get(rates))
}

async fn rates() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "daytime": 65,
        "evening": 85,
        "weekend": 95,
        "currency": "GBP"
    }))
}
```
