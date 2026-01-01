use crate::components::seo::SeoHead;
use leptos::prelude::*;
use leptos_router::components::A;
use shared::PageMetadata;

#[component]
pub fn PlumbingRepairs() -> impl IntoView {
    view! {
        <SeoHead metadata=PageMetadata {
            title: "Plumbing Repairs & Maintenance Coventry | XFTradesmen Handyman".to_string(),
            description: "Reliable plumbing repairs in Coventry. Faucet fixing, leak repairs, sink installation, and blockage clearing. Fast response for your home plumbing needs.".to_string(),
            canonical_url: Some("https://handyman-coventry.co.uk/services/plumbing".to_string()),
            og_image: Some("/images/services/plumbing.png".to_string()),
        }/>

        <div class="bg-gray-50 min-h-screen">
             // Hero Section
            <div class="relative bg-gradient-to-r from-blue-900 to-blue-800 text-white py-24 px-6 text-center overflow-hidden">
                <div class="relative z-10 max-w-4xl mx-auto">
                    <h1 class="text-4xl md:text-5xl font-bold mb-6 font-heading tracking-tight">"Plumbing Repairs"</h1>
                    <p class="text-blue-100 text-lg md:text-xl max-w-2xl mx-auto">"Dripping taps? Leaking pipes? We handle the small plumbing jobs that keep your home running smoothly."</p>
                </div>
            </div>

            // Main Content
            <div class="max-w-7xl mx-auto py-16 px-6">
                <div class="grid md:grid-cols-2 gap-12 items-center">
                    // Image Column
                    <div class="space-y-6">
                         <img
                            src="/images/services/plumbing.png"
                            alt="Handyman Fixing Kitchen Sink"
                            class="rounded-2xl shadow-xl w-full object-cover h-[500px]"
                        />
                    </div>

                    // Text Column
                    <div class="space-y-8">
                        <div>
                            <h2 class="text-3xl font-bold text-slate-900 mb-4">"How We Work"</h2>
                             <p class="text-slate-600 text-lg leading-relaxed">
                                "We specialize in the essential maintenance plumbing jobs that big firms often overlook. Our approach is clean, efficient, and focused on solving your problem permanently."
                            </p>
                        </div>

                        <div class="space-y-6">
                            // Step 1
                            <div class="flex gap-4">
                                <div class="w-12 h-12 shrink-0 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center font-bold text-xl">"1"</div>
                                <div>
                                    <h3 class="text-xl font-bold text-slate-900 mb-2">"Diagnosis"</h3>
                                    <p class="text-slate-600">"We quickly identify the source of the leak or blockage, explaining clearly what's wrong and how we'll fix it."</p>
                                </div>
                            </div>

                            // Step 2
                            <div class="flex gap-4">
                                <div class="w-12 h-12 shrink-0 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center font-bold text-xl">"2"</div>
                                <div>
                                    <h3 class="text-xl font-bold text-slate-900 mb-2">"Efficient Repair"</h3>
                                    <p class="text-slate-600">"We carry common spares and tools to fix many issues on the spot, replacing washers, cartridges, or traps as needed."</p>
                                </div>
                            </div>

                            // Step 3
                            <div class="flex gap-4">
                                <div class="w-12 h-12 shrink-0 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center font-bold text-xl">"3"</div>
                                <div>
                                    <h3 class="text-xl font-bold text-slate-900 mb-2">"Testing & Quality Check"</h3>
                                    <p class="text-slate-600">"We thoroughly test the repair under pressure and ensure there are absolutely no leaks before we leave."</p>
                                </div>
                            </div>
                        </div>

                        <div class="pt-6">
                            <A href="/handyman-coventry/booking" attr:class="inline-flex items-center justify-center px-8 py-4 text-base font-bold text-white transition-all duration-200 bg-blue-600 border border-transparent rounded-full hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-600 shadow-lg hover:shadow-xl hover:-translate-y-1">
                                "Get a Free Quote"
                            </A>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
