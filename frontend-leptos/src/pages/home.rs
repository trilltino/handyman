//! Homepage component.
//!
//! Main landing page with hero, features, and CTA sections.

use leptos::prelude::*;
use crate::components::ui::{FeatureList, Button, ButtonVariant};
use crate::components::seo::SeoHead;
use shared::PageMetadata;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <SeoHead metadata=PageMetadata::for_homepage() />
        
        <div class="space-y-0 overflow-x-hidden">
            // HERO SECTION
            <section class="relative bg-void min-h-screen flex items-center pt-20 pb-32 px-4 overflow-hidden">
                // Background Effects
                <div class="absolute inset-0 bg-cyber-grid bg-[length:40px_40px] opacity-20"></div>
                <div class="absolute top-[-20%] left-1/2 -translate-x-1/2 w-[800px] h-[800px] bg-brand/10 blur-[120px] rounded-full pointer-events-none animate-pulse-slow"></div>
                <div class="absolute bottom-[-10%] right-[-10%] w-[600px] h-[600px] bg-void-highlight/30 blur-[100px] rounded-full pointer-events-none"></div>
                
                <div class="relative max-w-7xl mx-auto text-center z-10 animate-fade-in">
                    <div class="inline-flex items-center gap-2 mb-8 px-4 py-1.5 rounded-full bg-void-surface/80 border border-void-highlight/50 backdrop-blur-md">
                        <span class="w-2 h-2 rounded-full bg-brand-glow animate-pulse"></span>
                        <span class="text-gray-300 text-sm font-mono tracking-wider uppercase">"System Online v2.0"</span>
                    </div>
                
                    <h1 class="text-5xl md:text-7xl lg:text-8xl font-heading font-black tracking-tighter mb-8 leading-[0.9]">
                        "DOMINATE" <br/>
                        <span class="text-transparent bg-clip-text bg-gradient-to-r from-white via-gray-200 to-gray-500">
                            "THE MARKET"
                        </span>
                    </h1>
                    
                    <p class="text-xl md:text-2xl text-gray-400 max-w-2xl mx-auto mb-12 leading-relaxed font-light">
                        "High-performance digital infrastructure for tradespeople. <span class='text-brand-light font-semibold'>Zero compromise.</span>"
                    </p>
                    
                    <div class="flex flex-col sm:flex-row gap-6 justify-center items-center animate-fade-in-delay-2">
                        <Button 
                            label="INITIALIZE SEQUENCE" 
                            href="/pricing" 
                            variant=ButtonVariant::Primary 
                        />
                        <Button 
                            label="SYSTEM SPECS" 
                            href="/about" 
                            variant=ButtonVariant::Secondary 
                        />
                    </div>

                    // Stats / Social Proof
                    <div class="mt-20 grid grid-cols-2 md:grid-cols-4 gap-8 border-t border-void-highlight/30 pt-10 opacity-70">
                        <div>
                            <div class="text-3xl font-bold text-white font-mono">"0.2s"</div>
                            <div class="text-xs text-gray-500 uppercase tracking-widest">"Load Time"</div>
                        </div>
                        <div>
                            <div class="text-3xl font-bold text-white font-mono">"100%"</div>
                            <div class="text-xs text-gray-500 uppercase tracking-widest">"Uptime"</div>
                        </div>
                        <div>
                            <div class="text-3xl font-bold text-white font-mono">"SEO"</div>
                            <div class="text-xs text-gray-500 uppercase tracking-widest">"Optimized"</div>
                        </div>
                        <div>
                            <div class="text-3xl font-bold text-white font-mono">"24/7"</div>
                            <div class="text-xs text-gray-500 uppercase tracking-widest">"Support"</div>
                        </div>
                    </div>
                </div>
            </section>

            // TARGET SECTORS
            <section class="py-24 px-4 bg-void-surface relative">
                <div class="absolute inset-x-0 top-0 h-px bg-gradient-to-r from-transparent via-brand-dark/50 to-transparent"></div>
                
                <div class="max-w-7xl mx-auto">
                    <div class="flex flex-col md:flex-row justify-between items-end mb-16 gap-6">
                        <div>
                            <h2 class="text-4xl md:text-5xl font-bold text-white tracking-tighter mb-2">"DEPLOY TARGETS"</h2>
                            <p class="text-gray-400 max-w-md">"Specialized modules for specific trade industries."</p>
                        </div>
                        <a href="/pricing" class="btn-ghost group">
                            "View All Modules" 
                            <span class="group-hover:translate-x-1 transition-transform inline-block ml-1">"->"</span>
                        </a>
                    </div>

                    <div class="grid md:grid-cols-2 gap-8">
                        // Electrician Card
                        <a href="/electrician-coventry" class="group block relative overflow-hidden rounded-3xl bg-void border border-void-highlight hover:border-yellow-500/50 transition-all duration-300">
                            <div class="absolute inset-0 bg-gradient-to-br from-yellow-500/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity"></div>
                            <div class="p-8 relative z-10">
                                <div class="w-14 h-14 bg-void-surface border border-void-highlight rounded-xl flex items-center justify-center text-yellow-500 mb-6 group-hover:scale-110 transition-transform">
                                    <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/></svg>
                                </div>
                                <h3 class="text-2xl font-bold text-white mb-2 font-heading">"ELECTRICIAN_MODULE"</h3>
                                <p class="text-gray-500 mb-6 font-mono text-sm">"NICEIC COMPLIANT // HIGH VOLTAGE"</p>
                                <span class="text-yellow-500 font-semibold group-hover:underline decoration-yellow-500/30 underline-offset-4">"Execute Protocol ->"</span>
                            </div>
                        </a>

                        // Plumber Card
                        <a href="/plumber-coventry" class="group block relative overflow-hidden rounded-3xl bg-void border border-void-highlight hover:border-cyan-500/50 transition-all duration-300">
                            <div class="absolute inset-0 bg-gradient-to-br from-cyan-500/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity"></div>
                            <div class="p-8 relative z-10">
                                <div class="w-14 h-14 bg-void-surface border border-void-highlight rounded-xl flex items-center justify-center text-cyan-500 mb-6 group-hover:scale-110 transition-transform">
                                    <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.384-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"/></svg>
                                </div>
                                <h3 class="text-2xl font-bold text-white mb-2 font-heading">"PLUMBER_MODULE"</h3>
                                <p class="text-gray-500 mb-6 font-mono text-sm">"GAS SAFE READY // EMERGENCY RESPONSE"</p>
                                <span class="text-cyan-500 font-semibold group-hover:underline decoration-cyan-500/30 underline-offset-4">"Execute Protocol ->"</span>
                            </div>
                        </a>
                    </div>
                </div>
            </section>

            // FEATURES
            <section class="py-24 px-4 bg-void relative overflow-hidden">
                <div class="max-w-7xl mx-auto">
                     <div class="mb-16">
                        <span class="text-brand font-mono text-sm tracking-widest uppercase mb-2 block">"Core Capabilities"</span>
                        <h2 class="text-4xl md:text-5xl font-bold text-white tracking-tighter">"SYSTEM SPECIFICATIONS"</h2>
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
                        
                        <div class="relative">
                           // Abstract Tech Visualization
                           <div class="absolute inset-0 bg-brand/5 blur-3xl rounded-full"></div>
                           <div class="card-glass relative z-10 backdrop-blur-xl bg-void/40 border-brand/20">
                               <div class="space-y-8">
                                    <div class="flex items-center gap-6">
                                        <div class="w-12 h-12 rounded-lg bg-brand/10 border border-brand/20 flex items-center justify-center text-brand font-mono">"01"</div>
                                        <div>
                                            <h3 class="text-white font-bold text-lg">"Deploy Template"</h3>
                                            <p class="text-gray-500 text-sm">"Select industry-specific configuration."</p>
                                        </div>
                                    </div>
                                    <div class="w-0.5 h-8 bg-void-highlight ml-6"></div>
                                    <div class="flex items-center gap-6">
                                        <div class="w-12 h-12 rounded-lg bg-brand/10 border border-brand/20 flex items-center justify-center text-brand font-mono">"02"</div>
                                        <div>
                                            <h3 class="text-white font-bold text-lg">"Input Data"</h3>
                                            <p class="text-gray-500 text-sm">"Inject business logic and assets."</p>
                                        </div>
                                    </div>
                                    <div class="w-0.5 h-8 bg-void-highlight ml-6"></div>
                                    <div class="flex items-center gap-6">
                                        <div class="w-12 h-12 rounded-lg bg-brand/20 border border-brand/40 flex items-center justify-center text-brand-light font-mono shadow-[0_0_15px_rgba(220,38,38,0.3)]">"03"</div>
                                        <div>
                                            <h3 class="text-white font-bold text-lg">"Launch System"</h3>
                                            <p class="text-gray-500 text-sm">"Go live on global edge network."</p>
                                        </div>
                                    </div>
                               </div>
                           </div>
                        </div>
                    </div>
                </div>
            </section>

            // CTA
            <section class="relative py-32 px-4 text-center overflow-hidden">
                <div class="absolute inset-0 bg-brand-deep"></div>
                 <div class="absolute inset-0 bg-[url('https://www.transparenttextures.com/patterns/carbon-fibre.png')] opacity-10 mix-blend-overlay"></div>
                <div class="absolute inset-0 bg-gradient-to-t from-void via-transparent to-void opacity-80"></div>
                
                <div class="relative max-w-4xl mx-auto z-10">
                    <h2 class="text-5xl md:text-6xl font-black text-white mb-8 tracking-tighter">"READY TO ASCEND?"</h2>
                    <p class="text-gray-300 text-xl mb-10 w-3/4 mx-auto font-light">
                        "Join the elite tradespeople dominating their local sectors."
                    </p>
                    <Button 
                        label="INITIATE SUCCESS" 
                        href="/pricing" 
                        variant=ButtonVariant::Primary
                    />
                </div>
            </section>

            // FOOTER / AREAS
            <section class="bg-void py-16 px-4 border-t border-void-highlight text-center">
                 <div class="max-w-6xl mx-auto opacity-40 hover:opacity-100 transition-opacity duration-500">
                    <h3 class="text-xs font-mono text-gray-500 uppercase tracking-widest mb-6">"Operational Zones // CV_WAR_AREA"</h3>
                    <div class="flex flex-wrap justify-center gap-x-6 gap-y-2 text-gray-600 text-xs font-mono">
                        <span>"[COVENTRY_CENTRAL]"</span>
                        <span>"[EARLSDON_SECTOR]"</span>
                        <span>"[BINLEY_ZONE]"</span>
                        <span>"[FINHAM_DISTRICT]"</span>
                        <span>"[KENILWORTH_OUTPOST]"</span>
                        <span>"[LEAMINGTON_HUB]"</span>
                        <span>"[WARWICK_BASE]"</span>
                    </div>
                </div>
            </section>
        </div>
    }
}
