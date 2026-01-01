use crate::components::seo::SeoHead;
use leptos::prelude::*;
use leptos_router::components::A;
use shared::PageMetadata;

#[component]
pub fn FurnitureAssembly() -> impl IntoView {
    view! {
        <SeoHead metadata=PageMetadata {
            title: "Furniture Assembly Services Coventry | XFTradesmen Handyman".to_string(),
            description: "Expert flat-pack furniture assembly in Coventry. IKEA, Wayfair, Argos - we build it all perfectly. Save time and stress with our professional service.".to_string(),
            canonical_url: Some("https://handyman-coventry.co.uk/services/furniture-assembly".to_string()),
            og_image: Some("/images/services/furniture_assembly.png".to_string()),
        }/>

        <div class="bg-gray-50 min-h-screen">
            // Hero Section
            <div class="relative bg-gradient-to-r from-blue-900 to-blue-800 text-white py-24 px-6 text-center overflow-hidden">
                <div class="relative z-10 max-w-4xl mx-auto">
                    <h1 class="text-4xl md:text-5xl font-bold mb-6 font-heading tracking-tight">"Furniture Assembly Services"</h1>
                    <p class="text-blue-100 text-lg md:text-xl max-w-2xl mx-auto">"Don't struggle with confusing instructions. We build your furniture quickly, correctly, and stress-free."</p>
                </div>
            </div>

            // Main Content
            <div class="max-w-7xl mx-auto py-16 px-6">
                <div class="grid md:grid-cols-2 gap-12 items-center">
                    // Image Column
                    <div class="space-y-6">
                        <img
                            src="/images/services/furniture_assembly.png"
                            alt="Professional Handyman Assembling Furniture"
                            class="rounded-2xl shadow-xl w-full object-cover h-[500px]"
                        />
                    </div>

                    // Text Column
                    <div class="space-y-8">
                        <div>
                            <h2 class="text-3xl font-bold text-slate-900 mb-4">"How We Work"</h2>
                            <p class="text-slate-600 text-lg leading-relaxed">
                                "Our furniture assembly process is designed to be as effortless for you as possible. Whether it's a single bedside table or a whole office fit-out, we bring the tools and expertise to get the job done right."
                            </p>
                        </div>

                        <div class="space-y-6">
                            // Step 1
                            <div class="flex gap-4">
                                <div class="w-12 h-12 shrink-0 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center font-bold text-xl">"1"</div>
                                <div>
                                    <h3 class="text-xl font-bold text-slate-900 mb-2">"Assessment & Unpacking"</h3>
                                    <p class="text-slate-600">"We carefully unpack all items, check for damage, and organize parts to ensure a smooth build."</p>
                                </div>
                            </div>

                            // Step 2
                            <div class="flex gap-4">
                                <div class="w-12 h-12 shrink-0 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center font-bold text-xl">"2"</div>
                                <div>
                                    <h3 class="text-xl font-bold text-slate-900 mb-2">"Professional Assembly"</h3>
                                    <p class="text-slate-600">"Using professional tools, we assemble your furniture sturdily and safely, following manufacturer guidelines precisely."</p>
                                </div>
                            </div>

                            // Step 3
                            <div class="flex gap-4">
                                <div class="w-12 h-12 shrink-0 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center font-bold text-xl">"3"</div>
                                <div>
                                    <h3 class="text-xl font-bold text-slate-900 mb-2">"Positioning & Cleanup"</h3>
                                    <p class="text-slate-600">"We place the furniture exactly where you want it (including wall-fixing if needed) and clear away all packaging."</p>
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
