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

mod components;
mod pages;
// mod seo; // Removed, moved to components
pub mod api;
use crate::pages::about::About;
use crate::pages::blog::article::BlogArticle;
use crate::pages::blog::index::BlogIndex;
use crate::pages::contact::Contact;
use crate::pages::coventry::Coventry;
use crate::pages::handyman::Handyman;
use crate::pages::home::Home;
use crate::pages::packages::Packages;
use crate::pages::pricing::Pricing;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en-gb" />
        <Stylesheet id="leptos" href="/xftradesmen.css"/>
        <Title text="XFTradesmen"/>

        // Import Google Fonts
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
        <Link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&family=Outfit:wght@400;500;600;700;800;900&display=swap" rel="stylesheet"/>

        <Router>
            // Fixed Navigation
            <crate::components::layout::Navbar />

            // Main Content (with top padding for fixed nav)
            <main class="pt-16">
                <Routes fallback=|| "Page not found.">
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/pricing") view=Pricing/>
                    <Route path=path!("/packages") view=Packages/>
                    <Route path=path!("/about") view=About/>
                    <Route path=path!("/contact") view=Contact/>
                    <Route path=path!("/coventry") view=Coventry/>
                    <Route path=path!("/handyman") view=Handyman/>
                    <Route path=path!("/blog") view=BlogIndex/>
                    <Route path=path!("/blog/:slug") view=BlogArticle/>
                    <Route path=path!("/industries") view=crate::pages::industries::Industries/>
                </Routes>
            </main>

            // Footer
            <crate::components::layout::Footer />
        </Router>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
