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
                            <a href="/handyman-coventry" class="dropdown-item">
                                <svg class="w-5 h-5 text-brand" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                                </svg>
                                "Handyman"
                            </a>
                        </div>
                    </div>

                    <a href="/industries" class="navbar-link">"Industries"</a>
                    <a href="/packages" class="navbar-link">"Packages"</a>

                    // About Dropdown with Blog
                    <div class="dropdown">
                        <button class="navbar-link dropdown-trigger">
                            "About"
                            <svg class="w-4 h-4 ml-1 dropdown-arrow" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                            </svg>
                        </button>
                        <div class="dropdown-menu">
                            <a href="/about" class="dropdown-item">
                                <svg class="w-5 h-5 text-brand" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                </svg>
                                "About Us"
                            </a>
                            <a href="/blog" class="dropdown-item">
                                <svg class="w-5 h-5 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"/>
                                </svg>
                                "Blog"
                            </a>
                        </div>
                    </div>

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
