//! Handyman Services Page.

use crate::pages::examples::handyman_app::components::{CtaButton, GlassCard, SectionTitle};
use leptos::prelude::*;

#[component]
pub fn HandymanServices() -> impl IntoView {
    view! {
        <div class="bg-slate-50 min-h-screen">
            <section class="bg-blue-900 text-white py-20 px-6 relative overflow-hidden">
                 <div class="absolute inset-0 bg-[url('https://images.unsplash.com/photo-1581094794329-cd2a1fb4d942?q=80&w=2000&auto=format&fit=crop')] bg-cover bg-center opacity-10"></div>
                 <div class="relative z-10 text-center max-w-4xl mx-auto">
                    <SectionTitle subtitle="What We Do" title="Comprehensive Home Services" align="center" dark=true />
                    <p class="text-blue-100 text-xl">"Professional solutions for your home maintenance needs."</p>
                 </div>
            </section>

            <section class="py-20 px-6 md:px-12 max-w-6xl mx-auto">
                <div class="grid gap-12">
                    // Service Item 1
                    <GlassCard class="flex flex-col md:flex-row gap-8 !p-0 overflow-hidden">
                        <div class="md:w-1/3 bg-slate-200 min-h-[300px] relative group overflow-hidden">
                             // Placeholder for map/image
                             <div class="absolute inset-0 bg-gradient-to-t from-black/50 to-transparent z-10"></div>
                             <div class="absolute inset-0 flex items-center justify-center text-slate-400 font-bold uppercase tracking-widest bg-slate-100">"Plumbing Image"</div>
                             <div class="absolute bottom-6 left-6 z-20 text-white font-bold text-xl transform translate-y-2 opacity-0 group-hover:translate-y-0 group-hover:opacity-100 transition duration-500">
                                "See Coverage Area"
                             </div>
                        </div>
                        <div class="p-8 md:p-12 md:w-2/3 flex flex-col justify-center">
                            <div class="flex items-center gap-3 mb-4">
                                <span class="bg-blue-100 text-blue-900 p-2 rounded-lg text-2xl">"🚰"</span>
                                <h3 class="text-3xl font-bold text-blue-900">"Plumbing & Water"</h3>
                            </div>
                            <p class="text-slate-600 text-lg leading-relaxed mb-8">
                                "From a dripping tap to a running toilet, we fix common plumbing annoyances quickly. We handle repairs, replacements, and new installations for sinks, toilets, and showers."
                            </p>

                            <div class="flex flex-wrap gap-4 mb-8">
                                <ServiceTag text="Leaky Taps"/>
                                <ServiceTag text="Toilet Repair"/>
                                <ServiceTag text="Blocked Drains"/>
                                <ServiceTag text="Shower Install"/>
                            </div>

                            <div>
                                <CtaButton text="View Details & 3D Map" href="/handyman-coventry/services/plumbing" />
                            </div>
                        </div>
                    </GlassCard>

                    // Service Item 2
                     <GlassCard class="flex flex-col md:flex-row gap-8 !p-0 overflow-hidden">
                        <div class="md:w-1/3 bg-slate-200 min-h-[300px] relative group overflow-hidden">
                             <div class="absolute inset-0 flex items-center justify-center text-slate-400 font-bold uppercase tracking-widest bg-slate-100">"Electrical Image"</div>
                        </div>
                        <div class="p-8 md:p-12 md:w-2/3 flex flex-col justify-center">
                            <div class="flex items-center gap-3 mb-4">
                                <span class="bg-yellow-100 text-yellow-700 p-2 rounded-lg text-2xl">"💡"</span>
                                <h3 class="text-3xl font-bold text-blue-900">"Electrical Fixtures"</h3>
                            </div>
                            <p class="text-slate-600 text-lg leading-relaxed mb-8">
                                "Safe replacement of sockets, switches, and light fixtures. We can upgrade your home to modern LED lighting or replace broken outlets."
                            </p>

                            <div class="flex flex-wrap gap-4 mb-8">
                                <ServiceTag text="Light Fittings"/>
                                <ServiceTag text="Socket Replacement"/>
                                <ServiceTag text="Dimmer Switches"/>
                                <ServiceTag text="Bulb Replacement"/>
                            </div>

                            <div>
                                <CtaButton text="View Details" href="#" variant="outline" /> // Placeholder link
                            </div>
                        </div>
                    </GlassCard>
                </div>
            </section>
        </div>
    }
}

#[component]
fn ServiceTag(text: &'static str) -> impl IntoView {
    view! {
        <span class="px-3 py-1 bg-slate-100 text-slate-600 rounded-full text-sm font-bold border border-slate-200">
            {text}
        </span>
    }
}
