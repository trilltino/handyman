//! # Root App Component
//!
//! ## Purpose
//! Root component of the Yew application.
//! Sets up routing, navigation, and global context providers.
//!
//! ## Component Structure
//! ```
//! App
//! ├── AuthProvider (authentication context)
//! │   └── BrowserRouter (client-side routing)
//! │       ├── Nav (navigation bar)
//! │       └── main
//! │           └── Switch (route-based page rendering)
//! ```
//!
//! ## Routes
//! - `/` → Home page
//! - `/login` → Login form
//! - `/register` → Registration form
//! - `/booking` → Service booking form
//! - `/contact` → Contact form
//! - `/experience` → Handyman experience/services page
//! - `/map` → Interactive Cesium 3D map
//!
//! ## Relation to Entire Program
//! - **Root Component**: Rendered by main.rs
//! - **Provides**: Auth context to all child components
//! - **Routing**: Client-side SPA navigation

use yew::prelude::*;        // Yew framework (html! macro, etc.)
use yew_router::prelude::*; // Client-side routing

use crate::components::Nav;  // Navigation bar component
use crate::contexts::AuthProvider;  // Authentication context provider
use crate::pages::{Booking, Contact, Experience, Home, Login, Register, MapPage};  // Page components
use crate::Route;  // Route enum

/// Root App component
/// Sets up global providers and routing
#[function_component]
pub fn App() -> Html {
    html! {
        // Wrap entire app in AuthProvider for auth context
        <AuthProvider>
            // Set up client-side routing
            <BrowserRouter>
                // Navigation bar (visible on all pages)
                <Nav />
                // Main content area (route-specific)
                <main>
                    // Switch renders component based on current route
                    <Switch<Route> render={switch} />
                </main>
            </BrowserRouter>
        </AuthProvider>
    }
}

/// Route switch function
/// Maps Route enum to page components
fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Login => html! { <Login /> },
        Route::Register => html! { <Register /> },
        Route::Booking => html! { <Booking /> },
        Route::Contact => html! { <Contact /> },
        Route::Experience => html! { <Experience /> },
        Route::Map => html! { <MapPage /> },
        Route::NotFound => html! {
            <div class="error-page">
                <h1>{ "404 - Page Not Found" }</h1>
                <p>{ "The page you're looking for doesn't exist." }</p>
            </div>
        },
    }
}
