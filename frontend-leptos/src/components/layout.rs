//! Navigation bar component with responsive design.

use leptos::prelude::*;
use leptos_router::components::A;

/// Fixed navigation bar with dropdown menus.
#[component]
pub fn Navbar() -> impl IntoView {
    // State for mobile menu
    let (is_open, set_is_open) = signal(false);

    // Close menu when a link is clicked
    let close_menu = move |_| set_is_open.set(false);

    view! {
        <nav class="fixed top-0 left-0 right-0 z-50">
            // Main Navbar Content - Seamless (Transparent)
            <div class="w-full px-6 h-24 flex items-center justify-between transition-all duration-300">
                // Left: Brand Logo & Tagline
                <div class="flex items-center gap-6">
                    <A href="/" attr:class="flex items-center gap-3" on:click=close_menu>
                        <span class="text-2xl md:text-3xl font-black tracking-tight" style="color: white;">
                            "XFTradesman.com"
                        </span>
                    </A>
                    // Vertical Divider
                    <div class="hidden md:block w-px h-8 bg-white/20"></div>
                    // Tagline
                    <div class="hidden md:flex flex-col justify-center">
                        <span class="text-xs font-bold tracking-wide leading-tight" style="color: #d1d5db;">"Transforming Local Business"</span>
                        <span class="text-xs tracking-wide leading-tight" style="color: #6b7280;">"Websites with expert design"</span>
                    </div>
                </div>

                // Right: Navigation & Burger
                <div class="flex items-center gap-8 md:gap-12">
                    // Desktop Links
                    <div class="hidden md:flex items-center gap-8">
                        <A href="/packages" attr:class="text-sm font-black hover:opacity-80 transition-opacity uppercase tracking-widest" attr:style="color: white;">"PACKAGES"</A>
                        <A href="/handyman-coventry" attr:class="text-sm font-black hover:opacity-80 transition-opacity uppercase tracking-widest" attr:style="color: white;">"EXAMPLE"</A>
                        <A href="/contact" attr:class="text-sm font-black hover:opacity-80 transition-opacity uppercase tracking-widest" attr:style="color: white;">"CONTACT"</A>
                    </div>

                    // Burger Menu (Visible on Desktop too as per design, or predominantly mobile)
                    // The user's design shows links AND the burger.
                    <button
                        class="text-white p-2 focus:outline-none hover:text-brand transition-colors"
                        on:click=move |_| set_is_open.update(|v| *v = !*v)
                        aria-label="Toggle menu"
                    >
                         {move || if is_open.get() {
                            view! {
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                </svg>
                            }.into_any()
                        } else {
                            view! {
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                                </svg>
                            }.into_any()
                        }}
                    </button>
                </div>
            </div>

            // Mobile/Drawer Dropdown Menu
            <div class=move || format!(
                "absolute top-full left-0 right-0 bg-black/95 border-b border-white/10 transition-all duration-300 ease-in-out overflow-hidden {}",
                if is_open.get() { "max-h-screen opacity-100 border-t border-white/10" } else { "max-h-0 opacity-0 border-none" }
            )>
                <div class="flex flex-col px-8 py-12 space-y-8 text-center min-h-[50vh] justify-center">
                    <A href="/" attr:class="text-lg font-bold text-white hover:text-gray-300" on:click=close_menu>"Home"</A>
                    <A href="/about" attr:class="text-lg font-bold text-white hover:text-gray-300" on:click=close_menu>"About"</A>
                    <A href="/services" attr:class="text-lg font-bold text-white hover:text-gray-300" on:click=close_menu>"Services"</A>
                    <A href="/packages" attr:class="text-lg font-bold text-white hover:text-gray-300" on:click=close_menu>"Packages"</A>
                    <A href="/handyman-coventry" attr:class="text-lg font-bold text-white hover:text-gray-300" on:click=close_menu>"Example"</A>
                    <A href="/reviews" attr:class="text-lg font-bold text-white hover:text-gray-300" on:click=close_menu>"Reviews"</A>
                    <A href="/contact" attr:class="inline-block px-8 py-3 bg-white text-black font-black uppercase tracking-widest hover:bg-gray-200 mx-auto" on:click=close_menu>
                        "Contact Us"
                    </A>
                </div>
            </div>
        </nav>
    }
}

/// Site footer with links and copyright.
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-black border-t border-white/10 pt-20 pb-10 font-sans">
            <div class="max-w-7xl mx-auto px-6">
                <div class="grid md:grid-cols-4 gap-12 mb-12">
                    // Brand
                    <div class="md:col-span-2">
                        <a href="/" class="text-xl font-bold text-white mb-4 inline-block tracking-tight">
                            "XFTradesmen"
                        </a>
                        <p class="text-gray-500 max-w-sm text-sm leading-relaxed">
                            "Professional handyman services in Coventry and surrounding areas. Licensed, insured, and ready to help."
                        </p>
                    </div>

                    // Company Links
                    <div>
                        <h4 class="text-xs font-bold text-white uppercase tracking-[0.2em] mb-4">
                            "Company"
                        </h4>
                        <ul class="space-y-3">
                            <li><a href="/about" class="text-gray-500 hover:text-white transition-colors text-sm">"About Us"</a></li>
                            <li><a href="/contact" class="text-gray-500 hover:text-white transition-colors text-sm">"Contact"</a></li>
                            <li><a href="/blog" class="text-gray-500 hover:text-white transition-colors text-sm">"Blog"</a></li>
                            <li><a href="/faq" class="text-gray-500 hover:text-white transition-colors text-sm">"FAQ"</a></li>
                        </ul>
                    </div>

                    // Legal Links
                    <div>
                        <h4 class="text-xs font-bold text-white uppercase tracking-[0.2em] mb-4">
                            "Legal"
                        </h4>
                        <ul class="space-y-3">
                            <li><a href="/service-agreement" class="text-gray-500 hover:text-white transition-colors text-sm">"Service Agreement"</a></li>
                            <li><a href="/terms" class="text-gray-500 hover:text-white transition-colors text-sm">"Terms & Conditions"</a></li>
                            <li><a href="/privacy" class="text-gray-500 hover:text-white transition-colors text-sm">"Privacy Policy"</a></li>
                        </ul>
                    </div>
                </div>

                // Bottom
                <div class="flex flex-col md:flex-row justify-between items-center pt-8 border-t border-white/10 text-gray-600">
                    <p class="text-xs">"© 2025 XF Tradesmen. All rights reserved."</p>
                    <div class="flex gap-6 mt-4 md:mt-0 font-medium text-xs">
                        <a href="/privacy" class="hover:text-white transition-colors">"Privacy"</a>
                        <a href="/terms" class="hover:text-white transition-colors">"Terms"</a>
                    </div>
                </div>
            </div>
        </footer>
    }
}
