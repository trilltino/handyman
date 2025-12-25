//! Homepage component.
//!
//! Main landing page with hero, features, and CTA sections.

use crate::components::seo::{LocalBusinessSchema, SeoHead};
use crate::components::ui::FeatureList;
use leptos::prelude::*;
use shared::PageMetadata;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <SeoHead metadata=PageMetadata::for_homepage() />
        <LocalBusinessSchema />

        <div class="space-y-0 overflow-x-hidden">
            // HERO SECTION
            <section class="relative bg-void min-h-screen flex items-center pt-20 pb-32 px-4 overflow-hidden">
                // Background Effects
                <div class="absolute inset-0 bg-cyber-grid bg-[length:40px_40px] opacity-20"></div>
                <div class="absolute top-[-20%] left-1/2 -translate-x-1/2 w-[800px] h-[800px] bg-brand/10 blur-[120px] rounded-full pointer-events-none animate-pulse-slow"></div>
                <div class="absolute bottom-[-10%] right-[-10%] w-[600px] h-[600px] bg-void-highlight/30 blur-[100px] rounded-full pointer-events-none"></div>

                <div class="relative max-w-7xl mx-auto text-center z-10 animate-fade-in">
                    <h1 class="text-5xl md:text-7xl lg:text-8xl font-heading font-black tracking-tighter mb-8 leading-[0.9]">
                        "GROW YOUR" <br/>
                        <span class="text-transparent bg-clip-text bg-gradient-to-r from-white via-gray-200 to-gray-500">
                            "BUSINESS IN 2026"
                        </span>
                    </h1>

                    <p class="text-xl md:text-2xl text-gray-400 max-w-2xl mx-auto mb-12 leading-relaxed font-light">
                        "Get more customers with a professional website. <span class='text-brand-light font-semibold'>Start booking jobs online today.</span>"
                    </p>

                    // Promo Video with Play Button Overlay
                    <div class="max-w-4xl mx-auto mb-12 rounded-2xl overflow-hidden shadow-2xl border border-void-highlight/50 relative group">
                        <video
                            id="promo-video"
                            class="w-full h-auto"
                            loop
                            playsinline
                            preload="auto"
                        >
                            <source src="/promo-video.mp4" type="video/mp4"/>
                            "Your browser does not support the video tag."
                        </video>
                        // Play Button Overlay
                        <div
                            id="play-overlay"
                            class="absolute inset-0 flex items-center justify-center bg-black/40 cursor-pointer transition-opacity duration-300"
                            onclick="document.getElementById('promo-video').play(); document.getElementById('play-overlay').style.display='none';"
                        >
                            <div class="w-24 h-24 rounded-full bg-brand/90 flex items-center justify-center shadow-lg hover:bg-brand hover:scale-110 transition-all duration-300">
                                <svg class="w-12 h-12 text-white ml-2" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M8 5v14l11-7z"/>
                                </svg>
                            </div>
                        </div>
                    </div>

                    // Stats / Social Proof
                    // Deep Business Features
                    <div class="mt-20 grid grid-cols-2 md:grid-cols-4 gap-8 border-t border-void-highlight/30 pt-10 opacity-90">
                        <div>
                            <div class="text-2xl md:text-3xl font-bold text-white font-mono mb-1">"AUTO-BOOK"</div>
                            <div class="text-xs text-gray-500 uppercase tracking-widest font-semibold">"24/7 Lead Capture"</div>
                        </div>
                        <div>
                            <div class="text-2xl md:text-3xl font-bold text-white font-mono mb-1">"GEO-RANK"</div>
                            <div class="text-xs text-gray-500 uppercase tracking-widest font-semibold">"Local SEO Core"</div>
                        </div>
                        <div>
                            <div class="text-2xl md:text-3xl font-bold text-white font-mono mb-1">"LEAD-SYNC"</div>
                            <div class="text-xs text-gray-500 uppercase tracking-widest font-semibold">"Instant SMS Alerts"</div>
                        </div>
                        <div>
                            <div class="text-2xl md:text-3xl font-bold text-white font-mono mb-1">"TRUST-FLOW"</div>
                            <div class="text-xs text-gray-500 uppercase tracking-widest font-semibold">"Review Integration"</div>
                        </div>
                    </div>
                </div>
            </section>



            // FEATURES
            <section class="py-24 px-4 bg-void relative overflow-hidden">
                <div class="max-w-7xl mx-auto">
                     <div class="mb-16">
                        <span class="text-brand font-mono text-sm tracking-widest uppercase mb-2 block">"Core Capabilities"</span>
                        <h2 class="text-4xl md:text-5xl font-bold text-white tracking-tighter">"WEBSITE SPECIFICATIONS"</h2>
                    </div>

                    <div class="grid md:grid-cols-2 gap-16 items-center">
                        <div>
                            <p class="text-xl text-gray-400 leading-relaxed mb-8 font-light">
                                "Built on a <span class='text-white font-semibold'>Rust-based architecture</span> for maximum throughput and reliability. This isn't just a website; it's a lead-generation weapon."
                            </p>
                            <FeatureList features=vec![
                                "Hyper-Optimized SEO Core".to_string(),
                                "Mobile-First Responsiveness".to_string(),
                                "Instant Booking Integration".to_string(),
                                "Zero-Latency Performance".to_string(),
                            ] />
                        </div>


                    </div>
                </div>
            </section>



            // FOOTER / AREAS
            <section class="bg-void py-16 px-4 border-t border-void-highlight text-center">
                 <div class="max-w-6xl mx-auto opacity-70 hover:opacity-100 transition-opacity duration-500">
                    <h3 class="text-sm font-semibold text-gray-500 uppercase tracking-widest mb-6">"Serving the following areas"</h3>
                    <div class="flex flex-wrap justify-center gap-x-8 gap-y-4 text-gray-400 text-sm">
                        <span>"Coventry Central"</span>
                        <span>"Earlsdon"</span>
                        <span>"Binley"</span>
                        <span>"Finham"</span>
                        <span>"Kenilworth"</span>
                        <span>"Leamington Spa"</span>
                        <span>"Warwick"</span>
                    </div>
                </div>
            </section>
        </div>
    }
}
