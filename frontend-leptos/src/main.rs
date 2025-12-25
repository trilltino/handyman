//! Frontend application entry point.
//!
//! Handles SSR server setup (when `ssr` feature is enabled).

// Increase recursion limit for complex Leptos view hierarchies in SSR
#![recursion_limit = "1024"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        routing::{any, get},
        Router,
    };
    use frontend_leptos::App;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

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
        .route("/sitemap.xml", get(proxy_handler))
        .route("/robots.txt", get(proxy_handler))
        .route("/api/{*fn_name}", any(proxy_handler)) // Proxy API requests
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // Start the server
    log::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "ssr")]
async fn proxy_handler(req: axum::extract::Request) -> axum::response::Response {
    use axum::response::IntoResponse;

    // The backend API URL - usually running on localhost:3001 in production
    // or defined via environment variable
    let api_url = std::env::var("API_URL").unwrap_or_else(|_| "http://127.0.0.1:3001".to_string());

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

    let req_builder = client.request(parts.method, uri).headers(parts.headers);

    // Axum body to Reqwest body conversion is complex, simpler to stream bytes
    // For now, let's just forward as is if possible, or read bytes
    // Using bytes is safer for simple proxying
    let bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .unwrap_or_default();

    let resp = req_builder.body(bytes).send().await;
    match resp {
        Ok(resp) => {
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
            log::error!("Proxy error: {}", e);
            (
                axum::http::StatusCode::BAD_GATEWAY,
                format!("Proxy error: {}", e),
            )
                .into_response()
        }
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function
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
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() />
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
