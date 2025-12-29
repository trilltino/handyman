//! Dedicated layout for the Handyman Example Site.

// use crate::pages::examples::handyman_app::components::CtaButton;
use crate::components::seo::HandymanLocalBusinessSchema;
use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn HandymanLayout() -> impl IntoView {
    view! {
        // SEO structured data for all handyman pages
        <HandymanLocalBusinessSchema />

        <div class="handyman-theme font-sans antialiased text-gray-900 bg-slate-50 min-h-screen flex flex-col selection:bg-yellow-400 selection:text-blue-900">
            // Demo Site Banner
            <div class="bg-gradient-to-r from-purple-600 to-indigo-600 text-white py-2 px-4 text-center text-sm">
                <span class="mr-2">"This is an example site showcasing XFTradesmen capabilities."</span>
                <a href="/" class="underline font-bold hover:text-purple-200 transition">"Back to XFTradesmen"</a>
            </div>


            // Header Container
            <header class="font-sans flex flex-col shadow-xl sticky top-0 z-50">
                // TOP ROW: Logo & Contact Info
                <div class="bg-white text-slate-900 py-4 px-2 border-b border-gray-100">
                    <div class="w-full flex flex-col md:flex-row justify-between items-center gap-0">
                        // Logo
                        <a href="/handyman-coventry" class="flex items-center gap-3 group mr-auto">
                             <div class="w-12 h-12 bg-gradient-to-br from-yellow-400 to-yellow-600 rounded-lg flex items-center justify-center text-blue-900 font-black text-2xl shadow-lg group-hover:scale-105 transition">"H"</div>
                             <div class="flex flex-col">
                                <span class="text-2xl font-black tracking-tight leading-none text-slate-900">"HANDLE"</span>
                                <span class="text-2xl font-black tracking-tight leading-none text-blue-900">"MAN"</span>
                                <span class="text-xs text-red-600 font-bold">"DEBUG MODE"</span>
                             </div>
                        </a>

                        // Contact Widgets (Hidden on small mobile)
                        <div class="hidden md:flex items-center divide-x divide-gray-200 ml-auto bg-white/50 backdrop-blur-sm">
                            // Phone
                            <div class="flex items-center gap-4 px-6">
                                <div class="w-10 h-10 bg-slate-100 rounded-full flex items-center justify-center text-blue-900">
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"/></svg>
                                </div>
                                <div>
                                    <div class="text-xs text-gray-500 font-medium uppercase tracking-wide">"Call Today"</div>
                                    <a href="tel:02476123456" class="text-lg font-bold text-slate-900 hover:text-blue-600 transition">"024 7612 3456"</a>
                                </div>
                            </div>

                            // Email
                            <div class="flex items-center gap-4 px-6">
                                <div class="w-10 h-10 bg-slate-100 rounded-full flex items-center justify-center text-blue-900">
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/></svg>
                                </div>
                                <div>
                                    <div class="text-xs text-gray-500 font-medium uppercase tracking-wide">"Send a Message"</div>
                                    <a href="mailto:hello@coventryhandyman.co.uk" class="text-lg font-bold text-slate-900 hover:text-blue-600 transition">"Click To Email"</a>
                                </div>
                            </div>

                             // Location
                            <div class="flex items-center gap-4 px-6 border-r-0">
                                <div class="w-10 h-10 bg-slate-100 rounded-full flex items-center justify-center text-blue-900">
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"/><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"/></svg>
                                </div>
                                <div>
                                    <div class="text-xs text-gray-500 font-medium uppercase tracking-wide">"Located In"</div>
                                    <div class="text-lg font-bold text-slate-900">"Coventry, England"</div>
                                </div>
                            </div>
                        </div>

                         // Mobile Menu Button (Visible only on mobile)
                        <div class="md:hidden ml-auto">
                            <MobileMenuButton />
                        </div>
                    </div>
                </div>

                // BOTTOM ROW: Navigation Bar
                <div class="bg-blue-900 text-white py-0 shadow-md hidden md:block">
                    <div class="w-full flex justify-end items-center gap-6 px-4">
                        <nav class="flex items-center gap-6">
                             // ABOUT US Dropdown
                             <div class="group relative">
                                <button class="px-5 py-4 text-sm font-bold uppercase tracking-wide text-white hover:text-yellow-400 hover:bg-white/5 transition flex items-center gap-1">
                                    "About Us"
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/></svg>
                                </button>
                                <div class="absolute left-0 top-full w-48 bg-white text-slate-800 shadow-xl rounded-b-lg overflow-hidden invisible group-hover:visible opacity-0 group-hover:opacity-100 transition-all duration-200 z-50 border-t-2 border-yellow-400">
                                    <a href="/handyman-coventry/features" class="block px-6 py-3 text-sm hover:bg-gray-50 hover:text-blue-900 border-b border-gray-100">"Why Choose Us"</a>
                                    <a href="/handyman-coventry/testimonials" class="block px-6 py-3 text-sm hover:bg-gray-50 hover:text-blue-900 border-b border-gray-100">"Reviews"</a>
                                    <a href="/handyman-coventry/faq" class="block px-6 py-3 text-sm hover:bg-gray-50 hover:text-blue-900">"FAQ"</a>
                                </div>
                             </div>

                             // SERVICES Dropdown
                              <div class="group relative">
                                <button class="px-5 py-4 text-sm font-bold uppercase tracking-wide text-white hover:text-yellow-400 hover:bg-white/5 transition flex items-center gap-1">
                                    "Services"
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/></svg>
                                </button>
                                <div class="absolute left-0 top-full w-48 bg-white text-slate-800 shadow-xl rounded-b-lg overflow-hidden invisible group-hover:visible opacity-0 group-hover:opacity-100 transition-all duration-200 z-50 border-t-2 border-yellow-400">
                                    <a href="/handyman-coventry/services" class="block px-6 py-3 text-sm hover:bg-gray-50 hover:text-blue-900 border-b border-gray-100">"All Services"</a>
                                    <a href="/handyman-coventry/service-area" class="block px-6 py-3 text-sm hover:bg-gray-50 hover:text-blue-900">"Service Area"</a>
                                </div>
                             </div>

                            <a href="/handyman-coventry/testimonials" class="px-5 py-4 text-sm font-bold uppercase tracking-wide text-white hover:text-yellow-400 hover:bg-white/5 transition">"Reviews"</a>
                            <a href="/handyman-coventry/contact" class="px-5 py-4 text-sm font-bold uppercase tracking-wide text-white hover:text-yellow-400 hover:bg-white/5 transition">"Contact"</a>
                        </nav>

                        // CTA Button
                        <div class="py-2">
                             <a href="/handyman-coventry/booking" class="px-6 py-2.5 bg-blue-600 text-white border border-blue-500 rounded-full hover:bg-blue-500 hover:shadow-lg hover:-translate-y-0.5 transition font-bold text-sm tracking-wide uppercase flex items-center gap-2">
                                 <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/></svg>
                                "GET YOUR CUSTOM QUOTE"
                            </a>
                        </div>
                    </div>
                </div>
            </header>

            // Main Content Area
            <main class="flex-grow">
                <Outlet/>
            </main>

             // Deep Footer
            <footer class="bg-slate-900 text-slate-300 pt-20 pb-10 px-6 border-t font-light">
                <div class="max-w-7xl mx-auto grid md:grid-cols-4 gap-12 mb-16">
                    <div class="col-span-1 md:col-span-2 space-y-6">
                        <h3 class="text-white text-2xl font-bold mb-4 flex items-center gap-2">
                             <div class="w-8 h-8 bg-yellow-500 rounded flex items-center justify-center text-blue-900 text-lg">"H"</div>
                            "Coventry Handyman"
                        </h3>
                        <p class="text-lg leading-relaxed max-w-md text-slate-400">
                            "Raising the standard of home maintenance. We combine traditional craftsmanship with modern reliability. Fully insured, vetted, and trusted by hundreds of local families."
                        </p>
                        <div class="flex gap-4">
                            // Social placeholders
                            <div class="w-10 h-10 bg-white/5 rounded-full flex items-center justify-center hover:bg-yellow-500 hover:text-blue-900 transition cursor-pointer">"FB"</div>
                            <div class="w-10 h-10 bg-white/5 rounded-full flex items-center justify-center hover:bg-yellow-500 hover:text-blue-900 transition cursor-pointer">"IG"</div>
                            <div class="w-10 h-10 bg-white/5 rounded-full flex items-center justify-center hover:bg-yellow-500 hover:text-blue-900 transition cursor-pointer">"LI"</div>
                        </div>
                    </div>
                    <div>
                        <h4 class="text-white font-bold text-lg mb-6 uppercase tracking-wider">"Explore"</h4>
                        <ul class="space-y-4">
                            <li><a href="/handyman-coventry/services" class="hover:text-yellow-400 transition flex items-center gap-2">"All Services"</a></li>
                            <li><a href="/handyman-coventry/features" class="hover:text-yellow-400 transition flex items-center gap-2">"Our Guarantee"</a></li>
                            <li><a href="/handyman-coventry/testimonials" class="hover:text-yellow-400 transition flex items-center gap-2">"Customer Stories"</a></li>
                            <li><a href="/handyman-coventry/service-area" class="hover:text-yellow-400 transition flex items-center gap-2">"Service Area Map"</a></li>
                            <li><a href="/handyman-coventry/booking" class="hover:text-yellow-400 transition flex items-center gap-2">"Book Online"</a></li>
                        </ul>
                    </div>
                    <div>
                        <h4 class="text-white font-bold text-lg mb-6 uppercase tracking-wider">"Contact"</h4>
                        <ul class="space-y-4">
                             <li class="flex items-start gap-3">
                                <svg class="w-6 h-6 text-yellow-500 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"/><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"/></svg>
                                <span>"Head Office: Coventry, CV1 1AA, United Kingdom"</span>
                            </li>
                            <li class="flex items-center gap-3">
                                <svg class="w-6 h-6 text-yellow-500 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"/></svg>
                                <a href="tel:07833263486" class="hover:text-yellow-400 font-bold text-white text-lg">"+44 7833 263486"</a>
                            </li>
                             <li class="flex items-center gap-3">
                                <svg class="w-6 h-6 text-yellow-500 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/></svg>
                                <a href="mailto:hello@xftradesman.com" class="hover:text-yellow-400">"hello@coventryhandyman.co.uk"</a>
                            </li>
                        </ul>
                    </div>
                </div>
                <div class="text-center text-sm border-t border-white/5 pt-8 flex flex-col md:flex-row justify-between items-center text-slate-500">
                     <span>"© 2026 Coventry Handyman Services. All rights reserved."</span>
                     <div class="flex gap-6 mt-4 md:mt-0">
                        <a href="#" class="hover:text-white transition">"Privacy Policy"</a>
                        <a href="#" class="hover:text-white transition">"Terms of Service"</a>
                     </div>
                </div>
            </footer>
        </div>
    }
}

/// Mobile menu button with hamburger icon and slide-out menu
#[component]
fn MobileMenuButton() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    view! {
        // Hamburger Button (visible on mobile only)
        <button
            class="md:hidden flex flex-col gap-1.5 p-2 z-50"
            on:click=move |_| set_is_open.update(|v| *v = !*v)
            aria-label="Toggle menu"
        >
            <span class={move || format!("w-6 h-0.5 bg-white transition-all duration-300 {}",
                if is_open.get() { "rotate-45 translate-y-2" } else { "" })}></span>
            <span class={move || format!("w-6 h-0.5 bg-white transition-all duration-300 {}",
                if is_open.get() { "opacity-0" } else { "" })}></span>
            <span class={move || format!("w-6 h-0.5 bg-white transition-all duration-300 {}",
                if is_open.get() { "-rotate-45 -translate-y-2" } else { "" })}></span>
        </button>

        // Mobile Menu Overlay
        <div
            class={move || format!("md:hidden fixed inset-0 bg-black/50 z-40 transition-opacity duration-300 {}",
                if is_open.get() { "opacity-100 pointer-events-auto" } else { "opacity-0 pointer-events-none" })}
            on:click=move |_| set_is_open.set(false)
        ></div>

        // Mobile Menu Panel
        <nav
            class={move || format!("md:hidden fixed top-0 right-0 h-full w-72 bg-blue-900 z-50 flex flex-col pt-20 px-6 gap-4 shadow-2xl transition-transform duration-300 {}",
                if is_open.get() { "translate-x-0" } else { "translate-x-full" })}
        >
            <a href="/handyman-coventry" class="text-white font-bold text-lg py-3 border-b border-white/10 hover:text-yellow-400"
               on:click=move |_| set_is_open.set(false)>"Home"</a>

            // About Section
            <div class="py-2 border-b border-white/10">
                 <div class="text-white/50 text-xs font-bold uppercase tracking-widest mb-2">"About"</div>
                 <a href="/handyman-coventry/features" class="block text-white font-bold text-lg py-2 hover:text-yellow-400"
                   on:click=move |_| set_is_open.set(false)>"Why Choose Us"</a>
                 <a href="/handyman-coventry/faq" class="block text-white font-bold text-lg py-2 hover:text-yellow-400"
                   on:click=move |_| set_is_open.set(false)>"FAQ"</a>
                 <a href="/handyman-coventry/testimonials" class="block text-white font-bold text-lg py-2 hover:text-yellow-400"
                   on:click=move |_| set_is_open.set(false)>"Reviews"</a>
            </div>

            // Services Section
             <div class="py-2 border-b border-white/10">
                 <div class="text-white/50 text-xs font-bold uppercase tracking-widest mb-2">"Services"</div>
                 <a href="/handyman-coventry/services" class="block text-white font-bold text-lg py-2 hover:text-yellow-400"
                   on:click=move |_| set_is_open.set(false)>"All Services"</a>
                 <a href="/handyman-coventry/service-area" class="block text-white font-bold text-lg py-2 hover:text-yellow-400"
                   on:click=move |_| set_is_open.set(false)>"Service Area"</a>
            </div>

            <a href="/handyman-coventry/contact" class="text-white font-bold text-lg py-3 border-b border-white/10 hover:text-yellow-400"
               on:click=move |_| set_is_open.set(false)>"Contact"</a>

            <a href="/handyman-coventry/booking"
               class="mt-6 px-6 py-4 bg-gradient-to-r from-yellow-400 to-yellow-500 text-blue-900 rounded-lg text-center font-black uppercase tracking-wide shadow-lg"
               on:click=move |_| set_is_open.set(false)>"GET YOUR CUSTOM QUOTE"</a>
        </nav>
    }
}
