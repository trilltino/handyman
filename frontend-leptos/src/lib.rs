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
#![recursion_limit = "1024"]

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path,
};

pub mod api;
mod components;
mod pages;

// Main pages
use crate::pages::main::About;
use crate::pages::main::Contact;
use crate::pages::main::Home;
use crate::pages::main::Industries;
use crate::pages::main::Packages;
use crate::pages::main::Pricing;

// Legal pages
use crate::pages::legal::ServiceAgreement;
use crate::pages::legal::Terms;

// Location pages
use crate::pages::locations::Coventry;
use crate::pages::locations::Handyman;

// Info pages
use crate::pages::info::Faq;
use crate::pages::info::NotFound;

// Blog pages
use crate::pages::blog::article::BlogArticle;
use crate::pages::blog::index::BlogIndex;

// Handyman example app
use crate::pages::examples::handyman_app::components::layout::HandymanLayout;
use crate::pages::examples::handyman_app::pages::admin::AdminDashboard;
use crate::pages::examples::handyman_app::pages::booking::AdminCustomers;
use crate::pages::examples::handyman_app::pages::booking::HandymanBooking;
use crate::pages::examples::handyman_app::pages::booking::HandymanEmergency;
use crate::pages::examples::handyman_app::pages::booking::HandymanQuote;
use crate::pages::examples::handyman_app::pages::content::HandymanAbout;
use crate::pages::examples::handyman_app::pages::content::HandymanBlog;
use crate::pages::examples::handyman_app::pages::content::HandymanContact;
use crate::pages::examples::handyman_app::pages::content::HandymanTestimonials;
use crate::pages::examples::handyman_app::pages::content::{
    HandymanFaq, PrivacyPolicy, TermsOfService,
};
use crate::pages::examples::handyman_app::pages::main::HandymanAreaPage;
use crate::pages::examples::handyman_app::pages::main::HandymanFeatures;
use crate::pages::examples::handyman_app::pages::main::HandymanHome;
use crate::pages::examples::handyman_app::pages::main::HandymanServiceDetail;
use crate::pages::examples::handyman_app::pages::main::HandymanServiceMap;
use crate::pages::examples::handyman_app::pages::main::HandymanServices;
use crate::pages::examples::handyman_app::pages::services::{
    FurnitureAssembly, MountingInstallation, PlumbingRepairs,
};

#[component]
fn MainLayout() -> impl IntoView {
    view! {
        <div class="min-h-screen text-white font-sans selection:bg-brand selection:text-white overflow-x-hidden bg-gradient-to-br from-red-950 via-black to-black">
            <crate::components::layout::Navbar />
            <main>
                <Outlet/>
            </main>
            <crate::components::layout::Footer />
        </div>
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
            <Routes fallback=NotFound>
                // Main Site Routes
                <ParentRoute path=path!("/") view=MainLayout>
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/pricing") view=Pricing/>
                    <Route path=path!("/packages") view=Packages/>
                    <Route path=path!("/about") view=About/>
                    <Route path=path!("/contact") view=Contact/>
                    <Route path=path!("/coventry") view=Coventry/>

                    <Route path=path!("/blog") view=BlogIndex/>
                    <Route path=path!("/blog/:slug") view=BlogArticle/>
                    <Route path=path!("/industries") view=Industries/>
                    <Route path=path!("/terms") view=Terms/>
                    <Route path=path!("/faq") view=Faq/>
                    <Route path=path!("/service-agreement") view=ServiceAgreement/>
                </ParentRoute>

                // Handyman Example App Routes
                <ParentRoute path=path!("/handyman-coventry") view=HandymanLayout>
                    <Route path=path!("/") view=HandymanHome/>
                    <Route path=path!("/services") view=HandymanServices/>
                    <Route path=path!("/services/furniture-assembly") view=FurnitureAssembly/>
                    <Route path=path!("/services/plumbing") view=PlumbingRepairs/>
                    <Route path=path!("/services/mounting") view=MountingInstallation/>
                    <Route path=path!("/services/:slug") view=HandymanServiceDetail/>
                    <Route path=path!("/faq") view=HandymanFaq/>
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
