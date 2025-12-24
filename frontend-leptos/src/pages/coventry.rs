//! Coventry location landing page.
//!
//! Local SEO-optimized page for Coventry area.

use crate::components::seo::SeoHead;
use crate::components::ui::{
    Badge, BadgeVariant, Button, ButtonVariant, FeatureList, Section, StatCard,
};
use crate::components::CoventryServiceMap;
use leptos::prelude::*;
use leptos_meta::Script;
use shared::PageMetadata;

#[component]
pub fn Coventry() -> impl IntoView {
    let schema = r#"{
    "@context": "https://schema.org",
    "@type": "LocalBusiness",
    "name": "XF Tradesmen - Coventry",
    "url": "https://xftradesmen.com/coventry",
    "image": "https://xftradesmen.com/coventry-office.jpg",
    "description": "Professional website builder for Coventry tradesmen",
    "address": {
      "@type": "PostalAddress",
      "addressLocality": "Coventry",
      "addressRegion": "England",
      "addressCountry": "GB"
    },
    \"priceRange\": \"£30-£200\",
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
            title: "Website Builder for Coventry Tradesmen | XF Tradesmen".to_string(),
            description: "Coventry's #1 website builder for plumbers, electricians, and handymen. Get found in Coventry & Warwickshire. Start free.".to_string(),
            canonical_url: Some("https://xftradesmen.com/coventry".to_string()),
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
                    <div class="mb-6 flex justify-center">
                        <Badge label="Location: Coventry & Warwickshire" variant=BadgeVariant::Brand />
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

            // Features Section using Section component
            <Section title="WHY COVENTRY TRADES CHOOSE US">
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

                    <div class="grid grid-cols-2 gap-4">
                        <StatCard value="CV1-6" label="Primary Zone" />
                        <StatCard value="#1" label="Rank Goal" />
                        <StatCard value="20km" label="Coverage Radius" />
                        <StatCard value="100%" label="Local Focus" />
                    </div>
                </div>
            </Section>

            // Service Area Map
            <Section title="SERVICE COVERAGE AREA" darker=true>
                <p class="text-gray-400 mb-8 max-w-2xl">
                    "We specialize in helping tradespeople dominate the Coventry and Warwickshire area. Our SEO strategies are tailored to local search patterns."
                </p>
                <CoventryServiceMap />
            </Section>
        </div>
    }
}
