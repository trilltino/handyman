//! # Route Definitions
//!
//! ## Purpose
//! Defines all client-side routes for the single-page application.
//! Used by yew-router for URL-based navigation.
//!
//! ## Routes
//! - `/` - Home page (landing page with hero, features)
//! - `/login` - User login form
//! - `/register` - User registration form
//! - `/booking` - Service booking form
//! - `/contact` - Contact us form
//! - `/experience` - Handyman experience/portfolio page
//! - `/map` - Interactive 3D Cesium map
//! - `/404` - Not found page (catch-all)
//!
//! ## Relation to Entire Program
//! - **Used By**: App component (route switching), Nav component (links)
//! - **Client-Side**: No page reloads, updates URL and renders component
//! - **History API**: Uses browser history for back/forward buttons

use yew_router::prelude::*;  // Yew router macros and traits

/// Route enum for client-side navigation
/// Each variant maps to a URL path
#[derive(Clone, Routable, PartialEq, Eq, Debug)]
pub enum Route {
    #[at("/")]
    Home,           // Landing page
    #[at("/login")]
    Login,          // Login form
    #[at("/register")]
    Register,       // Registration form
    #[at("/booking")]
    Booking,        // Booking form
    #[at("/contact")]
    Contact,        // Contact form
    #[at("/experience")]
    Experience,     // Handyman experience page
    #[at("/map")]
    Map,            // Interactive 3D map
    #[not_found]
    #[at("/404")]
    NotFound,       // 404 page (fallback for invalid routes)
}
