use crate::components::seo::SeoHead;
use leptos::prelude::*;
use leptos_router::components::A;
use shared::PageMetadata;

#[component]
pub fn MountingInstallation() -> impl IntoView {
    view! {
        <SeoHead metadata=PageMetadata {
            title: "TV Mounting & Wall Installation Coventry | XFTradesmen Handyman".to_string(),
            description: "Professional TV wall mounting, shelf installation, and picture hanging in Coventry. Safe, secure, and perfectly level. Trust the experts with your walls.".to_string(),
            canonical_url: Some("https://handyman-coventry.co.uk/services/mounting".to_string()),
             og_image: Some("/images/services/mounting.png".to_string()),
        }/>

        <div class="bg-gray-50 min-h-screen">
             // Hero Section
            <div class="relative bg-gradient-to-r from-blue-900 to-blue-800 text-white py-24 px-6 text-center overflow-hidden">
                <div class="relative z-10 max-w-4xl mx-auto">
                    <h1 class="text-4xl md:text-5xl font-bold mb-6 font-heading tracking-tight">"Mounting & Installation"</h1>
                    <p class="text-blue-100 text-lg md:text-xl max-w-2xl mx-auto">"Secure, level, and clean. We mount TVs, shelves, mirrors, and artwork so they stay up for good."</p>
                </div>
            </div>

            // Main Content
            <div class="max-w-7xl mx-auto py-16 px-6">
                <div class="grid md:grid-cols-2 gap-12 items-center">
                    // Image Column
                    <div class="space-y-6">
                         <img
                            src="/images/services/mounting.png"
                            alt="Handyman Mounting TV on Wall"
                            class="rounded-2xl shadow-xl w-full object-cover h-[500px]"
                        />
                    </div>

                    // Text Column
                    <div class="space-y-8">
                        <div>
                            <h2 class="text-3xl font-bold text-slate-900 mb-4">"How We Work"</h2>
                             <p class="text-slate-600 text-lg leading-relaxed">
                                "Drilling into walls can be daunting. We take that stress away by using the right fixings for your specific wall type (brick, plasterboard, or stud) to ensure a rock-solid hold."
                            </p>
                        </div>

                        <div class="space-y-6">
                            // Step 1
                            <div class="flex gap-4">
                                <div class="w-12 h-12 shrink-0 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center font-bold text-xl">"1"</div>
                                <div>
                                    <h3 class="text-xl font-bold text-slate-900 mb-2">"Wall Assessment"</h3>
                                    <p class="text-slate-600">"We check for hidden pipes and cables using digital detectors and determine the wall composition to choose the correct heavy-duty fixings."</p>
                                </div>
                            </div>

                            // Step 2
                            <div class="flex gap-4">
                                <div class="w-12 h-12 shrink-0 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center font-bold text-xl">"2"</div>
                                <div>
                                    <h3 class="text-xl font-bold text-slate-900 mb-2">"Precision Leveling"</h3>
                                    <p class="text-slate-600">"Using spirit levels and laser guides, we mark the exact positions to ensure your TV or shelf is perfectly straight."</p>
                                </div>
                            </div>

                            // Step 3
                            <div class="flex gap-4">
                                <div class="w-12 h-12 shrink-0 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center font-bold text-xl">"3"</div>
                                <div>
                                    <h3 class="text-xl font-bold text-slate-900 mb-2">"Secure Installation"</h3>
                                    <p class="text-slate-600">"We drill cleanly (using dust catchers), install the bracket/shelf securely, and can even help channel cables for a tidy finish."</p>
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
