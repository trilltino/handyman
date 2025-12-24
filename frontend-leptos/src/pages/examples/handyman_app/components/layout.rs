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

        <div class="font-sans antialiased text-gray-900 bg-slate-50 min-h-screen flex flex-col selection:bg-yellow-400 selection:text-blue-900">
            // Demo Site Banner
            <div class="bg-gradient-to-r from-purple-600 to-indigo-600 text-white py-2 px-4 text-center text-sm">
                <span class="mr-2">"This is an example site showcasing XFTradesmen capabilities."</span>
                <a href="/" class="underline font-bold hover:text-purple-200 transition">"Back to XFTradesmen"</a>
            </div>

            // Header
            <header class="bg-blue-900/95 backdrop-blur-md text-white py-4 px-6 md:px-12 flex justify-between items-center sticky top-0 z-50 shadow-xl shadow-blue-900/20 border-b border-white/10">
                <a href="/handyman-coventry" class="group flex items-center gap-2">
                     <div class="w-10 h-10 bg-gradient-to-br from-yellow-400 to-yellow-600 rounded-lg flex items-center justify-center text-blue-900 font-black text-xl shadow-lg group-hover:scale-105 transition">"H"</div>
                     <div class="flex flex-col">
                        <span class="text-xl font-bold tracking-tight leading-none group-hover:text-yellow-400 transition">"COVENTRY"</span>
                        <span class="text-xs font-medium text-blue-200 tracking-[0.2em] uppercase">"Handyman Services"</span>
                     </div>
                </a>

                <nav class="hidden md:flex items-center gap-8 text-sm font-bold uppercase tracking-wide">
                    <a href="/handyman-coventry" class="hover:text-yellow-400 transition relative group">
                        "Home"
                        <span class="absolute -bottom-1 left-0 w-0 h-0.5 bg-yellow-400 transition-all group-hover:w-full"></span>
                    </a>
                    <a href="/handyman-coventry/services" class="hover:text-yellow-400 transition relative group">
                        "Services"
                         <span class="absolute -bottom-1 left-0 w-0 h-0.5 bg-yellow-400 transition-all group-hover:w-full"></span>
                    </a>
                    <a href="/handyman-coventry/features" class="hover:text-yellow-400 transition relative group">
                        "Why Us"
                         <span class="absolute -bottom-1 left-0 w-0 h-0.5 bg-yellow-400 transition-all group-hover:w-full"></span>
                    </a>
                    <a href="/handyman-coventry/testimonials" class="hover:text-yellow-400 transition relative group">
                        "Reviews"
                         <span class="absolute -bottom-1 left-0 w-0 h-0.5 bg-yellow-400 transition-all group-hover:w-full"></span>
                    </a>
                    <a href="/handyman-coventry/service-area" class="hover:text-yellow-400 transition relative group">
                        "Service Area"
                         <span class="absolute -bottom-1 left-0 w-0 h-0.5 bg-yellow-400 transition-all group-hover:w-full"></span>
                    </a>
                    <a href="/handyman-coventry/quote" class="hover:text-yellow-400 transition relative group">
                        "Get Quote"
                         <span class="absolute -bottom-1 left-0 w-0 h-0.5 bg-yellow-400 transition-all group-hover:w-full"></span>
                    </a>
                    <a href="/handyman-coventry/booking" class="px-6 py-3 bg-gradient-to-r from-yellow-400 to-yellow-500 text-blue-900 rounded-lg hover:shadow-lg hover:shadow-yellow-500/20 hover:-translate-y-0.5 transition font-bold">
                        "Book Now"
                    </a>
                </nav>
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
                                <a href="mailto:hello@xftradesmen.com" class="hover:text-yellow-400">"hello@coventryhandyman.co.uk"</a>
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
