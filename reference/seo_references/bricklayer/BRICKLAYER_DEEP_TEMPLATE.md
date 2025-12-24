# Bricklayer Deep Template & SEO Strategy

## 1. Deep SEO Strategy

### Core Keywords
- **High Intent**: "Bricklayer needed", "Repointing specialist", "Garden wall builder", "Extension brickwork".
- **Service-Based**: "Brick stitching", "Chimney repair", "Stone masonry", "Patio construction".
- **Long-Tail**: "Cost to build a brick front wall", "Lime mortar repointing specialists", "Match existing brickwork".

### Recommended Content Hierarchy
1.  **Home Page**: "Quality Masonry", "Extensions & Walls".
2.  **Service Pages**: /repointing, /garden-walls, /extensions.
3.  **About Us**: Experience with different brick types, heritage work.

### Schema Markup Strategy
Use `HomeAndConstructionBusiness` (or `GeneralContractor`).

```json
{
  "@context": "https://schema.org",
  "@type": "HomeAndConstructionBusiness",
  "name": "Solid Build Bricklaying",
  "description": "Expert bricklaying, repointing, and masonry services.",
  "address": {
    "@type": "PostalAddress",
    "addressLocality": "Leeds"
  }
}
```

---

## 2. Leptos Frontend Scaffold
**File**: `frontend-leptos/src/pages/trades/bricklayer.rs`

```rust
use leptos::*;
use crate::components::seo::SeoHead;

#[component]
pub fn BricklayerLanding() -> impl IntoView {
    view! {
        <SeoHead
            title="Expert Bricklayers | Repointing, Walls & Extensions"
            description="Professional bricklaying services. Garden walls, house extensions, and restoration work. Quality guaranteed."
            canonical_url="https://handyman.com/trades/bricklayer"
        />

        <section class="hero bg-red-900 text-white py-20">
            <div class="container mx-auto px-4">
                <h1 class="text-5xl font-bold mb-6">"Solid Foundations, Expert Finish"</h1>
                <p class="text-xl mb-8">"From garden walls to grand extensions, we build to last."</p>
            </div>
        </section>

        // Map Integration
        <section class="py-12">
            <div class="container mx-auto px-4">
                <ServiceMap lat=53.8008 lng=-1.5491 radius=6000.0 />
            </div>
        </section>

        <section class="services py-16">
            <div class="container mx-auto px-4">
               <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                    <div class="card p-6 border shadow-md">
                        <h3 class="text-2xl font-bold text-red-800">"Repointing"</h3>
                        <p>"Restore your masonry with our dust-free repointing service."</p>
                    </div>
                     <div class="card p-6 border shadow-md">
                        <h3 class="text-2xl font-bold text-red-800">"New Builds"</h3>
                        <p>"Precision brickwork for extensions and new homes."</p>
                    </div>
               </div>
            </div>
        </section>
    }
}
```

---

## 3. Axum Backend Scaffold
**File**: `backend/src/api/trades/bricklayer.rs`

```rust
use axum::{
    routing::{get},
    Router, Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct BrickService {
    name: String,
    unit_price: String, // e.g. "per sq meter"
}

pub fn routes() -> Router {
    Router::new()
        .route("/pricing-guide", get(get_pricing))
}

async fn get_pricing() -> Json<Vec<BrickService>> {
    Json(vec![
        BrickService { name: "Standard Brickwork".into(), unit_price: "Contact for Quote".into() },
        BrickService { name: "Repointing".into(), unit_price: "$50 per m2".into() },
    ])
}
```
