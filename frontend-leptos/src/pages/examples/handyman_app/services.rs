//! Handyman Services Page.

use leptos::prelude::*;

#[component]
pub fn HandymanServices() -> impl IntoView {
    view! {
        <div class="bg-gray-50 min-h-screen">
            <section class="bg-blue-900 text-white py-16 px-6 text-center">
                <h1 class="text-4xl font-bold mb-4">"Our Services"</h1>
                <p class="text-blue-200">"Comprehensive solutions for your home maintenance needs."</p>
            </section>

            <section class="py-16 px-6 md:px-12 max-w-6xl mx-auto">
                <div class="grid md:grid-cols-2 gap-8">
                    // Service Item
                    <div class="bg-white rounded-xl overflow-hidden shadow-sm hover:shadow-md transition flex flex-col md:flex-row">
                        <div class="md:w-1/3 bg-gray-200 h-48 md:h-auto relative">
                             // Placeholder for map/image
                             <div class="absolute inset-0 flex items-center justify-center text-gray-400 font-bold uppercase tracking-widest">"Plumbing"</div>
                        </div>
                        <div class="p-8 md:w-2/3">
                            <h3 class="text-2xl font-bold text-blue-900 mb-2">"Plumbing & Water"</h3>
                             <p class="text-gray-600 mb-4">"From a dripping tap to a running toilet, we fix common plumbing annoyances quickly."</p>
                            <a href="/handyman-coventry/services/plumbing" class="text-yellow-600 font-bold hover:text-yellow-500 uppercase text-sm tracking-wide">
                                "View Details & 3D Map →"
                            </a>
                        </div>
                    </div>

                    // Service Item
                    <div class="bg-white rounded-xl overflow-hidden shadow-sm hover:shadow-md transition flex flex-col md:flex-row">
                         <div class="md:w-1/3 bg-gray-200 h-48 md:h-auto relative">
                             <div class="absolute inset-0 flex items-center justify-center text-gray-400 font-bold uppercase tracking-widest">"Electrical"</div>
                        </div>
                        <div class="p-8 md:w-2/3">
                            <h3 class="text-2xl font-bold text-blue-900 mb-2">"Electrical Fixtures"</h3>
                             <p class="text-gray-600 mb-4">"Safe replacement of sockets, switches, and light fixtures. Not for major rewiring."</p>
                            <a href="#" class="text-yellow-600 font-bold hover:text-yellow-500 uppercase text-sm tracking-wide">
                                "View Details →"
                            </a>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}
