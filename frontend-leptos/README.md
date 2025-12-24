# Frontend (Leptos)

Leptos-based SSR + hydration frontend.

## Structure

```
frontend-leptos/
├── src/
│   ├── lib.rs           # App entry, routing
│   ├── main.rs          # SSR server + proxy
│   ├── api/             # Backend API client
│   ├── components/      # Reusable UI components
│   └── pages/           # Route page components
│       ├── home.rs
│       ├── pricing.rs
│       └── examples/    # Demo sites
│           └── handyman_app/
├── public/              # Static assets
│   ├── robots.txt
│   └── sitemap.xml
└── style/               # CSS source
    └── input.css
```

## Running

```bash
# Development (hot reload)
npm run watch:css   # Terminal 1
cargo leptos watch  # Terminal 2

# Production build
cargo leptos build --release
```

## Key Patterns

- **SSR + Hydration**: Server renders HTML, client hydrates
- **Nested Routes**: Layout wrapping with `<ParentRoute>`
- **leptos_meta**: SEO tags from components
- **Proxy Handler**: `/api/*` proxied to backend

## Component Pattern

```rust
#[component]
pub fn MyComponent(
    #[prop(into)] title: String,
    #[prop(optional)] subtitle: Option<String>,
) -> impl IntoView {
    view! {
        <h1>{title}</h1>
        {subtitle.map(|s| view! { <p>{s}</p> })}
    }
}
```

## Styling

Uses Tailwind CSS 4:
- Source: `style/input.css`
- Output: `public/xftradesmen.css`

See [IDM/02_frontend_architecture.md](../IDM/02_frontend_architecture.md) for details.
