//! Frontend application entry point.
//!
//! Handles SSR server setup (when `ssr` feature is enabled).

// Increase recursion limit for complex Leptos view hierarchies in SSR
#![recursion_limit = "1024"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        http::header::HeaderValue,
        routing::{any, get},
        Router,
    };
    use frontend_leptos::App;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_http::set_header::SetResponseHeaderLayer;

    // Initialize logging
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .ok();

    // Get Leptos configuration
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // Generate route list from Leptos app
    let routes = generate_route_list(App);

    // Build Axum router with Leptos integration
    let app = Router::new()
        .route("/health", get(|| async { "OK" })) // Health check for Fly.io
        .route("/sitemap.xml", get(sitemap_handler))
        .route("/robots.txt", get(robots_handler))
        .route("/api/{*fn_name}", any(proxy_handler)) // Proxy API requests
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        // Security headers - HSTS
        .layer(SetResponseHeaderLayer::if_not_present(
            axum::http::header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=31536000; includeSubDomains"),
        ))
        // Canonical Redirect (www -> non-www)
        .layer(axum::middleware::from_fn(canonical_redirect_middleware));

    // Start the server
    log::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

/// Handler for /robots.txt - served directly from frontend
#[cfg(feature = "ssr")]
async fn robots_handler() -> axum::response::Response {
    use axum::http::header;

    let robots = r#"User-agent: *
Allow: /
Sitemap: https://xftradesman.com/sitemap.xml
"#;

    axum::response::Response::builder()
        .header(header::CONTENT_TYPE, "text/plain")
        .body(axum::body::Body::from(robots))
        .unwrap_or_else(|_| axum::response::Response::default())
}

/// Handler for /sitemap.xml - served directly from frontend
#[cfg(feature = "ssr")]
async fn sitemap_handler() -> axum::response::Response {
    use axum::http::header;

    let base_url = "https://xftradesman.com";

    let static_routes = [
        ("", "1.0"),
        ("/about", "0.8"),
        ("/contact", "0.8"),
        ("/pricing", "0.8"),
        ("/blog", "0.8"),
        ("/coventry", "0.9"),
        ("/packages", "0.8"),
        ("/handyman", "0.9"),
        ("/industries", "0.8"),
    ];

    let blog_posts = [
        "/blog/why-tradesmen-need-websites",
        "/blog/local-seo-guide",
        "/blog/building-trust-online",
    ];

    let mut url_entries = String::new();

    for (route, priority) in static_routes {
        url_entries.push_str(&format!(
            r#"  <url>
    <loc>{}{}</loc>
    <changefreq>weekly</changefreq>
    <priority>{}</priority>
  </url>
"#,
            base_url, route, priority
        ));
    }

    for post in blog_posts {
        url_entries.push_str(&format!(
            r#"  <url>
    <loc>{}{}</loc>
    <changefreq>monthly</changefreq>
    <priority>0.7</priority>
  </url>
"#,
            base_url, post
        ));
    }

    let sitemap = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{}</urlset>"#,
        url_entries
    );

    axum::response::Response::builder()
        .header(header::CONTENT_TYPE, "application/xml")
        .body(axum::body::Body::from(sitemap))
        .unwrap_or_else(|_| axum::response::Response::default())
}

#[cfg(feature = "ssr")]
async fn proxy_handler(req: axum::extract::Request) -> axum::response::Response {
    use axum::response::IntoResponse;

    // The backend API URL - usually running on localhost:3001 in production
    // or defined via environment variable
    let api_url = std::env::var("API_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());

    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let uri = format!("{}{}", api_url, path_query);

    let client = reqwest::Client::new();

    // Reconstruct the request to the backend
    let (parts, body) = req.into_parts();

    let req_builder = client.request(parts.method, &uri).headers(parts.headers);

    // Axum body to Reqwest body conversion is complex, simpler to stream bytes
    // For now, let's just forward as is if possible, or read bytes
    // Using bytes is safer for simple proxying
    let bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .unwrap_or_default();

    let resp = req_builder.body(bytes).send().await;
    match resp {
        Ok(resp) => {
            log::info!("Proxy success: {} -> {}", uri, resp.status());
            let mut response_builder = axum::response::Response::builder().status(resp.status());
            // Forward headers
            if let Some(headers) = response_builder.headers_mut() {
                for (key, value) in resp.headers() {
                    headers.insert(key, value.clone());
                }
            }

            let bytes = resp.bytes().await.unwrap_or_default();
            response_builder
                .body(axum::body::Body::from(bytes))
                .unwrap()
        }
        Err(e) => {
            log::error!("Proxy error for url ({}): {:#?}", uri, e);
            (
                axum::http::StatusCode::BAD_GATEWAY,
                format!(
                    "Proxy error: error sending request for url ({}): {}",
                    uri, e
                ),
            )
                .into_response()
        }
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
}

#[cfg(feature = "ssr")]
fn shell(options: leptos::prelude::LeptosOptions) -> impl leptos::prelude::IntoView {
    use frontend_leptos::App;
    use leptos::prelude::*;
    use leptos_meta::MetaTags;

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" type="image/png" href="/favicon.png"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() />
                <MetaTags/>
                // TikTok Pixel Code
                <script>
                    {r#"
                    !function (w, d, t) {
                        w.TiktokAnalyticsObject=t;var ttq=w[t]=w[t]||[];ttq.methods=["page","track","identify","instances","debug","on","off","once","ready","alias","group","enableCookie","disableCookie","holdConsent","revokeConsent","grantConsent"],ttq.setAndDefer=function(t,e){t[e]=function(){t.push([e].concat(Array.prototype.slice.call(arguments,0)))}};for(var i=0;i<ttq.methods.length;i++)ttq.setAndDefer(ttq,ttq.methods[i]);ttq.instance=function(t){for(
                        var e=ttq._i[t]||[],n=0;n<ttq.methods.length;n++)ttq.setAndDefer(e,ttq.methods[n]);return e},ttq.load=function(e,n){var r="https://analytics.tiktok.com/i18n/pixel/events.js",o=n&&n.partner;ttq._i=ttq._i||{},ttq._i[e]=[],ttq._i[e]._u=r,ttq._t=ttq._t||{},ttq._t[e]=+new Date,ttq._o=ttq._o||{},ttq._o[e]=n||{};n=document.createElement("script")
                        ;n.type="text/javascript",n.async=!0,n.src=r+"?sdkid="+e+"&lib="+t;e=document.getElementsByTagName("script")[0];e.parentNode.insertBefore(n,e)};
                        ttq.load('D56V23RC77U4D2G7TO00');
                        ttq.page();
                    }(window, document, 'ttq');
                    "#}
                </script>
                // Google Analytics 4
                <script async src="https://www.googletagmanager.com/gtag/js?id=G-66NMFXK2G3"></script>
                <script>
                    {r#"
                    window.dataLayer = window.dataLayer || [];
                    function gtag(){dataLayer.push(arguments);}
                    gtag('js', new Date());
                    gtag('config', 'G-66NMFXK2G3');
                    "#}
                </script>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

/// Middleware to strictly redirect www to non-www
#[cfg(feature = "ssr")]
async fn canonical_redirect_middleware(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    use axum::http::header;
    use axum::response::IntoResponse;

    let host = req
        .headers()
        .get(header::HOST)
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default();

    if host.starts_with("www.") {
        let new_host = host.trim_start_matches("www.");
        let uri = req.uri();
        let path = uri.path();
        let query = uri.query().map(|q| format!("?{}", q)).unwrap_or_default();

        // Always force HTTPS in the redirect as well
        let new_url = format!("https://{}{}{}", new_host, path, query);

        log::info!("Redirecting canonical: {} -> {}", host, new_url);
        return axum::response::Redirect::permanent(&new_url).into_response();
    }

    next.run(req).await
}
