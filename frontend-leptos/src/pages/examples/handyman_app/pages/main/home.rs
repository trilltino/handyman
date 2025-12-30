//! Handyman Home Page.

use crate::components::seo::{LocalBusinessSchema, SeoHead};
use crate::pages::examples::handyman_app::components::{CtaButton, GlassCard};
use leptos::prelude::*;
use shared::PageMetadata;

#[component]
pub fn HandymanHome() -> impl IntoView {
    view! {
        <SeoHead metadata=PageMetadata {
            title: "Coventry Handyman Services | Reliable & Local Examples".to_string(),
            description: "Example site for a local handyman business. Shows how XFTradesmen builds high-converting websites.".to_string(),
            canonical_url: Some("https://xftradesman.com/handyman-coventry".to_string()),
            og_image: None,
        }/>
        <LocalBusinessSchema />

         // Deep Hero
            <section class="relative min-h-[90vh] flex items-center justify-center overflow-hidden">
                // Background Gradient/Mesh - Red & Black
                <div class="absolute inset-0 bg-gradient-to-br from-red-900 via-red-950 to-black z-0"></div>
                <div class="absolute -top-40 -right-40 w-[600px] h-[600px] bg-red-500/20 rounded-full blur-[100px] animate-pulse"></div>
                <div class="absolute bottom-0 left-0 w-[800px] h-[800px] bg-orange-500/10 rounded-full blur-[120px]"></div>

                // Content
                <div class="relative z-10 max-w-6xl mx-auto text-center px-6 animate-fade-in-up">
                    <div class="flex justify-center gap-4 mb-8">
                        <span class="text-xl md:text-2xl font-light text-white tracking-widest uppercase">"Easy. Secure. Smooth."</span>
                    </div>

                    <h1 class="text-4xl md:text-5xl font-black text-white leading-tight mb-8 drop-shadow-2xl" style="color: white !important;">
                        "Meet the renown local handyman everyone is Talking about!"
                    </h1>

                    <p class="text-xl md:text-2xl text-white max-w-4xl mx-auto mb-12 font-light leading-relaxed" style="color: white !important;">
                        "Rick makes fixing things around your home simple again with 10 years of experience in the local area enjoy repairs at affordable rates with attentive workmanship"
                    </p>

                    <div class="flex flex-col sm:flex-row gap-6 justify-center">
                        <CtaButton text="Get a Free Quote" href="/handyman-coventry/booking" />

                    </div>
                </div>

                // Scroll Indicator
                <div class="absolute bottom-8 left-1/2 -translate-x-1/2 text-white/30 animate-bounce">
                    <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3"/></svg>
                </div>
            </section>



             // Services Preview - Seamless continuation of gradient
            <section class="py-24 px-6 md:px-12 bg-gradient-to-b from-black via-red-950/50 to-black">
                <div class="max-w-6xl mx-auto">
                    <div class="text-center mb-16">
                        <span class="text-red-400 font-bold tracking-widest uppercase text-sm">"What We Do"</span>
                        <h2 class="text-4xl font-black text-white mt-2">"Expertise You Can Trust"</h2>
                    </div>

                    <div class="grid md:grid-cols-3 gap-8">
                        <ServiceCard
                            title="Home Repairs"
                            desc="Drywall patching, door handle replacements, and general wear-and-tear."
                            icon="Repair"
                            delay="0"
                        />
                        <ServiceCard
                            title="Assembly & Install"
                            desc="Flat pack furniture assembly, TV mounting, and shelf installation."
                            icon="Build"
                            delay="100"
                        />
                        <ServiceCard
                            title="Plumbing Basics"
                            desc="Leaky taps, toilet repairs, drain unclogging, and shower heads."
                            icon="Water"
                            delay="200"
                        />
                    </div>

                    <div class="text-center mt-12">
                        <a href="/handyman-coventry/services" class="text-red-400 font-bold hover:text-red-300 flex items-center justify-center gap-2 group">
                            "View Full Service and Areas Map"
                            <svg class="w-4 h-4 group-hover:translate-x-1 transition" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/></svg>
                        </a>
                    </div>
                </div>
            </section>
    }
}

#[component]
fn StatItem(value: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <div class="space-y-2">
            <div class="text-4xl font-black text-yellow-400">{value}</div>
            <div class="text-sm font-bold uppercase tracking-wider text-blue-200">{label}</div>
        </div>
    }
}

#[component]
fn ServiceCard(
    title: &'static str,
    desc: &'static str,
    icon: &'static str,
    delay: &'static str,
) -> impl IntoView {
    let _ = delay;
    view! {
        <GlassCard class={format!("h-full flex flex-col items-start")}>
            <div class="w-16 h-16 bg-gradient-to-br from-red-500/30 to-red-900/50 rounded-2xl flex items-center justify-center text-3xl shadow-sm mb-6 text-white">
                {icon}
            </div>
            <h3 class="text-2xl font-bold text-white mb-3">{title}</h3>
            <p class="text-gray-300 leading-relaxed mb-6 flex-grow">{desc}</p>
            <span class="text-red-400 font-bold text-sm uppercase tracking-wide group-hover:translate-x-2 transition-transform inline-flex items-center gap-2">
                "Learn More" <span class="text-lg">"→"</span>
            </span>
        </GlassCard>
    }
}
