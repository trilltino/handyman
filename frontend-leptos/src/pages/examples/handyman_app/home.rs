//! Handyman Home Page.

use crate::components::seo::{LocalBusinessSchema, SeoHead};
use leptos::prelude::*;
use shared::PageMetadata;

#[component]
pub fn HandymanHome() -> impl IntoView {
    view! {
        <SeoHead metadata=PageMetadata {
            title: "Coventry Handyman Services | Reliable & Local Examples".to_string(),
            description: "Example site for a local handyman business. Shows how XFTradesmen builds high-converting websites.".to_string(),
            canonical_url: Some("https://xftradesmen.com/handyman-coventry".to_string()),
            og_image: None,
        }/>
        <LocalBusinessSchema />

         // Hero
            <section class="relative bg-gray-900 text-white py-24 px-6 md:px-12 overflow-hidden">
                <div class="absolute inset-0 bg-blue-950/80"></div>
                <div class="relative max-w-4xl mx-auto text-center space-y-6 animate-fade-in">
                    <span class="inline-block px-3 py-1 bg-yellow-500 text-blue-900 text-xs font-bold uppercase tracking-widest rounded-full mb-4">
                        "Serving Coventry & Warwickshire"
                    </span>
                    <h1 class="text-4xl md:text-6xl font-black leading-tight">
                        "YOUR RELIABLE LOCAL" <br/> <span class="text-yellow-400">"FIX-IT EXPERT"</span>
                    </h1>
                    <p class="text-lg md:text-xl text-gray-300 max-w-2xl mx-auto">
                        "From leaky taps to flat pack assembly, we handle the jobs you don't have time for. Professional, insured, and trusted."
                    </p>
                    <div class="pt-8 flex flex-col sm:flex-row justify-center gap-4">
                        <a href="/handyman-coventry/contact" class="px-8 py-4 bg-yellow-500 text-blue-900 font-bold rounded text-lg hover:bg-yellow-400 transition shadow-lg transform hover:-translate-y-1">
                            "Book a Job Now"
                        </a>
                        <a href="/handyman-coventry/services" class="px-8 py-4 bg-white/10 border border-white/20 font-bold rounded text-lg hover:bg-white/20 transition backdrop-blur-sm">
                            "View Our Services"
                        </a>
                    </div>
                </div>
            </section>

             // Services Preview
            <section class="py-20 px-6 md:px-12 bg-gray-50">
                <div class="max-w-6xl mx-auto">
                    <div class="text-center mb-16">
                        <h2 class="text-3xl font-bold text-blue-900 mb-4">"Our Core Services"</h2>
                        <div class="w-20 h-1 bg-yellow-500 mx-auto"></div>
                    </div>

                    <div class="grid md:grid-cols-3 gap-8">
                        <ServiceCard
                            title="Home Repairs"
                            desc="Drywall patching, door handle replacements, squeaky floor fixes, and general wear-and-tear repairs."
                            icon_path="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"
                        />
                        <ServiceCard
                            title="Assembly & Install"
                            desc="Flat pack furniture assembly, TV mounting, shelf installation, and picture hanging."
                            icon_path="M21 13.255A23.931 23.931 0 0112 15c-3.183 0-6.22-.62-9-1.745M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2 2v2m4 6h.01M5 20h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
                        />
                        <ServiceCard
                            title="Plumbing Basics"
                            desc="Leaky taps, toilet repairs, drain unclogging, and shower head replacements."
                            icon_path="M19.428 15.428a2 2 0 00-1.022-.547l-2.384-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"
                        />
                    </div>
                </div>
            </section>
    }
}

#[component]
fn ServiceCard(title: &'static str, desc: &'static str, icon_path: &'static str) -> impl IntoView {
    view! {
        <div class="bg-white p-8 rounded-xl shadow-sm border border-gray-100 hover:shadow-md transition">
            <div class="w-12 h-12 bg-blue-100 text-blue-900 flex items-center justify-center rounded-lg mb-6">
                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d=icon_path/></svg>
            </div>
            <h3 class="text-xl font-bold text-gray-900 mb-3">{title}</h3>
            <p class="text-gray-600">{desc}</p>
        </div>
    }
}
