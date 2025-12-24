//! Handyman Marketplace Frontend.
//!
//! Leptos-based SSR frontend for handyman service marketplace.
//!
//! # Architecture
//!
//! - `pages`: Route page components
//! - `components`: Reusable UI components  
//! - `api`: Backend API client functions

// Increase recursion limit for complex Leptos view hierarchies
#![recursion_limit = "512"]

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path,
};

pub mod api;
mod components;
mod design_system;
mod pages;
use crate::pages::about::About;
use crate::pages::blog::article::BlogArticle;
use crate::pages::blog::index::BlogIndex;
use crate::pages::contact::Contact;
use crate::pages::coventry::Coventry;
use crate::pages::examples::handyman_app::components::layout::HandymanLayout;
// Admin pages
use crate::pages::examples::handyman_app::pages::admin::AdminDashboard;
// Main pages
use crate::pages::examples::handyman_app::pages::main::HandymanAreaPage;
use crate::pages::examples::handyman_app::pages::main::HandymanFeatures;
use crate::pages::examples::handyman_app::pages::main::HandymanHome;
use crate::pages::examples::handyman_app::pages::main::HandymanServiceDetail;
use crate::pages::examples::handyman_app::pages::main::HandymanServiceMap;
use crate::pages::examples::handyman_app::pages::main::HandymanServices;
// Booking pages
use crate::pages::examples::handyman_app::pages::booking::AdminCustomers;
use crate::pages::examples::handyman_app::pages::booking::HandymanBooking;
use crate::pages::examples::handyman_app::pages::booking::HandymanEmergency;
use crate::pages::examples::handyman_app::pages::booking::HandymanQuote;
// Content pages
use crate::pages::examples::handyman_app::pages::content::HandymanAbout;
use crate::pages::examples::handyman_app::pages::content::HandymanBlog;
use crate::pages::examples::handyman_app::pages::content::HandymanContact;
use crate::pages::examples::handyman_app::pages::content::HandymanTestimonials;
use crate::pages::examples::handyman_app::pages::content::{PrivacyPolicy, TermsOfService};

use crate::pages::handyman::Handyman;
use crate::pages::home::Home;
use crate::pages::packages::Packages;
use crate::pages::pricing::Pricing;

#[component]
fn MainLayout() -> impl IntoView {
    view! {
        <crate::components::layout::Navbar />
         <main class="pt-16">
            <Outlet/>
        </main>
        <crate::components::layout::Footer />
    }
}

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
            <Routes fallback=|| "Page not found.">
                // Main Site Routes
                <ParentRoute path=path!("/") view=MainLayout>
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
                </ParentRoute>

                // Handyman Example App Routes
                <ParentRoute path=path!("/handyman-coventry") view=HandymanLayout>
                    <Route path=path!("/") view=HandymanHome/>
                    <Route path=path!("/services") view=HandymanServices/>
                    <Route path=path!("/services/:slug") view=HandymanServiceDetail/>
                    <Route path=path!("/features") view=HandymanFeatures/>
                    <Route path=path!("/testimonials") view=HandymanTestimonials/>
                    <Route path=path!("/service-area") view=HandymanServiceMap/>
                    <Route path=path!("/booking") view=HandymanBooking/>
                    <Route path=path!("/quote") view=HandymanQuote/>
                    <Route path=path!("/about") view=HandymanAbout/>
                    <Route path=path!("/contact") view=HandymanContact/>
                    <Route path=path!("/admin") view=AdminDashboard/>
                    <Route path=path!("/admin/customers") view=AdminCustomers/>
                    <Route path=path!("/blog") view=HandymanBlog/>
                    <Route path=path!("/areas/:area") view=HandymanAreaPage/>
                    <Route path=path!("/emergency") view=HandymanEmergency/>
                    <Route path=path!("/privacy") view=PrivacyPolicy/>
                    <Route path=path!("/terms") view=TermsOfService/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
