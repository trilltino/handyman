//! Custom 404 Not Found page.
//!
//! Provides a branded 404 page with helpful navigation.

use crate::components::seo::SeoHead;
use leptos::prelude::*;
use shared::PageMetadata;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <SeoHead metadata=PageMetadata {
            title: "Page Not Found - XF Tradesmen".to_string(),
            description: "The page you're looking for doesn't exist. Let's get you back on track.".to_string(),
            canonical_url: Some("https://xftradesman.com/404".to_string()),
            og_image: None,
        }/>

        <div class="min-h-screen bg-void flex items-center justify-center px-4">
            <div class="absolute inset-0 bg-cyber-grid bg-[length:40px_40px] opacity-10"></div>

            <div class="relative z-10 text-center max-w-2xl mx-auto">
                // Glitch-style 404
                <div class="mb-8">
                    <h1 class="text-[150px] md:text-[200px] font-heading font-black leading-none text-transparent bg-clip-text bg-gradient-to-b from-brand to-brand-deep tracking-tighter">
                        "404"
                    </h1>
                </div>

                <h2 class="text-3xl md:text-4xl font-heading font-bold text-white mb-4 tracking-tight">
                    "SIGNAL LOST"
                </h2>

                <p class="text-lg text-gray-400 mb-10 font-light">
                    "The page you're looking for has been moved, deleted, or never existed. Let's get you back on track."
                </p>

                <div class="flex flex-col sm:flex-row gap-4 justify-center">
                    <a href="/" class="btn-primary">
                        <span class="flex items-center justify-center gap-2">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/>
                            </svg>
                            "Return Home"
                        </span>
                    </a>
                    <a href="/contact" class="btn-secondary">
                        "Contact Support"
                    </a>
                </div>

                <div class="mt-16 pt-8 border-t border-void-highlight/30">
                    <p class="text-sm text-gray-500 mb-4">"Popular destinations:"</p>
                    <div class="flex flex-wrap justify-center gap-4 text-sm">
                        <a href="/pricing" class="text-gray-400 hover:text-brand transition">"Pricing"</a>
                        <span class="text-void-highlight">"•"</span>
                        <a href="/about" class="text-gray-400 hover:text-brand transition">"About Us"</a>
                        <span class="text-void-highlight">"•"</span>
                        <a href="/handyman" class="text-gray-400 hover:text-brand transition">"Handyman Sites"</a>
                        <span class="text-void-highlight">"•"</span>
                        <a href="/blog" class="text-gray-400 hover:text-brand transition">"Blog"</a>
                    </div>
                </div>
            </div>
        </div>
    }
}
