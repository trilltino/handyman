//! Navigation bar component with responsive design.

use leptos::prelude::*;

/// Site navigation links.
const NAV_LINKS: &[(&str, &str)] = &[
    ("Services", "/pricing"),
    ("Electrician", "/electrician-coventry"),
    ("Plumber", "/plumber-coventry"),
    ("About", "/about"),
    ("Blog", "/blog"),
    ("Contact", "/contact"),
];

/// Fixed navigation bar with blur effect.
#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="navbar">
            <div class="navbar-inner">
                // Logo
                <a href="/" class="navbar-logo">
                    <span class="text-brand">"⚡"</span>
                    " Handyman"
                </a>
                
                // Desktop Navigation
                <div class="hidden md:flex navbar-nav">
                    {NAV_LINKS.iter().map(|(label, href)| {
                        view! {
                            <a href=*href class="navbar-link">{*label}</a>
                        }
                    }).collect::<Vec<_>>()}
                    
                    <a href="/contact" class="btn btn-primary btn-sm">
                        "Get Quote"
                    </a>
                </div>
                
                // Mobile Menu Button (basic - could be enhanced with JS)
                <button class="md:hidden btn-ghost" aria-label="Menu">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
                    </svg>
                </button>
            </div>
        </nav>
    }
}

/// Site footer with links and copyright.
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-void-surface border-t border-void-highlight">
            <div class="container section">
                <div class="grid md:grid-cols-4 gap-8 mb-12">
                    // Brand
                    <div class="md:col-span-2">
                        <a href="/" class="text-2xl font-bold text-white mb-4 inline-block">
                            <span class="text-brand">"⚡"</span>" Handyman"
                        </a>
                        <p class="text-gray-500 max-w-sm">
                            "Professional handyman services in Coventry and surrounding areas. 
                            Licensed, insured, and ready to help."
                        </p>
                    </div>
                    
                    // Services
                    <div>
                        <h4 class="text-sm font-semibold text-gray-300 uppercase tracking-wider mb-4">
                            "Services"
                        </h4>
                        <ul class="space-y-2">
                            <li><a href="/electrician-coventry" class="text-gray-500 hover:text-brand-light text-sm">"Electrical"</a></li>
                            <li><a href="/plumber-coventry" class="text-gray-500 hover:text-brand-light text-sm">"Plumbing"</a></li>
                            <li><a href="/pricing" class="text-gray-500 hover:text-brand-light text-sm">"All Services"</a></li>
                        </ul>
                    </div>
                    
                    // Company
                    <div>
                        <h4 class="text-sm font-semibold text-gray-300 uppercase tracking-wider mb-4">
                            "Company"
                        </h4>
                        <ul class="space-y-2">
                            <li><a href="/about" class="text-gray-500 hover:text-brand-light text-sm">"About Us"</a></li>
                            <li><a href="/contact" class="text-gray-500 hover:text-brand-light text-sm">"Contact"</a></li>
                            <li><a href="/blog" class="text-gray-500 hover:text-brand-light text-sm">"Blog"</a></li>
                        </ul>
                    </div>
                </div>
                
                // Bottom
                <div class="flex flex-col md:flex-row justify-between items-center pt-8 border-t border-void-highlight text-gray-600 text-sm">
                    <p>"© 2024 Handyman Marketplace. All rights reserved."</p>
                    <div class="flex gap-6 mt-4 md:mt-0">
                        <a href="#" class="hover:text-brand-light">"Privacy"</a>
                        <a href="#" class="hover:text-brand-light">"Terms"</a>
                    </div>
                </div>
            </div>
        </footer>
    }
}
