//! Handyman Home Page.

use crate::components::seo::{LocalBusinessSchema, SeoHead};
use crate::pages::examples::handyman_app::components::{CtaButton, GlassCard, TrustBadge};
use leptos::prelude::*;
use shared::PageMetadata;

#[component]
pub fn HandymanHome() -> impl IntoView {
    view! {
        <SeoHead metadata=PageMetadata {
            title: "Coventry Handyman Services | Reliable & Local Examples".to_string(),
            description: "Example site for a local handyman business. Shows how XFTradesmen builds high-converting websites.".to_string(),
            canonical_url: Some("https://xftradesmen.com/handyman-coventry".to_string()),
            og_image: None,
        }/>
        <LocalBusinessSchema />

         // Deep Hero
            <section class="relative min-h-[90vh] flex items-center justify-center overflow-hidden bg-blue-950">
                // Background Gradient/Mesh
                <div class="absolute inset-0 bg-gradient-to-br from-blue-900 via-slate-900 to-black z-0"></div>
                <div class="absolute -top-40 -right-40 w-[600px] h-[600px] bg-blue-500/20 rounded-full blur-[100px] animate-pulse"></div>
                <div class="absolute bottom-0 left-0 w-[800px] h-[800px] bg-yellow-500/10 rounded-full blur-[120px]"></div>

                // Content
                <div class="relative z-10 max-w-5xl mx-auto text-center px-6 animate-fade-in-up">
                    <div class="flex justify-center gap-4 mb-8">
                        <TrustBadge icon_path="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" text="Fully Insured"/>
                        <TrustBadge icon_path="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" text="Same Day Service"/>
                    </div>

                    <h1 class="text-5xl md::text-7xl font-black text-white leading-tight mb-8 drop-shadow-2xl">
                        "Your Home." <br/>
                        <span class="text-transparent bg-clip-text bg-gradient-to-r from-yellow-300 to-yellow-500">
                            "Perfected."
                        </span>
                    </h1>

                    <p class="text-xl md:text-2xl text-blue-100 max-w-2xl mx-auto mb-12 font-light leading-relaxed">
                        "Coventry's most trusted property maintenance specialists. We handle the small jobs with the same pride as the big ones."
                    </p>

                    <div class="flex flex-col sm:flex-row gap-6 justify-center">
                        <CtaButton text="Get a Free Quote" href="/handyman-coventry/booking" />
                        <CtaButton text="Explore Services" href="/handyman-coventry/services" variant="glass" />
                    </div>
                </div>

                // Scroll Indicator
                <div class="absolute bottom-8 left-1/2 -translate-x-1/2 text-white/30 animate-bounce">
                    <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3"/></svg>
                </div>
            </section>

             // Stats Bar
            <section class="bg-blue-900 py-12 relative z-20 shadow-2xl">
                <div class="max-w-6xl mx-auto grid grid-cols-2 md:grid-cols-4 gap-8 text-center text-white">
                    <StatItem value="500+" label="Jobs Completed"/>
                    <StatItem value="98%" label="5-Star Reviews"/>
                    <StatItem value="1hr" label="Avg Response"/>
                    <StatItem value="100%" label="Satisfaction"/>
                </div>
            </section>

             // Services Preview
            <section class="py-24 px-6 md:px-12 bg-slate-50">
                <div class="max-w-6xl mx-auto">
                    <div class="text-center mb-16">
                        <span class="text-blue-600 font-bold tracking-widest uppercase text-sm">"What We Do"</span>
                        <h2 class="text-4xl font-black text-slate-900 mt-2">"Expertise You Can Trust"</h2>
                    </div>

                    <div class="grid md:grid-cols-3 gap-8">
                        <ServiceCard
                            title="Home Repairs"
                            desc="Drywall patching, door handle replacements, and general wear-and-tear."
                            icon="🔧"
                            delay="0"
                        />
                        <ServiceCard
                            title="Assembly & Install"
                            desc="Flat pack furniture assembly, TV mounting, and shelf installation."
                            icon="🔨"
                            delay="100"
                        />
                        <ServiceCard
                            title="Plumbing Basics"
                            desc="Leaky taps, toilet repairs, drain unclogging, and shower heads."
                            icon="🚰"
                            delay="200"
                        />
                    </div>

                    <div class="text-center mt-12">
                        <a href="/handyman-coventry/services" class="text-blue-600 font-bold hover:text-blue-800 flex items-center justify-center gap-2 group">
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
    view! {
        <GlassCard class={format!("h-full flex flex-col items-start")}>
            <div class="w-16 h-16 bg-gradient-to-br from-blue-100 to-white rounded-2xl flex items-center justify-center text-3xl shadow-sm mb-6">
                {icon}
            </div>
            <h3 class="text-2xl font-bold text-slate-900 mb-3">{title}</h3>
            <p class="text-slate-600 leading-relaxed mb-6 flex-grow">{desc}</p>
            <span class="text-blue-600 font-bold text-sm uppercase tracking-wide group-hover:translate-x-2 transition-transform inline-flex items-center gap-2">
                "Learn More" <span class="text-lg">"→"</span>
            </span>
        </GlassCard>
    }
}
