//! Handyman Marketplace Frontend.
//!
//! Leptos-based SSR frontend for handyman service marketplace.
//!
//! # Architecture
//!
//! - `pages`: Route page components
//! - `components`: Reusable UI components  
//! - `api`: Backend API client functions

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

mod pages;
mod components;
// mod seo; // Removed, moved to components
pub mod api;

use crate::pages::home::Home;
use crate::pages::about::About;
use crate::pages::contact::Contact;
use crate::pages::pricing::Pricing;
use crate::pages::coventry::Coventry;
use crate::pages::blog::index::BlogIndex;
use crate::pages::blog::article::BlogArticle;
use crate::pages::electrician::Electrician;
use crate::pages::plumber::Plumber;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en-gb" />
        <Stylesheet id="leptos" href="/pkg/handyman.css"/>
        <Title text="Handyman Marketplace"/>
        
        <Router>
            <main>
                <Routes fallback=|| "Page not found.">
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/pricing") view=Pricing/>
                    <Route path=path!("/about") view=About/>
                    <Route path=path!("/contact") view=Contact/>
                    <Route path=path!("/coventry") view=Coventry/>
                    <Route path=path!("/electrician-coventry") view=Electrician/>
                    <Route path=path!("/plumber-coventry") view=Plumber/>
                    <Route path=path!("/blog") view=BlogIndex/>
                    <Route path=path!("/blog/:slug") view=BlogArticle/>
                </Routes>
            </main>
        </Router>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
