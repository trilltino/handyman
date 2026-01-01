use crate::components::seo::SeoHead;
use leptos::prelude::*;
use shared::PageMetadata;

#[component]
pub fn HandymanFaq() -> impl IntoView {
    view! {
        <SeoHead metadata=PageMetadata {
            title: "Frequently Asked Questions | XFTradesmen Handyman Coventry".to_string(),
            description: "Answers to common questions about our handyman services in Coventry. Pricing, booking, insurance, and service areas explained.".to_string(),
             canonical_url: Some("https://handyman-coventry.co.uk/faq".to_string()),
            og_image: None,
        }/>

        <div class="min-h-screen bg-gradient-to-br from-blue-900 via-slate-900 to-black text-white">
            // Hero Section
            <div class="relative py-24 px-6 text-center overflow-hidden">
                <div class="relative z-10 max-w-4xl mx-auto">
                    <h1 class="text-4xl md:text-5xl font-bold mb-6 font-heading tracking-tight text-white">"FAQs"</h1>
                    <p class="text-blue-100 text-lg md:text-xl max-w-2xl mx-auto">"Expert answers to your renovation and repair questions."</p>
                </div>
            </div>

            // FAQ Accordion Content
            <div class="max-w-3xl mx-auto py-8 px-6 space-y-4 pb-24">

                // Question 1
                <details class="group bg-white/5 hover:bg-white/10 backdrop-blur-sm rounded-xl border border-white/10 overflow-hidden transition-all duration-200">
                    <summary class="flex justify-between items-center cursor-pointer p-6 list-none text-lg font-bold text-white group-open:text-blue-300 transition-colors">
                        "What typical handyman services do you offer?"
                        <div class="w-8 h-8 rounded-full bg-white/10 flex items-center justify-center text-blue-300 group-open:rotate-180 transition-transform duration-300">
                             <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/></svg>
                        </div>
                    </summary>
                    <div class="px-6 pb-6 text-blue-100 text-base leading-relaxed animate-in slide-in-from-top-2 duration-200">
                        "We handle varied tasks: furniture assembly (IKEA, etc.), TV wall mounting, blind/curtain fitting, hanging mirrors/pictures, basic plumbing (tap washers, toilet seats), lock replacements, and general home repairs. If it's a small-to-medium job, we can likely do it!"
                    </div>
                </details>

                 // Question 2
                <details class="group bg-white/5 hover:bg-white/10 backdrop-blur-sm rounded-xl border border-white/10 overflow-hidden transition-all duration-200">
                    <summary class="flex justify-between items-center cursor-pointer p-6 list-none text-lg font-bold text-white group-open:text-blue-300 transition-colors">
                        "How do you charge – by the hour or per job?"
                         <div class="w-8 h-8 rounded-full bg-white/10 flex items-center justify-center text-blue-300 group-open:rotate-180 transition-transform duration-300">
                             <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/></svg>
                        </div>
                    </summary>
                    <div class="px-6 pb-6 text-blue-100 text-base leading-relaxed animate-in slide-in-from-top-2 duration-200">
                        "For small, undefined lists of tasks, we typically charge an hourly rate (minimum 1 hour). For specific, defined jobs like 'Install 3 Curtain Rails' or 'Build 1 Wardrobe', we can provide a fixed price quote so you know exactly what you'll pay."
                    </div>
                </details>

                 // Question 3
                 <details class="group bg-white/5 hover:bg-white/10 backdrop-blur-sm rounded-xl border border-white/10 overflow-hidden transition-all duration-200">
                    <summary class="flex justify-between items-center cursor-pointer p-6 list-none text-lg font-bold text-white group-open:text-blue-300 transition-colors">
                        "Do you cover materials?"
                         <div class="w-8 h-8 rounded-full bg-white/10 flex items-center justify-center text-blue-300 group-open:rotate-180 transition-transform duration-300">
                             <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/></svg>
                        </div>
                    </summary>
                    <div class="px-6 pb-6 text-blue-100 text-base leading-relaxed animate-in slide-in-from-top-2 duration-200">
                         "We carry basic fixings (screws, wall plugs). For job-specific materials (like the paint, the new tap, or the shelf itself), you usually supply them. However, if you need us to pick items up, we can do so (time spent shopping is chargeable)."
                    </div>
                </details>

                 // Question 4
                 <details class="group bg-white/5 hover:bg-white/10 backdrop-blur-sm rounded-xl border border-white/10 overflow-hidden transition-all duration-200">
                    <summary class="flex justify-between items-center cursor-pointer p-6 list-none text-lg font-bold text-white group-open:text-blue-300 transition-colors">
                        "Are your handymen insured?"
                         <div class="w-8 h-8 rounded-full bg-white/10 flex items-center justify-center text-blue-300 group-open:rotate-180 transition-transform duration-300">
                             <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/></svg>
                        </div>
                    </summary>
                    <div class="px-6 pb-6 text-blue-100 text-base leading-relaxed animate-in slide-in-from-top-2 duration-200">
                        "Yes, absolutely. We hold full public liability insurance to protect your property and give you peace of mind while we're working in your home."
                    </div>
                </details>

                 // Question 5
                 <details class="group bg-white/5 hover:bg-white/10 backdrop-blur-sm rounded-xl border border-white/10 overflow-hidden transition-all duration-200">
                    <summary class="flex justify-between items-center cursor-pointer p-6 list-none text-lg font-bold text-white group-open:text-blue-300 transition-colors">
                        "What areas of Coventry do you cover?"
                         <div class="w-8 h-8 rounded-full bg-white/10 flex items-center justify-center text-blue-300 group-open:rotate-180 transition-transform duration-300">
                             <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/></svg>
                        </div>
                    </summary>
                    <div class="px-6 pb-6 text-blue-100 text-base leading-relaxed animate-in slide-in-from-top-2 duration-200">
                        "We cover all of Coventry (CV1-CV6) and immediate surrounding areas like Bedworth, Kenilworth, and Meriden. If you're slightly further out, just ask!"
                    </div>
                </details>
            </div>
        </div>
    }
}
