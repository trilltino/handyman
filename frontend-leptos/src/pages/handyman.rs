//! Handyman services page.
//!
//! Example site landing page for general handyman services.

use crate::components::seo::SeoHead;
use crate::components::ui::{Button, ButtonVariant, FeatureList};
use leptos::prelude::*;
use leptos_meta::Script;
use shared::PageMetadata;

#[component]
pub fn Handyman() -> impl IntoView {
    let schema = r#"{
    "@context": "https://schema.org",
    "@type": "Service",
    "serviceType": "Handyman Website Builder",
    "provider": {
      "@type": "LocalBusiness",
      "name": "XF Tradesmen",
      "address": {
        "@type": "PostalAddress",
        "addressLocality": "Coventry",
        "addressRegion": "England",
        "addressCountry": "GB"
      }
    },
    "areaServed": {
      "@type": "Country",
      "name": "United Kingdom"
    },
    "description": "Professional websites for handymen. Get more local jobs for repairs, maintenance, and home improvement."
    }"#;

    view! {
        <SeoHead metadata=PageMetadata {
            title: "Handyman Website Design - Get More Local Jobs | XF Tradesmen".to_string(),
            description: "Professional website builder for handymen. Rank locally for repairs, maintenance, and home improvement jobs.".to_string(),
            canonical_url: Some("https://xftradesman.com/handyman".to_string()),
            og_image: None,
        }/>
        <Script type_="application/ld+json">{schema}</Script>

        <div class="space-y-0 overflow-x-hidden">
            // Hero
            <section class="relative bg-void min-h-[90vh] flex items-center pt-20 pb-32 px-4 overflow-hidden border-b border-void-highlight">
                <div class="absolute inset-0 bg-cyber-grid bg-[length:40px_40px] opacity-20"></div>
                // Orange "Tool" Glow
                <div class="absolute top-0 right-0 w-[600px] h-[600px] bg-orange-500/10 blur-[150px] rounded-full pointer-events-none animate-pulse-slow"></div>

                <div class="relative max-w-7xl mx-auto z-10 grid md:grid-cols-2 gap-12 items-center">
                     <div class="animate-fade-in text-left">
                        <div class="inline-flex items-center gap-2 mb-6 px-3 py-1 rounded-full bg-orange-500/10 border border-orange-500/20">
                            <span class="w-1.5 h-1.5 rounded-full bg-orange-500 animate-pulse"></span>
                            <span class="text-orange-500 text-xs font-mono font-bold tracking-widest uppercase">"Module: Handyman_Core"</span>
                        </div>

                        <h1 class="text-5xl md:text-7xl font-heading font-black tracking-tighter mb-6 leading-[0.9] text-white">
                            "FIX" <br/>
                            <span class="text-transparent bg-clip-text bg-gradient-to-r from-orange-400 to-orange-600">
                                "EVERYTHING"
                            </span> <br/>
                            "ONLINE"
                        </h1>

                        <p class="text-xl text-gray-400 max-w-xl mb-10 leading-relaxed font-light">
                            "Your all-in-one digital toolkit. Professional infrastructure for the <span class='text-white font-semibold'>jack-of-all-trades</span> who masters everything."
                        </p>

                        <div class="flex flex-col sm:flex-row gap-4">
                            <Button
                                label="GET YOUR CUSTOM QUOTE"
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
                         <div class="absolute inset-0 bg-gradient-to-tr from-orange-500/20 to-transparent blur-2xl rounded-3xl"></div>
                         <div class="card-deep border-orange-500/20 bg-void-surface/50 backdrop-blur-xl relative overflow-hidden p-8">
                            <div class="flex justify-between items-start mb-8">
                                <div class="w-12 h-12 bg-orange-500/20 rounded-lg flex items-center justify-center text-orange-500">
                                   <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/></svg>
                                </div>
                                <div class="text-right">
                                    <div class="text-xs text-gray-500 font-mono uppercase">"Status"</div>
                                    <div class="text-green-400 font-bold text-sm">"READY"</div>
                                </div>
                            </div>
                            <div class="space-y-4">
                                <div class="h-2 bg-void-highlight rounded-full w-3/4"></div>
                                <div class="h-2 bg-void-highlight rounded-full w-1/2"></div>
                                <div class="h-2 bg-void-highlight rounded-full w-full"></div>
                            </div>
                            <div class="mt-8 border-t border-void-highlight pt-6 flex justify-between items-center">
                                <span class="text-sm text-gray-400">"Local SEO Score"</span>
                                <span class="text-orange-400 font-bold font-mono">"95/100"</span>
                            </div>
                         </div>
                    </div>
                </div>
            </section>

            <section class="py-24 px-4 bg-void-surface relative">
                <div class="max-w-6xl mx-auto">
                    <div class="mb-16">
                        <h2 class="text-3xl font-bold text-white tracking-tight font-heading mb-4">"YOUR DIGITAL TOOLBOX"</h2>
                        <div class="h-1 w-24 bg-gradient-to-r from-orange-500 to-transparent rounded-full"></div>
                    </div>

                    <div class="grid md:grid-cols-2 gap-16 items-center">
                        <div>
                            <p class="text-lg text-gray-300 leading-relaxed mb-8">
                                "From flat-pack assembly to full bathroom refits, your website should showcase the breadth of your skills and make it easy for customers to book."
                            </p>
                            <FeatureList features=vec![
                                "Multi-Service Portfolio Gallery".to_string(),
                                "Instant Quote Request Forms".to_string(),
                                "Before & After Project Showcase".to_string(),
                                "Google Reviews Integration".to_string(),
                            ] />
                        </div>
                        <div class="grid grid-cols-2 gap-4">
                            <div class="card-deep text-center py-8 border-orange-500/10">
                                <div class="text-3xl font-bold text-white mb-2 font-mono">"50+"</div>
                                <div class="text-xs text-gray-500 uppercase">"Service Types"</div>
                            </div>
                             <div class="card-deep text-center py-8 border-orange-500/10">
                                <div class="text-3xl font-bold text-white mb-2 font-mono">"5/5"</div>
                                <div class="text-xs text-gray-500 uppercase">"Reviews Ready"</div>
                            </div>
                             <div class="card-deep text-center py-8 border-orange-500/10">
                                <div class="text-3xl font-bold text-white mb-2 font-mono">"0.1s"</div>
                                <div class="text-xs text-gray-500 uppercase">"Load Time"</div>
                            </div>
                             <div class="card-deep text-center py-8 border-orange-500/10">
                                <div class="text-3xl font-bold text-white mb-2 font-mono">"SEO"</div>
                                <div class="text-xs text-gray-500 uppercase">"Optimized"</div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Before & After Gallery Section
            <section class="py-24 px-4 bg-void relative border-t border-void-highlight">
                <div class="max-w-6xl mx-auto">
                    <div class="mb-16 text-center">
                        <h2 class="text-3xl font-bold text-white tracking-tight font-heading mb-4">"BEFORE & AFTER SHOWCASE"</h2>
                        <p class="text-gray-400 max-w-2xl mx-auto">"Show your customers the transformation. Our gallery system makes your work speak for itself."</p>
                        <div class="h-1 w-24 bg-gradient-to-r from-orange-500 to-transparent rounded-full mx-auto mt-6"></div>
                    </div>

                    // Before/After Comparison
                    <div class="grid md:grid-cols-2 gap-8 mb-16">
                        <div class="relative group">
                            <div class="absolute top-4 left-4 bg-red-500/90 text-white text-xs font-bold px-3 py-1 rounded-full z-10">"BEFORE"</div>
                            <img
                                src="/images/projects/bathroom_before.png"
                                alt="Bathroom before renovation - dated UK bathroom with pink tiles"
                                class="w-full h-80 object-cover rounded-xl border border-void-highlight group-hover:border-orange-500/30 transition-colors"
                            />
                        </div>
                        <div class="relative group">
                            <div class="absolute top-4 left-4 bg-green-500/90 text-white text-xs font-bold px-3 py-1 rounded-full z-10">"AFTER"</div>
                            <img
                                src="/images/projects/bathroom_after.png"
                                alt="Bathroom after renovation - modern UK bathroom with grey tiles"
                                class="w-full h-80 object-cover rounded-xl border border-void-highlight group-hover:border-orange-500/30 transition-colors"
                            />
                        </div>
                    </div>

                    // Work in Progress Gallery
                    <div class="mb-8">
                        <h3 class="text-xl font-bold text-white mb-6">"WORK IN PROGRESS"</h3>
                    </div>
                    <div class="grid md:grid-cols-2 gap-8">
                        <div class="relative group overflow-hidden rounded-xl">
                            <img
                                src="/images/projects/handyman_kitchen.png"
                                alt="Handyman installing kitchen cabinet in UK home"
                                class="w-full h-64 object-cover transition-transform group-hover:scale-105"
                            />
                            <div class="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent"></div>
                            <div class="absolute bottom-4 left-4">
                                <div class="text-white font-bold">"Kitchen Installation"</div>
                                <div class="text-gray-400 text-sm">"Cabinet fitting & assembly"</div>
                            </div>
                        </div>
                        <div class="relative group overflow-hidden rounded-xl">
                            <img
                                src="/images/projects/handyman_fence.png"
                                alt="Handyman repairing fence in UK garden"
                                class="w-full h-64 object-cover transition-transform group-hover:scale-105"
                            />
                            <div class="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent"></div>
                            <div class="absolute bottom-4 left-4">
                                <div class="text-white font-bold">"Garden & Exterior"</div>
                                <div class="text-gray-400 text-sm">"Fence repair & maintenance"</div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            <section class="py-24 px-4 bg-void text-center border-t border-void-highlight">
                <div class="max-w-4xl mx-auto">
                     <h2 class="text-3xl md:text-5xl font-black text-white mb-8 tracking-tighter">"GET THE JOB DONE"</h2>
                    <p class="text-gray-400 text-xl mb-12">"Launch your handyman business into the digital age."</p>
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
