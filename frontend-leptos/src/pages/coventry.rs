//! Coventry location landing page.
//!
//! Local SEO-optimized page for Coventry area.

use leptos::prelude::*;
use crate::components::seo::SeoHead;
use shared::PageMetadata;
use leptos_meta::Script;
use crate::components::ui::{FeatureList, Button, ButtonVariant};

#[component]
pub fn Coventry() -> impl IntoView {
    let schema = r#"{
    "@context": "https://schema.org",
    "@type": "LocalBusiness",
    "name": "Handyman Marketplace - Coventry",
    "url": "https://handymanmarketplace.com/coventry",
    "image": "https://handymanmarketplace.com/coventry-office.jpg",
    "description": "Professional website builder for Coventry tradesmen",
    "address": {
      "@type": "PostalAddress",
      "addressLocality": "Coventry",
      "addressRegion": "England",
      "addressCountry": "GB"
    },
    "priceRange": "£29-£200",
    "areaServed": [
      {
        "@type": "City",
        "name": "Coventry"
      },
      {
        "@type": "City",
        "name": "Birmingham"
      },
      {
        "@type": "City",
        "name": "Warwick"
      }
    ]
    }"#;

    view! {
        <SeoHead metadata=PageMetadata {
            title: "Website Builder for Coventry Tradesmen | Handyman Marketplace".to_string(),
            description: "Coventry's #1 website builder for plumbers, electricians, and handymen. Get found in Coventry & Warwickshire. Start free.".to_string(),
            canonical_url: Some("https://handymanmarketplace.com/coventry".to_string()),
            og_image: None,
        }/>
        <Script type_="application/ld+json">{schema}</Script>

        <div class="space-y-0 overflow-x-hidden">
            // Hero
            <section class="relative bg-void text-white py-32 px-4 overflow-hidden border-b border-void-highlight">
                <div class="absolute inset-0 bg-cyber-grid bg-[length:40px_40px] opacity-20"></div>
                // Location Beacon Glow
                <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-brand/10 blur-[120px] rounded-full pointer-events-none"></div>

                <div class="relative max-w-6xl mx-auto text-center z-10 animate-fade-in">
                    <div class="inline-flex items-center gap-2 mb-6 px-4 py-1.5 rounded-full bg-void-surface border border-void-highlight text-brand-light text-sm font-mono tracking-widest uppercase shadow-[0_0_15px_rgba(220,38,38,0.3)]">
                         <svg class="w-4 h-4 text-brand" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"/><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"/></svg>
                        "Location: Coventry & Warwickshire"
                    </div>
                
                    <h1 class="text-5xl md:text-7xl font-heading font-black tracking-tighter mb-8 leading-[1.1]">
                        "DOMINATE THE" <br/>
                        <span class="text-transparent bg-clip-text bg-gradient-to-r from-brand to-brand-light">
                            "COVENTRY MARKET"
                        </span>
                    </h1>
                    
                    <p class="text-xl text-gray-400 max-w-2xl mx-auto mb-12 leading-relaxed font-light">
                        "Helping Coventry plumbers, electricians, and tradespeople capture local dominance. Built specifically for the unique needs of the West Midlands trade."
                    </p>
                    
                    <div class="flex flex-col sm:flex-row gap-6 justify-center animate-fade-in-delay">
                        <Button 
                            label="DEPLOY LOCAL SITE" 
                            href="/pricing" 
                            variant=ButtonVariant::Primary 
                        />
                        <Button 
                            label="SUCCESS STORIES" 
                            href="/about" 
                            variant=ButtonVariant::Secondary 
                        />
                    </div>
                </div>
            </section>

            <section class="py-24 px-4 bg-void-surface relative">
                <div class="max-w-6xl mx-auto">
                    <div class="flex items-center gap-4 mb-12">
                        <div class="h-1 w-12 bg-gradient-to-r from-brand to-transparent rounded-full"></div>
                        <h2 class="text-3xl font-bold text-white tracking-tight font-heading">"WHY COVENTRY TRADES CHOOSE US"</h2>
                    </div>
                    
                    <div class="grid md:grid-cols-2 gap-16 items-center">
                        <div>
                            <p class="text-lg text-gray-300 leading-relaxed mb-8">
                                "Competition in the West Midlands is tough. You need a website that ranks specifically for <span class='text-white font-semibold'>'Plumber in Coventry'</span> or <span class='text-white font-semibold'>'Electrician CV1'</span>. We handle the local SEO infrastructure so you can focus on the job."
                            </p>
                            <FeatureList features=vec![
                                "Hardcoded 'Coventry' Keyword Injection".to_string(),
                                "Google Business Profile Sync".to_string(),
                                "CV Postcode Area Targeting".to_string(),
                                "Mobile-First for On-Site Queries".to_string(),
                            ] />
                        </div>
                        
                        <div class="relative">
                             <div class="absolute -inset-1 bg-gradient-to-r from-brand to-transparent opacity-20 blur-lg rounded-xl"></div>
                             <div class="card-deep p-8 relative bg-void">
                                <div class="flex items-center justify-between mb-8 border-b border-void-highlight pb-4">
                                    <div class="font-mono text-xs text-gray-500 uppercase">"Target Region"</div>
                                    <div class="text-brand font-bold">"WEST MIDLANDS"</div>
                                </div>
                                <div class="grid grid-cols-2 gap-4 text-center">
                                    <div class="bg-void-surface rounded p-4 border border-void-highlight">
                                        <div class="text-2xl font-bold text-white mb-1">"CV1-6"</div>
                                        <div class="text-[10px] text-gray-500 uppercase">"Primary Zone"</div>
                                    </div>
                                    <div class="bg-void-surface rounded p-4 border border-void-highlight">
                                        <div class="text-2xl font-bold text-white mb-1">"#1"</div>
                                        <div class="text-[10px] text-gray-500 uppercase">"Rank Goal"</div>
                                    </div>
                                </div>
                             </div>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}
