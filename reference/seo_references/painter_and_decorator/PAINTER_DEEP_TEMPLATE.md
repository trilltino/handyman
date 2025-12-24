# Painter & Decorator Deep Template & SEO Strategy

## 1. Deep SEO Strategy

### Core Keywords
- **High Intent**: "Painter and decorator near me", "Professional painters [City]", "Exterior painting services", "Wallpaper hanging specialist".
- **Service-Based**: "Interior painting", "Exterior house painting", "Woodwork restoration", "Commercial decorators".
- **Long-Tail**: "Eco-friendly paint options", "Cost to paint hallway and landing", "Painting sash windows".

### Recommended Content Hierarchy
1.  **Home Page**: "Transform Your Space", "Meticulous Preparation".
2.  **Portfolio**: High-quality before/after sliders.
3.  **Service Pages**: /interior, /exterior, /commercial.

### Schema Markup Strategy
Use `HousePainter`.

```json
{
  "@context": "https://schema.org",
  "@type": "HousePainter",
  "name": "Prism Decorators",
  "description": "High-quality interior and exterior painting services.",
   "address": {
    "@type": "PostalAddress",
    "addressLocality": "Brighton"
  }
}
```

---

## 2. Leptos Frontend Scaffold
**File**: `frontend-leptos/src/pages/trades/painter.rs`

```rust
use leptos::*;
use crate::components::seo::SeoHead;

#[component]
pub fn PainterLanding() -> impl IntoView {
    view! {
        <SeoHead
            title="Professional Painter & Decorator | Interior & Exterior"
            description="Transform your home with our expert painting services. Flawless finishes, wallpapering, and wood restoration."
            canonical_url="https://handyman.com/trades/painter"
        />

        <section class="hero bg-white text-deep-black py-20 relative overflow-hidden">
             // Artistic background touch
            <div class="absolute inset-0 bg-gradient-to-r from-gray-100 to-white z-0"></div>
            <div class="container mx-auto px-4 relative z-10">
                <h1 class="text-5xl font-bold mb-6 text-deep-red">"Bringing Colour to Life"</h1>
                <p class="text-xl mb-8 text-gray-700">"Meticulous preparation. Flawless finish."</p>
                <a href="/portfolio" class="btn-primary">"See Our Work"</a>
            </div>
        </section>

        // Service Area Map
        <section class="py-12 bg-gray-50">
             <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold mb-6 text-center">"Decorating Across Brighton"</h2>
                <ServiceMap lat=50.8225 lng=-0.1372 radius=2000.0 />
            </div>
        </section>

        <section class="gallery py-16">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold mb-8 text-center">"Our Work"</h2>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                    // Placeholders for gallery
                    <div class="bg-gray-200 h-64 rounded-lg"></div>
                    <div class="bg-gray-200 h-64 rounded-lg"></div>
                    <div class="bg-gray-200 h-64 rounded-lg"></div>
                    <div class="bg-gray-200 h-64 rounded-lg"></div>
                </div>
            </div>
        </section>
    }
}
```

---

## 3. Axum Backend Scaffold
**File**: `backend/src/api/trades/painter.rs`

```rust
use axum::{
    routing::{get},
    Router, Json,
};

pub fn routes() -> Router {
    Router::new()
        .route("/colors", get(trending_colors))
}

async fn trending_colors() -> Json<Vec<&'static str>> {
    Json(vec!["Sage Green", "Navy Blue", "Crisp White", "Charcoal"])
}
```
