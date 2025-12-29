//! Navigation bar component with responsive design.

use leptos::prelude::*;

/// Fixed navigation bar with dropdown menus.
#[component]
pub fn Navbar() -> impl IntoView {
    let (mobile_open, set_mobile_open) = signal(false);
    let (main_open, set_main_open) = signal(false);
    let (resources_open, set_resources_open) = signal(false);
    let (example_open, set_example_open) = signal(false);
    let (legal_open, set_legal_open) = signal(false);

    view! {
        // Navbar with Dropdown Menus
        <nav class="fixed top-0 left-0 right-0 z-50 bg-black/80 backdrop-blur-md">
            <div class="px-6 md:px-12 py-8 md:py-10 flex items-center justify-between">
                // Left: Brand Logo without Separator
                <a href="/" class="flex items-center gap-3 md:gap-6">
                    <span class="text-lg md:text-xl lg:text-2xl xl:text-3xl font-black tracking-tight text-white">
                        "XFTradesman.com"
                    </span>
                    <div class="hidden md:flex flex-col leading-tight">
                        <span class="text-xs lg:text-sm font-medium tracking-tight text-white">
                            "Transforming Local Business"
                        </span>
                        <span class="text-xs lg:text-sm font-medium tracking-tight text-white">
                            "Websites with expert design"
                        </span>
                    </div>
                </a>

                // Center: Desktop Dropdown Navigation (hidden on mobile)
                <div class="hidden lg:flex items-center gap-8">
                    // Main Dropdown
                    <div class="group relative">
                        <button class="text-sm font-bold text-white/80 hover:text-white uppercase tracking-wide py-2 flex items-center gap-1">
                            "Main"
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                            </svg>
                        </button>
                        <div class="absolute left-0 top-full pt-2 invisible group-hover:visible opacity-0 group-hover:opacity-100 transition-all duration-200">
                            <div class="bg-black/95 backdrop-blur-md border border-white/10 rounded-lg shadow-2xl py-2 min-w-[200px]">
                                <a href="/" class="block px-6 py-2 text-sm text-white/70 hover:text-white hover:bg-white/5 transition-colors">"Home"</a>
                                <a href="/industries" class="block px-6 py-2 text-sm text-white/70 hover:text-white hover:bg-white/5 transition-colors">"Industries"</a>
                                <a href="/about" class="block px-6 py-2 text-sm text-white/70 hover:text-white hover:bg-white/5 transition-colors">"About"</a>
                            </div>
                        </div>
                    </div>

                    // Resources Dropdown
                    <div class="group relative">
                        <button class="text-sm font-bold text-white/80 hover:text-white uppercase tracking-wide py-2 flex items-center gap-1">
                            "Resources"
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                            </svg>
                        </button>
                        <div class="absolute left-0 top-full pt-2 invisible group-hover:visible opacity-0 group-hover:opacity-100 transition-all duration-200">
                            <div class="bg-black/95 backdrop-blur-md border border-white/10 rounded-lg shadow-2xl py-2 min-w-[200px]">
                                <a href="/blog" class="block px-6 py-2 text-sm text-white/70 hover:text-white hover:bg-white/5 transition-colors">"Blog"</a>
                                <a href="/faq" class="block px-6 py-2 text-sm text-white/70 hover:text-white hover:bg-white/5 transition-colors">"FAQ"</a>
                                <a href="/service-agreement" class="block px-6 py-2 text-sm text-white/70 hover:text-white hover:bg-white/5 transition-colors">"Service Agreement"</a>
                            </div>
                        </div>
                    </div>

                    // Example Dropdown
                    <div class="group relative">
                        <button class="text-sm font-bold text-white/80 hover:text-white uppercase tracking-wide py-2 flex items-center gap-1">
                            "Example"
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                            </svg>
                        </button>
                        <div class="absolute left-0 top-full pt-2 invisible group-hover:visible opacity-0 group-hover:opacity-100 transition-all duration-200">
                            <div class="bg-black/95 backdrop-blur-md border border-white/10 rounded-lg shadow-2xl py-2 min-w-[200px]">
                                <a href="/handyman-coventry" class="block px-6 py-2 text-sm text-white/70 hover:text-white hover:bg-white/5 transition-colors">"The Handyman"</a>
                            </div>
                        </div>
                    </div>

                    // Legal Dropdown
                    <div class="group relative">
                        <button class="text-sm font-bold text-white/80 hover:text-white uppercase tracking-wide py-2 flex items-center gap-1">
                            "Legal"
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                            </svg>
                        </button>
                        <div class="absolute left-0 top-full pt-2 invisible group-hover:visible opacity-0 group-hover:opacity-100 transition-all duration-200">
                            <div class="bg-black/95 backdrop-blur-md border border-white/10 rounded-lg shadow-2xl py-2 min-w-[200px]">
                                <a href="/terms" class="block px-6 py-2 text-sm text-white/70 hover:text-white hover:bg-white/5 transition-colors">"Terms & Conditions"</a>
                                <a href="/privacy" class="block px-6 py-2 text-sm text-white/70 hover:text-white hover:bg-white/5 transition-colors">"Privacy Policy"</a>
                            </div>
                        </div>
                    </div>
                </div>

                // Right: CTA Buttons and Mobile Menu
                <div class="flex items-center gap-3">
                    // Desktop CTAs
                    <a href="/packages" class="hidden lg:inline-flex px-4 py-2 rounded-none font-bold text-xs uppercase tracking-wide items-center gap-2 bg-gray-800 text-white hover:bg-gray-700 transition-colors">
                        "VIEW PACKAGES"
                    </a>
                    <a href="/handyman-coventry" class="hidden lg:inline-flex px-4 py-2 rounded-none font-bold text-xs uppercase tracking-wide items-center gap-2 bg-gray-800 text-white hover:bg-gray-700 transition-colors">
                        "EXAMPLE"
                    </a>
                    <a href="/contact" class="hidden lg:inline-flex px-4 py-2 rounded-none font-bold text-xs uppercase tracking-wide items-center gap-2 bg-white text-black hover:bg-gray-100 transition-colors">
                        "GET IN TOUCH"
                        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/>
                        </svg>
                    </a>

                    // Mobile/Tablet CTAs - Simple white text links (always visible below lg)
                    <a href="/packages" class="lg:hidden text-[11px] font-bold uppercase tracking-wide transition-colors" style="color: white !important;">
                        "PACKAGES"
                    </a>
                    <a href="/handyman-coventry" class="lg:hidden text-[11px] font-bold uppercase tracking-wide transition-colors" style="color: white !important;">
                        "EXAMPLE"
                    </a>
                    <a href="/contact" class="lg:hidden text-[11px] font-bold uppercase tracking-wide transition-colors" style="color: white !important;">
                        "CONTACT"
                    </a>

                    // Mobile Hamburger Menu (always visible on mobile/tablet)
                    <button
                        class="lg:hidden flex items-center justify-center p-2 ml-2"
                        on:click=move |_| set_mobile_open.update(|v| *v = !*v)
                        aria-label="Toggle menu"
                    >
                        <Show when=move || mobile_open.get() fallback=|| view! {
                            <div class="flex flex-col gap-1.5">
                                <span class="w-7 h-0.5 bg-white"></span>
                                <span class="w-7 h-0.5 bg-white"></span>
                                <span class="w-7 h-0.5 bg-white"></span>
                            </div>
                        }>
                            <svg class="w-6 h-6 text-white" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                            </svg>
                        </Show>
                    </button>
                </div>
            </div>

            // Mobile Navbar with Independent Dropdowns
            <div class={move || format!(
                "lg:hidden transition-all duration-500 ease-out {}",
                if mobile_open.get() { "max-h-[800px] opacity-100 pb-8" } else { "max-h-0 opacity-0 overflow-hidden" }
            )}>
                <div class="px-6 py-8 h-[calc(100vh-80px)] overflow-y-auto">
                    <div class="flex flex-col gap-6">
                        // MAIN Section
                        <div class="border-b border-white/10 pb-6">
                            <button
                                class="w-full flex items-center justify-between text-lg font-bold text-white uppercase tracking-wider mb-4"
                                on:click=move |_| set_main_open.update(|v| *v = !*v)
                            >
                                "Main"
                                <svg class={move || format!("w-5 h-5 transition-transform duration-300 {}", if main_open.get() { "rotate-180" } else { "" })} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                                </svg>
                            </button>
                            <div class={move || format!(
                                "overflow-hidden transition-all duration-300 {}",
                                if main_open.get() { "max-h-[500px] opacity-100" } else { "max-h-0 opacity-0" }
                            )}>
                                <nav class="flex flex-col gap-3 pl-4">
                                    <a href="/" class="text-white/70 hover:text-white py-2 block" on:click=move |_| set_mobile_open.set(false)>"Home"</a>
                                    <a href="/industries" class="text-white/70 hover:text-white py-2 block" on:click=move |_| set_mobile_open.set(false)>"Industries"</a>
                                    <a href="/about" class="text-white/70 hover:text-white py-2 block" on:click=move |_| set_mobile_open.set(false)>"About"</a>
                                </nav>
                            </div>
                        </div>

                        // RESOURCES Section
                        <div class="border-b border-white/10 pb-6">
                            <button
                                class="w-full flex items-center justify-between text-lg font-bold text-white uppercase tracking-wider mb-4"
                                on:click=move |_| set_resources_open.update(|v| *v = !*v)
                            >
                                "Resources"
                                <svg class={move || format!("w-5 h-5 transition-transform duration-300 {}", if resources_open.get() { "rotate-180" } else { "" })} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                                </svg>
                            </button>
                            <div class={move || format!(
                                "overflow-hidden transition-all duration-300 {}",
                                if resources_open.get() { "max-h-[500px] opacity-100" } else { "max-h-0 opacity-0" }
                            )}>
                                <nav class="flex flex-col gap-3 pl-4">
                                    <a href="/blog" class="text-white/70 hover:text-white py-2 block" on:click=move |_| set_mobile_open.set(false)>"Blog"</a>
                                    <a href="/faq" class="text-white/70 hover:text-white py-2 block" on:click=move |_| set_mobile_open.set(false)>"FAQ"</a>
                                    <a href="/service-agreement" class="text-white/70 hover:text-white py-2 block" on:click=move |_| set_mobile_open.set(false)>"Service Agreement"</a>
                                </nav>
                            </div>
                        </div>

                        // EXAMPLE Section
                        <div class="border-b border-white/10 pb-6">
                            <button
                                class="w-full flex items-center justify-between text-lg font-bold text-white uppercase tracking-wider mb-4"
                                on:click=move |_| set_example_open.update(|v| *v = !*v)
                            >
                                "Example"
                                <svg class={move || format!("w-5 h-5 transition-transform duration-300 {}", if example_open.get() { "rotate-180" } else { "" })} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                                </svg>
                            </button>
                            <div class={move || format!(
                                "overflow-hidden transition-all duration-300 {}",
                                if example_open.get() { "max-h-[500px] opacity-100" } else { "max-h-0 opacity-0" }
                            )}>
                                <nav class="flex flex-col gap-3 pl-4">
                                     <a href="/handyman-coventry" class="text-white/70 hover:text-white py-2 block" on:click=move |_| set_mobile_open.set(false)>"The Handyman"</a>
                                </nav>
                            </div>
                        </div>

                        // LEGAL Section
                        <div class="pb-6">
                            <button
                                class="w-full flex items-center justify-between text-lg font-bold text-white uppercase tracking-wider mb-4"
                                on:click=move |_| set_legal_open.update(|v| *v = !*v)
                            >
                                "Legal"
                                <svg class={move || format!("w-5 h-5 transition-transform duration-300 {}", if legal_open.get() { "rotate-180" } else { "" })} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                                </svg>
                            </button>
                            <div class={move || format!(
                                "overflow-hidden transition-all duration-300 {}",
                                if legal_open.get() { "max-h-[500px] opacity-100" } else { "max-h-0 opacity-0" }
                            )}>
                                <nav class="flex flex-col gap-3 pl-4">
                                    <a href="/terms" class="text-white/70 hover:text-white py-2 block" on:click=move |_| set_mobile_open.set(false)>"Terms & Conditions"</a>
                                    <a href="/privacy" class="text-white/70 hover:text-white py-2 block" on:click=move |_| set_mobile_open.set(false)>"Privacy Policy"</a>
                                </nav>
                            </div>
                        </div>
                    </div>
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
