//! About page component.
//!
//! Company information and team overview.

use crate::components::seo::SeoHead;
use crate::components::ui::FeatureList;
use leptos::prelude::*;
use shared::PageMetadata;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <SeoHead metadata=PageMetadata {
            title: "About XF Tradesmen - Built by Tradespeople | XF Tradesmen".to_string(),
            description: "Learn our story. We help handymen, plumbers, electricians get found online with professional, SEO-optimized websites.".to_string(),
            canonical_url: Some("https://xftradesmen.com/about".to_string()),
            og_image: None,
        }/>


        <div class="space-y-0 overflow-x-hidden">
             // Hero
             <section class="bg-void relative border-b border-void-highlight text-white py-32 px-4 overflow-hidden">
                <div class="absolute inset-0 bg-cyber-grid bg-[length:40px_40px] opacity-20"></div>
                <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[800px] h-[800px] bg-void-highlight/5 blur-[120px] rounded-full pointer-events-none"></div>

                <div class="max-w-6xl mx-auto text-center relative z-10 animate-fade-in">
                    <span class="text-brand font-mono text-sm tracking-widest uppercase mb-4 block">"System Identity"</span>
                    <h1 class="text-5xl md:text-7xl font-heading font-black mb-6 tracking-tighter">"BUILT BY" <span class="text-brand">"ENGINEERS"</span> <br/> "FOR TRADESPEOPLE"</h1>
                    <p class="text-xl text-gray-400 max-w-2xl mx-auto font-light">
                        "Connecting skilled professionals with customers through high-performance digital infrastructure."
                    </p>
                </div>
            </section>

            <section class="py-24 px-4 bg-void-surface relative">
                <div class="max-w-6xl mx-auto">
                    <div class="grid md:grid-cols-2 gap-16 items-center">
                        <div class="relative group">
                            <div class="absolute -inset-2 bg-gradient-to-r from-brand to-brand-dark rounded-xl opacity-20 group-hover:opacity-40 blur-lg transition duration-500"></div>
                            <div class="card-deep h-80 flex items-center justify-center text-gray-600 border border-void-highlight/50 relative bg-void">
                                <div class="text-center">
                                    <svg class="w-16 h-16 mx-auto mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"/></svg>
                                    <span class="font-mono text-xs uppercase tracking-widest">"Visual Asset // Team_Core"</span>
                                </div>
                            </div>
                        </div>

                        <div>
                            <div class="inline-block px-3 py-1 rounded-full bg-brand/10 border border-brand/20 text-brand text-xs font-mono font-bold mb-6">"OUR MISSION"</div>
                            <h2 class="text-3xl font-bold text-white mb-6 font-heading">"Digital Dominance for Local Trades"</h2>

                            <div class="space-y-6 text-gray-400 leading-relaxed">
                                <p>
                                    "Founded in <span class='text-white font-semibold'>2024</span>, XF Tradesmen was initialized with a singular directive:
                                    to equip independent tradespeople with elite-level digital tools."
                                </p>
                                <p>
                                    "We observed a market failure: skilled professionals relying on subpar, slow, and unoptimized websites.
                                    We engineered a solution. A platform that provides high-speed, SEO-optimised infrastructure 
                                    that ensures you dominate local search results."
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            <section class="py-24 px-4 bg-void border-t border-void-highlight">
                <div class="max-w-6xl mx-auto">
                    <div class="text-center mb-16">
                         <span class="text-brand font-mono text-sm tracking-widest uppercase mb-2 block">"System Capabilities"</span>
                        <h2 class="text-3xl md:text-4xl font-bold text-white tracking-tight font-heading">"WHAT WE PROVIDE"</h2>
                    </div>

                    <div class="max-w-4xl mx-auto">
                        <FeatureList features=vec![
                            "High-Performance Rust Architecture".to_string(),
                            "Automated Local SEO Injection".to_string(),
                            "Secure Payment & Booking Gateways".to_string(),
                            "Zero-Latency Edge Delivery".to_string(),
                            "Mobile-First Responsive Design".to_string(),
                            "24/7 Technical Overwatch".to_string(),
                        ] />
                    </div>
                </div>
            </section>

            <section class="bg-brand-deep py-24 px-4 text-center relative overflow-hidden">
                <div class="absolute inset-0 bg-[url('https://www.transparenttextures.com/patterns/carbon-fibre.png')] opacity-10 mix-blend-overlay"></div>
                <div class="max-w-3xl mx-auto relative z-10">
                    <h2 class="text-3xl font-bold text-white mb-6 font-heading">"JOIN THE NETWORK"</h2>
                    <p class="text-brand-light mb-8 text-lg">"Upgrade your business infrastructure today."</p>
                    <a href="/contact" class="btn-secondary bg-void text-white border-void-highlight hover:bg-void-surface">"Initiate Contact"</a>
                </div>
            </section>
        </div>
    }
}
