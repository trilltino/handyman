//! Navigation bar component with responsive design.

use leptos::prelude::*;

/// Fixed navigation bar with blur effect.
#[component]
pub fn Navbar() -> impl IntoView {
    let (mobile_open, set_mobile_open) = signal(false);

    view! {
        <nav class="navbar">
            <div class="navbar-inner">
                // Logo
                <a href="/" class="navbar-logo">
                    <svg class="w-5 h-5 inline text-brand" fill="currentColor" viewBox="0 0 20 20"><path d="M11.3 1.046A1 1 0 0112 2v5h4a1 1 0 01.82 1.573l-7 10A1 1 0 018 18v-5H4a1 1 0 01-.82-1.573l7-10a1 1 0 01.12-.38z"/></svg>
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
                                <div>
                                    <div class="font-medium">"Handyman Coventry"</div>
                                    <div class="text-xs text-gray-500">"Local service business demo"</div>
                                </div>
                            </a>
                        </div>
                    </div>

                    <a href="/industries" class="navbar-link">"Industries"</a>
                    <a href="/packages" class="navbar-link">"Packages"</a>
                    <a href="/blog" class="navbar-link">"Blog"</a>

                    // About Dropdown
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
                            <a href="/service-agreement" class="dropdown-item">
                                <svg class="w-5 h-5 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                                </svg>
                                "Service Agreement"
                            </a>
                            <a href="/faq" class="dropdown-item">
                                <svg class="w-5 h-5 text-emerald-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                </svg>
                                "FAQ"
                            </a>
                            <a href="/terms" class="dropdown-item">
                                <svg class="w-5 h-5 text-amber-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>
                                </svg>
                                "Terms & Conditions"
                            </a>
                        </div>
                    </div>

                    <a href="/contact" class="btn btn-primary btn-sm">
                        "Contact"
                    </a>
                </div>

                // Mobile Menu Button (Hamburger)
                <button
                    class="md:hidden flex flex-col gap-1.5 p-2 z-50"
                    on:click=move |_| set_mobile_open.update(|v| *v = !*v)
                    aria-label="Toggle menu"
                >
                    <span class={move || format!("w-6 h-0.5 bg-white transition-all duration-300 {}",
                        if mobile_open.get() { "rotate-45 translate-y-2" } else { "" })}></span>
                    <span class={move || format!("w-6 h-0.5 bg-white transition-all duration-300 {}",
                        if mobile_open.get() { "opacity-0" } else { "" })}></span>
                    <span class={move || format!("w-6 h-0.5 bg-white transition-all duration-300 {}",
                        if mobile_open.get() { "-rotate-45 -translate-y-2" } else { "" })}></span>
                </button>
            </div>
        </nav>

        // Mobile Menu Overlay
        <div
            class={move || format!("md:hidden fixed inset-0 bg-black/50 z-40 transition-opacity duration-300 {}",
                if mobile_open.get() { "opacity-100 pointer-events-auto" } else { "opacity-0 pointer-events-none" })}
            on:click=move |_| set_mobile_open.set(false)
        ></div>

        // Mobile Menu Panel
        <nav
            class={move || format!("md:hidden fixed top-0 right-0 h-full w-72 bg-void-surface z-50 flex flex-col pt-20 px-6 gap-4 shadow-2xl transition-transform duration-300 border-l border-void-highlight {}",
                if mobile_open.get() { "translate-x-0" } else { "translate-x-full" })}
        >
            <a href="/handyman-coventry" class="text-white font-medium text-lg py-3 border-b border-void-highlight hover:text-brand-light"
               on:click=move |_| set_mobile_open.set(false)>"Handyman Coventry"</a>
            <a href="/industries" class="text-white font-medium text-lg py-3 border-b border-void-highlight hover:text-brand-light"
               on:click=move |_| set_mobile_open.set(false)>"Industries"</a>
            <a href="/packages" class="text-white font-medium text-lg py-3 border-b border-void-highlight hover:text-brand-light"
               on:click=move |_| set_mobile_open.set(false)>"Packages"</a>
            <a href="/blog" class="text-white font-medium text-lg py-3 border-b border-void-highlight hover:text-brand-light"
               on:click=move |_| set_mobile_open.set(false)>"Blog"</a>
            <a href="/about" class="text-white font-medium text-lg py-3 border-b border-void-highlight hover:text-brand-light"
               on:click=move |_| set_mobile_open.set(false)>"About Us"</a>
            <a href="/service-agreement" class="text-white font-medium text-lg py-3 border-b border-void-highlight hover:text-brand-light"
               on:click=move |_| set_mobile_open.set(false)>"Service Agreement"</a>
            <a href="/faq" class="text-white font-medium text-lg py-3 border-b border-void-highlight hover:text-brand-light"
               on:click=move |_| set_mobile_open.set(false)>"FAQ"</a>
            <a href="/terms" class="text-white font-medium text-lg py-3 border-b border-void-highlight hover:text-brand-light"
               on:click=move |_| set_mobile_open.set(false)>"Terms & Conditions"</a>
            <a href="/contact"
               class="mt-4 btn btn-primary text-center"
               on:click=move |_| set_mobile_open.set(false)>"Contact"</a>
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
                            <svg class="w-5 h-5 inline text-brand" fill="currentColor" viewBox="0 0 20 20"><path d="M11.3 1.046A1 1 0 0112 2v5h4a1 1 0 01.82 1.573l-7 10A1 1 0 018 18v-5H4a1 1 0 01-.82-1.573l7-10a1 1 0 01.12-.38z"/></svg>" XFTradesmen"
                        </a>
                        <p class="text-gray-500 max-w-sm">
                            "Professional handyman services in Coventry and surrounding areas.
                            Licensed, insured, and ready to help."
                        </p>
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
                            <li><a href="/faq" class="text-gray-500 hover:text-brand-light text-sm">"FAQ"</a></li>
                            <li><a href="/service-agreement" class="text-gray-500 hover:text-brand-light text-sm">"Service Agreement"</a></li>
                            <li><a href="/terms" class="text-gray-500 hover:text-brand-light text-sm">"Terms & Conditions"</a></li>
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
