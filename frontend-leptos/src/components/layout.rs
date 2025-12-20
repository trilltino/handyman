//! Navigation bar component with responsive design.

use leptos::prelude::*;

/// Fixed navigation bar with blur effect.
#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="navbar">
            <div class="navbar-inner">
                // Logo
                <a href="/" class="navbar-logo">
                    <span class="text-brand">"⚡"</span>
                    " XFTradesmen"
                </a>

                // Desktop Navigation
                <div class="hidden md:flex navbar-nav">
                    // Example Sites Dropdown
                    <div class="dropdown">
                        <button class="navbar-link dropdown-trigger">
                            "Example Sites"
                            <svg class="w-4 h-4 ml-1 dropdown-arrow" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                            </svg>
                        </button>
                        <div class="dropdown-menu">
                            <a href="/electrician-coventry" class="dropdown-item">
                                <svg class="w-5 h-5 text-yellow-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                                </svg>
                                "Electrician"
                            </a>
                            <a href="/plumber-coventry" class="dropdown-item">
                                <svg class="w-5 h-5 text-cyan-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.384-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"/>
                                </svg>
                                "Plumber"
                            </a>
                        </div>
                    </div>

                    <a href="/industries" class="navbar-link">"Industries"</a>
                    <a href="/packages" class="navbar-link">"Packages"</a>
                    <a href="/about" class="navbar-link">"About"</a>
                    <a href="/blog" class="navbar-link">"Blog"</a>

                    <a href="/contact" class="btn btn-primary btn-sm">
                        "Contact"
                    </a>
                </div>

                // Mobile Menu Button
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
                            <span class="text-brand">"⚡"</span>" XFTradesmen"
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
                            <li><a href="/packages" class="text-gray-500 hover:text-brand-light text-sm">"Packages"</a></li>
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
                    <p>"© 2024 XF Tradesmen. All rights reserved."</p>
                    <div class="flex gap-6 mt-4 md:mt-0">
                        <a href="#" class="hover:text-brand-light">"Privacy"</a>
                        <a href="#" class="hover:text-brand-light">"Terms"</a>
                    </div>
                </div>
            </div>
        </footer>
    }
}
