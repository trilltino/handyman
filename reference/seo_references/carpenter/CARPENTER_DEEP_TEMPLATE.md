# Carpenter Deep Template & SEO Strategy

## 1. Deep SEO Strategy

### Core Keywords
- **High Intent**: "Carpenter near me", "Custom joinery services", "Fitted wardrobes", "Door installation".
- **Service-Based**: "Decking installer", "Kitchen cabinet installation", "Wood flooring fitting", "Staircase renovation".
- **Long-Tail**: "Bespoke bookshelves for alcoves", "Timber frame extension cost", "Replace rot in door frame".

### Recommended Content Hierarchy
1.  **Home Page**: Portfolio showcase (Visuals are key), "Bespoke Joinery" vs "First Fix/Second Fix".
2.  **Portfolio Page**: High-res images of completed projects.
3.  **Service Pages**: /fitted-furniture, /kitchens, /flooring.

### Schema Markup Strategy
Use `GeneralContractor` or `HomeAndConstructionBusiness`.

```json
{
  "@context": "https://schema.org",
  "@type": "HomeAndConstructionBusiness",
  "name": "Elite Joinery & Carpentry",
  "image": "https://example.com/carpenter-logo.png",
  "description": "Bespoke joinery, fitted wardrobes, and structural carpentry.",
  "@id": "https://example.com",
  "url": "https://example.com",
  "telephone": "+44 123 456 0000",
  "address": {
    "@type": "PostalAddress",
    "streetAddress": "12 Oak Lane",
    "addressLocality": "Birmingham",
    "postalCode": "B1 1AA",
    "addressCountry": "UK"
  },
  "openingHoursSpecification": [
    {
      "@type": "OpeningHoursSpecification",
      "dayOfWeek": ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"],
      "opens": "08:00",
      "closes": "17:00"
    }
  ]
}
```

---

## 2. Leptos Frontend Scaffold
**File**: `frontend-leptos/src/pages/trades/carpenter.rs`

```rust
use leptos::*;
use crate::components::seo::SeoHead;

#[component]
pub fn CarpenterLanding() -> impl IntoView {
    view! {
        <SeoHead
            title="Master Carpenter & Joiner | Bespoke Furniture & Flooring"
            description="Custom fitted wardrobes, kitchen installation, and structural carpentry. View our portfolio of handcrafted wood projects."
            canonical_url="https://handyman.com/trades/carpenter"
        />

        <section class="hero bg-stone-900 text-white py-20">
            <div class="container mx-auto px-4">
                <h1 class="text-5xl font-bold text-amber-500 mb-6">"Craftsmanship in Every Grain"</h1>
                <p class="text-xl mb-8">"Bespoke joinery and expert carpentry for your home."</p>
                <a href="/portfolio" class="btn-primary bg-amber-600 hover:bg-amber-700">"View Portfolio"</a>
            </div>
        </section>

        // Map Section
        <section class="py-12 bg-stone-50">
             <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold mb-6 text-center">"Areas We Cover"</h2>
                <ServiceMap lat=52.4862 lng=-1.8904 radius=10000.0 />
            </div>
        </section>

        <section class="portfolio-preview py-16">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold mb-8">"Latest Projects"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                     <div class="project-card">
                        <img src="/images/wardrobe.jpg" alt="Fitted Wardrobes" class="rounded-lg shadow-lg mb-4"/>
                        <h3 class="text-xl font-bold">"Bespoke Wardrobes"</h3>
                    </div>
                     <div class="project-card">
                        <img src="/images/decking.jpg" alt="Garden Decking" class="rounded-lg shadow-lg mb-4"/>
                        <h3 class="text-xl font-bold">"Garden Decking"</h3>
                    </div>
                </div>
            </div>
        </section>
    }
}
```

---

## 3. Axum Backend Scaffold
**File**: `backend/src/api/trades/carpenter.rs`

```rust
use axum::{
    routing::{get},
    Router, Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub image_url: String,
    pub description: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/portfolio", get(get_portfolio))
}

async fn get_portfolio() -> Json<Vec<Project>> {
    let projects = vec![
        Project {
            id: "p1".into(),
            title: "Oak Staircase".into(),
            image_url: "/img/staircase.jpg".into(),
            description: "Restoration of Victorian staircase".into(),
        },
        Project {
            id: "p2".into(),
            title: "Custom Library".into(),
            image_url: "/img/library.jpg".into(),
            description: "Floor to ceiling bookshelves".into(),
        },
    ];
    Json(projects)
}
```
