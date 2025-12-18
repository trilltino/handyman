# Plasterer Deep Template & SEO Strategy

## 1. Deep SEO Strategy

### Core Keywords
- **High Intent**: "Plasterer near me", "Skimming walls price", "Ceiling repair", "Plastering contractor".
- **Service-Based**: "Dry lining", "Rendering", "Venetian plastering", "External wall insulation".
- **Long-Tail**: "Plaster over Artex", "Smooth finish for painting", "Patch repair holes in wall".

### Recommended Content Hierarchy
1.  **Home Page**: "Smooth Finishes", "Clean & Tidy Work".
2.  **Service Pages**: /skimming, /rendering, /dry-lining.
3.  **Gallery**: Before and after photos of smooth walls.

### Schema Markup Strategy
Use `HomeAndConstructionBusiness`.

```json
{
  "@context": "https://schema.org",
  "@type": "HomeAndConstructionBusiness",
  "name": "Smooth Finish Plastering",
  "description": "Interior plastering, skimming, and exterior rendering.",
   "address": {
    "@type": "PostalAddress",
    "addressLocality": "Bristol"
  }
}
```

---

## 2. Leptos Frontend Scaffold
**File**: `frontend-leptos/src/pages/trades/plasterer.rs`

```rust
use leptos::*;
use crate::components::seo::SeoHead;

#[component]
pub fn PlastererLanding() -> impl IntoView {
    view! {
        <SeoHead
            title="Local Plasterer | Skimming, Rendering & Dry Lining"
            description="Get perfectly smooth walls. We specialize in skimming over Artex, dry lining, and repairs. Clean and professional."
            canonical_url="https://handyman.com/trades/plasterer"
        />

        <section class="hero bg-gray-100 text-deep-black py-20">
            <div class="container mx-auto px-4">
                <h1 class="text-5xl font-bold mb-6">"Perfection is in the Finish"</h1>
                <p class="text-xl mb-8">"Transform tired rooms with perfectly skimmed walls."</p>
            </div>
        </section>

        // Local Map
        <section class="py-12 bg-white">
             <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold mb-6 text-center">"Covering Bristol & Bath"</h2>
                <ServiceMap lat=51.4545 lng=-2.5879 radius=12000.0 />
            </div>
        </section>

        <section class="features py-16 bg-white">
            <div class="container mx-auto px-4">
               <ul class="list-disc pl-6 space-y-4 text-lg">
                    <li>"Skimming over Artex removal"</li>
                    <li>"Dot and Dab"</li>
                    <li>"External Rendering (K-Rend, Monocouche)"</li>
                    <li>"Clean up guaranteed"</li>
               </ul>
            </div>
        </section>
    }
}
```

---

## 3. Axum Backend Scaffold
**File**: `backend/src/api/trades/plasterer.rs`

```rust
use axum::{
    routing::{get},
    Router, Json,
};

pub fn routes() -> Router {
    Router::new()
        .route("/services", get(services))
}

async fn services() -> Json<Vec<String>> {
    Json(vec![
        "Skimming".into(),
        "Rendering".into(),
        "Plasterboarding".into(),
        "Coving".into(),
    ])
}
```
