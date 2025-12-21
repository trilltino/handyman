//! Dedicated layout for the Handyman Example Site.

use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn HandymanLayout() -> impl IntoView {
    view! {
        <div class="font-sans antialiased text-gray-900 bg-white min-h-screen flex flex-col">
            // Header
            <header class="bg-blue-900 text-white py-4 px-6 md:px-12 flex justify-between items-center sticky top-0 z-50 shadow-md">
                <a href="/handyman-coventry" class="text-2xl font-bold tracking-tight hover:text-yellow-400 transition">"COVENTRY HANDYMAN"</a>
                <nav class="hidden md:flex gap-6 text-sm font-semibold uppercase tracking-wide">
                    <a href="/handyman-coventry" class="hover:text-yellow-400 transition">"Home"</a>
                    <a href="/handyman-coventry/services" class="hover:text-yellow-400 transition">"Services"</a>
                    <a href="/handyman-coventry/about" class="hover:text-yellow-400 transition">"About"</a>
                    <a href="/handyman-coventry/contact" class="px-4 py-2 bg-yellow-500 text-blue-900 rounded hover:bg-yellow-400 transition">"Get a Quote"</a>
                </nav>
            </header>

            // Main Content Area
            <main class="flex-grow">
                <Outlet/>
            </main>

             // Footer
            <footer class="bg-gray-900 text-gray-400 py-12 px-6">
                <div class="max-w-6xl mx-auto grid md:grid-cols-4 gap-8 mb-8">
                    <div class="col-span-2">
                        <h3 class="text-white text-lg font-bold mb-4">"Coventry Handyman Services"</h3>
                        <p class="text-sm leading-relaxed">"Your trusted local partner for home repairs, maintenance, and improvements. Fully insured and satisfaction guaranteed."</p>
                    </div>
                    <div>
                        <h4 class="text-white font-bold mb-4">"Quick Links"</h4>
                        <ul class="space-y-2 text-sm">
                            <li><a href="/handyman-coventry" class="hover:text-yellow-400">"Home"</a></li>
                            <li><a href="/handyman-coventry/services" class="hover:text-yellow-400">"Services"</a></li>
                            <li><a href="/handyman-coventry/contact" class="hover:text-yellow-400">"Contact"</a></li>
                        </ul>
                    </div>
                    <div>
                        <h4 class="text-white font-bold mb-4">"Contact"</h4>
                        <ul class="space-y-2 text-sm">
                            <li>"Coventry, UK"</li>
                            <li><a href="tel:07833263486" class="hover:text-yellow-400">"+44 7833 263486"</a></li>
                        </ul>
                    </div>
                </div>
                <div class="text-center text-xs border-t border-gray-800 pt-8">
                     "© 2026 Coventry Handyman Services. Demo Site by XFTradesmen."
                </div>
            </footer>
        </div>
    }
}
