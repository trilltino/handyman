//! Plumber services page.
//!
//! Service-specific landing page for plumbing work.

use leptos::prelude::*;
use crate::components::seo::SeoHead;
use shared::PageMetadata;
use leptos_meta::Script;
use crate::components::ui::{FeatureList, Button, ButtonVariant};

#[component]
pub fn Plumber() -> impl IntoView {
    let schema = r#"{
    "@context": "https://schema.org",
    "@type": "Service",
    "serviceType": "Plumber Website Builder",
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
    "description": "Professional websites for plumbers in Coventry. Get more emergency leaks and bathroom installation jobs."
    }"#;

    view! {
        <SeoHead metadata=PageMetadata {
            title: "Plumber Website Design Coventry - Get More Jobs | XF Tradesmen".to_string(),
            description: "Best website builder for Coventry plumbers. Rank for 'Emergency Plumber Coventry' and book more bathroom fit-outs and boiler repairs.".to_string(),
            canonical_url: Some("https://xftradesmen.com/plumber-coventry".to_string()),
            og_image: None,
        }/>
        <Script type_="application/ld+json">{schema}</Script>

        <div class="space-y-0 overflow-x-hidden">
             // Hero
            <section class="relative bg-void min-h-[90vh] flex items-center pt-20 pb-32 px-4 overflow-hidden border-b border-void-highlight">
                <div class="absolute inset-0 bg-cyber-grid bg-[length:40px_40px] opacity-20"></div>
                // Cyan "Hydro" Glow
                <div class="absolute bottom-0 left-0 w-[600px] h-[600px] bg-cyan-500/10 blur-[150px] rounded-full pointer-events-none animate-pulse-slow"></div>
                
                <div class="relative max-w-7xl mx-auto z-10 grid md:grid-cols-2 gap-12 items-center">
                     <div class="animate-fade-in text-left">
                        <div class="inline-flex items-center gap-2 mb-6 px-3 py-1 rounded-full bg-cyan-500/10 border border-cyan-500/20">
                            <span class="w-1.5 h-1.5 rounded-full bg-cyan-500 animate-pulse"></span>
                            <span class="text-cyan-500 text-xs font-mono font-bold tracking-widest uppercase">"Module: Plumber_Core"</span>
                        </div>
                    
                        <h1 class="text-5xl md:text-7xl font-heading font-black tracking-tighter mb-6 leading-[0.9] text-white">
                            "LIQUID" <br/>
                            <span class="text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-cyan-600">
                                "ASSETS"
                            </span> <br/>
                            "REGENERATED"
                        </h1>
                        
                        <p class="text-xl text-gray-400 max-w-xl mb-10 leading-relaxed font-light">
                            "Flood your diary with high-value jobs. Specialized systems for <span class='text-white font-semibold'>Gas Safe</span> engineers and emergency responders."
                        </p>
                        
                        <div class="flex flex-col sm:flex-row gap-4">
                            <Button 
                                label="DEPLOY SITE" 
                                href="/pricing" 
                                variant=ButtonVariant::Primary 
                            />
                             <Button 
                                label="SYSTEM SPECS" 
                                href="/about" 
                                variant=ButtonVariant::Secondary 
                            />
                        </div>
                    </div>

                    // Visual Asset
                    <div class="relative animate-fade-in-delay">
                         <div class="absolute inset-0 bg-gradient-to-tr from-cyan-500/20 to-transparent blur-2xl rounded-3xl"></div>
                         <div class="card-deep border-cyan-500/20 bg-void-surface/50 backdrop-blur-xl relative overflow-hidden p-8">
                            <div class="flex justify-between items-start mb-8">
                                <div class="w-12 h-12 bg-cyan-500/20 rounded-lg flex items-center justify-center text-cyan-500">
                                   <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.384-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"/></svg>
                                </div>
                                <div class="text-right">
                                    <div class="text-xs text-gray-500 font-mono uppercase">"Flow Rate"</div>
                                    <div class="text-cyan-400 font-bold text-sm">"OPTIMAL"</div>
                                </div>
                            </div>
                            <div class="grid grid-cols-2 gap-4">
                                <div class="bg-void p-4 rounded-lg border border-void-highlight">
                                    <div class="text-xl font-bold text-white mb-1">"15+"</div>
                                    <div class="text-[10px] text-gray-500 uppercase">"Emergency Calls"</div>
                                </div>
                                <div class="bg-void p-4 rounded-lg border border-void-highlight">
                                    <div class="text-xl font-bold text-white mb-1">"3"</div>
                                    <div class="text-[10px] text-gray-500 uppercase">"New Installs"</div>
                                </div>
                            </div>
                            <div class="mt-8 border-t border-void-highlight pt-6">
                                <span class="text-xs text-gray-500 block mb-2">"System Status: Generating Leads"</span>
                                <div class="w-full bg-void-highlight h-1.5 rounded-full overflow-hidden">
                                    <div class="bg-cyan-500 h-full w-3/4 animate-pulse"></div>
                                </div>
                            </div>
                         </div>
                    </div>
                </div>
            </section>

             <section class="py-24 px-4 bg-void-surface relative">
                <div class="max-w-6xl mx-auto">
                    <div class="mb-16">
                        <h2 class="text-3xl font-bold text-white tracking-tight font-heading mb-4">"FLUID FEATURES"</h2>
                        <div class="h-1 w-24 bg-gradient-to-r from-cyan-500 to-transparent rounded-full"></div>
                    </div>
                    
                    <div class="grid md:grid-cols-2 gap-16 items-center">
                        <div>
                            <p class="text-lg text-gray-300 leading-relaxed mb-8">
                                "Highlight your <span class='text-white font-semibold'>Gas Safe</span> registration and expertise. Our plumbing templates are designed for rapid conversion during domestic emergencies."
                            </p>
                            <FeatureList features=vec![
                                "Gas Safe Register Integration".to_string(),
                                "Emergency 'One-Tap' Dialing".to_string(),
                                "Boiler Service Automated Reminders".to_string(),
                                "Bathroom Renovation Portfolio Grids".to_string(),
                            ] />
                        </div>
                         <div class="grid grid-cols-2 gap-4">
                            <div class="card-deep text-center py-8 border-cyan-500/10 hover:border-cyan-500/30 transition-colors">
                                <svg class="w-8 h-8 mx-auto mb-4 text-cyan-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
                                <div class="text-sm font-bold text-white uppercase">"Rapid Response"</div>
                            </div>
                             <div class="card-deep text-center py-8 border-cyan-500/10 hover:border-cyan-500/30 transition-colors">
                                <svg class="w-8 h-8 mx-auto mb-4 text-cyan-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
                                <div class="text-sm font-bold text-white uppercase">"Verified Pros"</div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
             <section class="py-24 px-4 bg-void text-center border-t border-void-highlight">
                <div class="max-w-4xl mx-auto">
                     <h2 class="text-3xl md:text-5xl font-black text-white mb-8 tracking-tighter">"START THE FLOW"</h2>
                    <p class="text-gray-400 text-xl mb-12">"Get the digital tools your trade deserves."</p>
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
