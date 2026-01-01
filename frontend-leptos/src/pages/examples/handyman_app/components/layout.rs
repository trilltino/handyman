//! Dedicated layout for the Handyman Example Site.

// use crate::pages::examples::handyman_app::components::CtaButton;
use crate::components::seo::HandymanLocalBusinessSchema;
use leptos::prelude::*;
use leptos_router::components::{Outlet, A};

#[component]
pub fn HandymanLayout() -> impl IntoView {
    view! {
        // SEO structured data for all handyman pages
        <HandymanLocalBusinessSchema />

        <div class="handyman-theme font-sans antialiased text-gray-900 bg-slate-50 min-h-screen flex flex-col selection:bg-yellow-400 selection:text-blue-900">
            // Demo Site Banner
            <div class="bg-gradient-to-r from-purple-600 to-indigo-600 text-white py-2 px-4 text-center text-sm">
                <span class="mr-2">"This is an example site showcasing XFTradesmen capabilities."</span>
                <A href="/" {..} class="underline font-bold hover:text-purple-200 transition">"Back to XFTradesmen"</A>
            </div>


            // Header Container
            <header class="font-sans flex flex-col shadow-xl sticky top-0 z-50">
                // TOP ROW: Logo & Contact Info
                <div class="bg-white text-slate-900 py-4 px-2 border-b border-gray-100">
                    <div class="w-full flex flex-col md:flex-row justify-between items-center gap-0">
                        // Logo
                        <A href="/handyman-coventry" {..} class="flex items-center gap-3 group mr-auto">
                             <div class="w-12 h-12 bg-gradient-to-br from-yellow-400 to-yellow-600 rounded-lg flex items-center justify-center text-blue-900 font-black text-2xl shadow-lg group-hover:scale-105 transition">"H"</div>
                             <div class="flex flex-col">
                                <span class="text-2xl font-black tracking-tight leading-none text-slate-900">"HANDLE"</span>
                                <span class="text-2xl font-black tracking-tight leading-none text-blue-900">"MAN"</span>
                             </div>
                        </A>

                        // Contact Widgets (Hidden on small mobile)
                        <div class="hidden md:flex items-center divide-x divide-gray-200 ml-auto bg-white/50 backdrop-blur-sm">
                            // Phone
                            <div class="flex items-center gap-4 px-6">
                                <div class="w-10 h-10 bg-slate-100 rounded-full flex items-center justify-center text-blue-900">
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"/></svg>
                                </div>
                                <div>
                                    <div class="text-xs text-gray-500 font-medium uppercase tracking-wide">"Call Today"</div>
                                    <A href="tel:02476123456" {..} class="text-lg font-bold text-slate-900 hover:text-blue-600 transition">"024 7612 3456"</A>
                                </div>
                            </div>

                            // Email
                            <div class="flex items-center gap-4 px-6">
                                <div class="w-10 h-10 bg-slate-100 rounded-full flex items-center justify-center text-blue-900">
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/></svg>
                                </div>
                                <div>
                                    <div class="text-xs text-gray-500 font-medium uppercase tracking-wide">"Send a Message"</div>
                                    <A href="mailto:hello@coventryhandyman.co.uk" {..} class="text-lg font-bold text-slate-900 hover:text-blue-600 transition">"Click To Email"</A>
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
                    <div class="w-full flex justify-end items-center gap-10 px-6">
                        <nav class="flex items-center gap-8">
                             // ABOUT US Dropdown
                             <div class="group relative">
                                <button class="px-5 py-4 text-sm font-bold uppercase tracking-wide text-white hover:text-yellow-400 hover:bg-white/5 transition flex items-center gap-1">
                                    "About Us"
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/></svg>
                                </button>
                                <div class="absolute left-0 top-full w-48 bg-white text-slate-800 shadow-xl rounded-b-lg overflow-hidden invisible group-hover:visible opacity-0 group-hover:opacity-100 transition-all duration-200 z-50 border-t-2 border-yellow-400">
                                    <A href="/handyman-coventry/features" {..} class="block px-6 py-3 text-sm hover:bg-gray-50 hover:text-blue-900 border-b border-gray-100">"Why Choose Us"</A>
                                    <A href="/handyman-coventry/faq" {..} class="block px-6 py-3 text-sm hover:bg-gray-50 hover:text-blue-900">"FAQ"</A>
                                </div>
                             </div>

                             // SERVICES Dropdown
                              <div class="group relative">
                                <button class="px-5 py-4 text-sm font-bold uppercase tracking-wide text-white hover:text-yellow-400 hover:bg-white/5 transition flex items-center gap-1">
                                    "Services"
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/></svg>
                                </button>
                                <div class="absolute left-0 top-full w-64 bg-white text-slate-800 shadow-xl rounded-b-lg overflow-hidden invisible group-hover:visible opacity-0 group-hover:opacity-100 transition-all duration-200 z-50 border-t-2 border-yellow-400">
                                    <A href="/handyman-coventry/services/furniture-assembly" {..} class="block px-6 py-3 text-sm hover:bg-gray-50 hover:text-blue-900 border-b border-gray-100">"Furniture Assembly"</A>
                                    <A href="/handyman-coventry/services/plumbing" {..} class="block px-6 py-3 text-sm hover:bg-gray-50 hover:text-blue-900 border-b border-gray-100">"Plumbing Repairs"</A>
                                    <A href="/handyman-coventry/services/mounting" {..} class="block px-6 py-3 text-sm hover:bg-gray-50 hover:text-blue-900 border-b border-gray-100">"Mounting & Installation"</A>
                                    <A href="/handyman-coventry/services" {..} class="block px-6 py-3 text-sm hover:bg-gray-50 hover:text-blue-900 bg-gray-50 font-bold border-t border-gray-100">"View All Services"</A>
                                </div>
                             </div>

                            <A href="/handyman-coventry/testimonials" {..} class="px-5 py-4 text-sm font-bold uppercase tracking-wide text-white hover:text-yellow-400 hover:bg-white/5 transition">"Reviews"</A>
                            <A href="/handyman-coventry/contact" {..} class="px-5 py-4 text-sm font-bold uppercase tracking-wide text-white hover:text-yellow-400 hover:bg-white/5 transition">"Contact"</A>
                        </nav>

                        // CTA Button
                        <div class="py-2">
                             <A href="/handyman-coventry/booking" {..} class="px-6 py-2.5 bg-blue-600 text-white border border-blue-500 rounded-full hover:bg-blue-500 hover:shadow-lg hover:-translate-y-0.5 transition font-bold text-sm tracking-wide uppercase flex items-center gap-2">
                                 <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/></svg>
                                "GET YOUR CUSTOM QUOTE"
                            </A>
                        </div>
                    </div>
                </div>
            </header>

            // Main Content Area
            <main class="flex-grow">
                <Outlet/>
            </main>

            // Deep Footer
            <footer class="bg-[#0a0f1c] text-slate-300 font-sans">
                // CTA Banner
                <div class="bg-gradient-to-r from-blue-950 to-[#0a0f1c] text-white py-16 px-6 text-center border-b border-white/5 relative overflow-hidden">
                     <div class="relative z-10 max-w-4xl mx-auto">
                        <h2 class="text-3xl md:text-4xl font-bold mb-6 font-heading leading-tight">"Stop Struggling With Home Repairs - Get XFTradesmen's Expert Help Today!"</h2>
                         <A href="/handyman-coventry/booking" {..} class="inline-block px-8 py-4 bg-blue-600 hover:bg-blue-500 text-white font-bold rounded-lg transition shadow-lg hover:shadow-blue-900/50 uppercase tracking-wide">
                            "Get Your Custom Quote"
                        </A>
                    </div>
                </div>

                // Main Footer Content
                <div class="max-w-7xl mx-auto pt-16 pb-10 px-6">
                    <div class="grid md:grid-cols-12 gap-12 mb-16">
                        // Column 1: Brand (4 cols)
                        <div class="md:col-span-4 space-y-6">
                             <div class="flex items-center gap-3">
                                <div class="w-10 h-10 bg-gradient-to-br from-yellow-400 to-yellow-600 rounded flex items-center justify-center text-blue-900 font-black text-xl">"H"</div>
                                <div class="flex flex-col">
                                    <span class="text-xl font-black tracking-tight leading-none text-white">"HANDLE"</span>
                                    <span class="text-xl font-black tracking-tight leading-none text-blue-500">"MAN"</span>
                                </div>
                             </div>
                            <p class="text-base leading-relaxed text-slate-400">
                                "XFTradesmen provides patented handles and lock systems that transform sliding doors into safer, easier, and more enjoyable features of your home."
                            </p>
                             <div class="flex gap-4 pt-4">
                                <A href="#" {..} class="w-10 h-10 bg-white/5 rounded-full flex items-center justify-center hover:bg-white/10 transition" aria-label="Facebook">
                                    <img src="/images/icons/facebook.webp" class="w-5 h-5 object-contain brightness-0 invert opacity-90" alt="Facebook" />
                                </A>
                                <A href="#" {..} class="w-10 h-10 bg-white/5 rounded-full flex items-center justify-center hover:bg-white/10 transition" aria-label="Instagram">
                                     <img src="/images/icons/instagram.png" class="w-5 h-5 object-contain opacity-90" alt="Instagram" />
                                </A>
                                <A href="#" {..} class="w-10 h-10 bg-white/5 rounded-full flex items-center justify-center hover:bg-white/10 transition" aria-label="LinkedIn">
                                    <img src="/images/icons/linkedin.webp" class="w-5 h-5 object-contain brightness-0 invert opacity-90" alt="LinkedIn" />
                                </A>
                            </div>
                        </div>

                        // Column 2: Phone & Hours (4 cols)
                        <div class="md:col-span-4 pl-0 md:pl-12 space-y-6">
                            <div>
                                <h4 class="text-xs font-bold text-slate-500 uppercase tracking-widest mb-1">"Phone Number"</h4>
                                <A href="tel:02476123456" {..} class="text-2xl font-bold text-white hover:text-blue-500 transition">"(024) 7612-3456"</A>
                            </div>
                             <div>
                                <h4 class="text-xs font-bold text-slate-500 uppercase tracking-widest mb-1">"Office Hours"</h4>
                                <p class="text-lg text-white font-bold">"Mon - Sat"</p>
                                <p class="text-lg text-white">"8:00 am - 6:00 pm"</p>
                            </div>
                        </div>

                        // Column 3: Email & Address (4 cols)
                        <div class="md:col-span-4 space-y-6">
                             <div>
                                <h4 class="text-xs font-bold text-slate-500 uppercase tracking-widest mb-1">"Email Address"</h4>
                                <A href="mailto:info@handyman-coventry.co.uk" {..} class="text-2xl font-bold text-white hover:text-blue-500 transition">"info@handyman-coventry.co.uk"</A>
                            </div>
                             <div>
                                <h4 class="text-xs font-bold text-slate-500 uppercase tracking-widest mb-1">"Office Address"</h4>
                                <p class="text-lg text-white font-bold">"Coventry, CV1 1AA"</p>
                                <p class="text-lg text-white">"United Kingdom"</p>
                            </div>


                        </div>
                    </div>

                    // Bottom Links
                    <div class="border-t border-white/5 pt-8">
                         <div class="flex flex-wrap justify-center gap-x-8 gap-y-4 text-sm font-medium text-slate-400">
                            <A href="#" {..} class="hover:text-white transition">"Home"</A>
                            <A href="#" {..} class="hover:text-white transition">"About"</A>
                            <A href="#" {..} class="hover:text-white transition">"Reviews"</A>
                            <A href="/handyman-coventry/faq" {..} class="hover:text-white transition">"FAQ"</A>
                            <A href="/handyman-coventry/contact" {..} class="hover:text-white transition">"Contact"</A>
                        </div>
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
            <A href="/handyman-coventry" {..} class="text-white font-bold text-lg py-3 border-b border-white/10 hover:text-yellow-400"
               on:click=move |_| set_is_open.set(false)>"Home"</A>

            // About Section
            <div class="py-2 border-b border-white/10">
                 <div class="text-white/50 text-xs font-bold uppercase tracking-widest mb-2">"About"</div>
                 <A href="/handyman-coventry/features" {..} class="block text-white font-bold text-lg py-2 hover:text-yellow-400"
                   on:click=move |_| set_is_open.set(false)>"Why Choose Us"</A>
                 <A href="/handyman-coventry/faq" {..} class="block text-white font-bold text-lg py-2 hover:text-yellow-400"
                   on:click=move |_| set_is_open.set(false)>"FAQ"</A>
                 <A href="/handyman-coventry/testimonials" {..} class="block text-white font-bold text-lg py-2 hover:text-yellow-400"
                   on:click=move |_| set_is_open.set(false)>"Reviews"</A>
            </div>

            // Services Section
             <div class="py-2 border-b border-white/10">
                 <div class="text-white/50 text-xs font-bold uppercase tracking-widest mb-2">"Services"</div>
                 <A href="/handyman-coventry/services" {..} class="block text-white font-bold text-lg py-2 hover:text-yellow-400"
                   on:click=move |_| set_is_open.set(false)>"All Services"</A>
                 <A href="/handyman-coventry/service-area" {..} class="block text-white font-bold text-lg py-2 hover:text-yellow-400"
                   on:click=move |_| set_is_open.set(false)>"Service Area"</A>
            </div>

            <A href="/handyman-coventry/contact" {..} class="text-white font-bold text-lg py-3 border-b border-white/10 hover:text-yellow-400"
               on:click=move |_| set_is_open.set(false)>"Contact"</A>

            <A href="/handyman-coventry/booking" {..}
               class="mt-6 px-6 py-4 bg-gradient-to-r from-yellow-400 to-yellow-500 text-blue-900 rounded-lg text-center font-black uppercase tracking-wide shadow-lg"
               on:click=move |_| set_is_open.set(false)>"GET YOUR CUSTOM QUOTE"</A>
        </nav>
    }
}
