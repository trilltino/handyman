//! Electrician services page.
//!
//! Service-specific landing page for electrical work.

use leptos::prelude::*;
use crate::components::seo::SeoHead;
use shared::PageMetadata;
use leptos_meta::Script;
use crate::components::ui::{FeatureList, Button, ButtonVariant};

#[component]
pub fn Electrician() -> impl IntoView {
    let schema = r#"{
    "@context": "https://schema.org",
    "@type": "Service",
    "serviceType": "Electrician Website Builder",
    "provider": {
      "@type": "LocalBusiness",
      "name": "XF Tradesmen - Coventry",
      "address": {
        "@type": "PostalAddress",
        "addressLocality": "Coventry",
        "addressRegion": "England",
        "addressCountry": "GB"
      }
    },
    "areaServed": {
      "@type": "City",
      "name": "Coventry"
    },
    "description": "Professional websites for electricians in Coventry. Get more emergency calls and electrical installation jobs."
    }"#;

    view! {
        <SeoHead metadata=PageMetadata {
            title: "Electrician Website Design Coventry - Get More Calls | XF Tradesmen".to_string(),
            description: "Best website builder for Coventry electricians. Rank for 'Emergency Electrician Coventry' and book more rewiring and installation jobs.".to_string(),
            canonical_url: Some("https://xftradesmen.com/electrician-coventry".to_string()),
            og_image: None,
        }/>
        <Script type_="application/ld+json">{schema}</Script>

        <div class="space-y-0 overflow-x-hidden">
            // Hero
            <section class="relative bg-void min-h-[90vh] flex items-center pt-20 pb-32 px-4 overflow-hidden border-b border-void-highlight">
                <div class="absolute inset-0 bg-cyber-grid bg-[length:40px_40px] opacity-20"></div>
                // Yellow "Voltage" Glow
                <div class="absolute top-0 right-0 w-[600px] h-[600px] bg-yellow-500/10 blur-[150px] rounded-full pointer-events-none animate-pulse-slow"></div>
                
                <div class="relative max-w-7xl mx-auto z-10 grid md:grid-cols-2 gap-12 items-center">
                     <div class="animate-fade-in text-left">
                        <div class="inline-flex items-center gap-2 mb-6 px-3 py-1 rounded-full bg-yellow-500/10 border border-yellow-500/20">
                            <span class="w-1.5 h-1.5 rounded-full bg-yellow-500 animate-pulse"></span>
                            <span class="text-yellow-500 text-xs font-mono font-bold tracking-widest uppercase">"Module: Electrician_Core"</span>
                        </div>
                    
                        <h1 class="text-5xl md:text-7xl font-heading font-black tracking-tighter mb-6 leading-[0.9] text-white">
                            "HIGH" <br/>
                            <span class="text-transparent bg-clip-text bg-gradient-to-r from-yellow-400 to-yellow-600">
                                "VOLTAGE"
                            </span> <br/>
                            "RESULTS"
                        </h1>
                        
                        <p class="text-xl text-gray-400 max-w-xl mb-10 leading-relaxed font-light">
                            "Dominate the local grid. Specialized digital infrastructure for <span class='text-white font-semibold'>NAPIT</span> and <span class='text-white font-semibold'>NICEIC</span> certified professionals."
                        </p>
                        
                        <div class="flex flex-col sm:flex-row gap-4">
                            <Button 
                                label="DEPLOY SITE" 
                                href="/pricing" 
                                variant=ButtonVariant::Primary 
                            />
                             <Button 
                                label="VIEW DEMO" 
                                href="/contact" 
                                variant=ButtonVariant::Secondary 
                            />
                        </div>
                    </div>

                    // Visual Asset
                    <div class="relative animate-fade-in-delay">
                         <div class="absolute inset-0 bg-gradient-to-tr from-yellow-500/20 to-transparent blur-2xl rounded-3xl"></div>
                         <div class="card-deep border-yellow-500/20 bg-void-surface/50 backdrop-blur-xl relative overflow-hidden p-8">
                            <div class="flex justify-between items-start mb-8">
                                <div class="w-12 h-12 bg-yellow-500/20 rounded-lg flex items-center justify-center text-yellow-500">
                                   <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/></svg>
                                </div>
                                <div class="text-right">
                                    <div class="text-xs text-gray-500 font-mono uppercase">"Status"</div>
                                    <div class="text-green-400 font-bold text-sm">"ONLINE"</div>
                                </div>
                            </div>
                            <div class="space-y-4">
                                <div class="h-2 bg-void-highlight rounded-full w-3/4"></div>
                                <div class="h-2 bg-void-highlight rounded-full w-1/2"></div>
                                <div class="h-2 bg-void-highlight rounded-full w-full"></div>
                            </div>
                            <div class="mt-8 border-t border-void-highlight pt-6 flex justify-between items-center">
                                <span class="text-sm text-gray-400">"Local SEO Score"</span>
                                <span class="text-yellow-400 font-bold font-mono">"98/100"</span>
                            </div>
                         </div>
                    </div>
                </div>
            </section>

            <section class="py-24 px-4 bg-void-surface relative">
                <div class="max-w-6xl mx-auto">
                    <div class="mb-16">
                        <h2 class="text-3xl font-bold text-white tracking-tight font-heading mb-4">"POWERED FEATURES"</h2>
                        <div class="h-1 w-24 bg-gradient-to-r from-yellow-500 to-transparent rounded-full"></div>
                    </div>
                    
                    <div class="grid md:grid-cols-2 gap-16 items-center">
                        <div>
                            <p class="text-lg text-gray-300 leading-relaxed mb-8">
                                "We understand the electrical trade. Your website isn't just a brochure; it's a funnel for emergency calls, rewires, and commercial contracts."
                            </p>
                            <FeatureList features=vec![
                                "NICEIC / NAPIT Badge Integration".to_string(),
                                "Emergency 'Click-to-Call' Floating Buttons".to_string(),
                                "Commercial vs Domestic Service Splitting".to_string(),
                                "PAT Testing & EICR Landing Pages".to_string(),
                            ] />
                        </div>
                        <div class="grid grid-cols-2 gap-4">
                            <div class="card-deep text-center py-8 border-yellow-500/10">
                                <div class="text-3xl font-bold text-white mb-2 font-mono">"24/7"</div>
                                <div class="text-xs text-gray-500 uppercase">"Emergency Ready"</div>
                            </div>
                             <div class="card-deep text-center py-8 border-yellow-500/10">
                                <div class="text-3xl font-bold text-white mb-2 font-mono">"100%"</div>
                                <div class="text-xs text-gray-500 uppercase">"Compliance"</div>
                            </div>
                             <div class="card-deep text-center py-8 border-yellow-500/10">
                                <div class="text-3xl font-bold text-white mb-2 font-mono">"0.1s"</div>
                                <div class="text-xs text-gray-500 uppercase">"Load Time"</div>
                            </div>
                             <div class="card-deep text-center py-8 border-yellow-500/10">
                                <div class="text-3xl font-bold text-white mb-2 font-mono">"SEO"</div>
                                <div class="text-xs text-gray-500 uppercase">"Optimized"</div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            
            <section class="py-24 px-4 bg-void text-center border-t border-void-highlight">
                <div class="max-w-4xl mx-auto">
                     <h2 class="text-3xl md:text-5xl font-black text-white mb-8 tracking-tighter">"COMPLETE THE CIRCUIT"</h2>
                    <p class="text-gray-400 text-xl mb-12">"Launch your electrical business into the modern era."</p>
                    <Button 
                        label="GET STARTED" 
                        href="/pricing" 
                        variant=ButtonVariant::Primary
                    />
                </div>
            </section>
        </div>
    }
}
