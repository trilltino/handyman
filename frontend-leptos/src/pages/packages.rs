//! Packages page component.
//!
//! Service packages with direct Stripe checkout integration.

use crate::components::seo::SeoHead;
use leptos::prelude::*;
use leptos_meta::Script;
use shared::PageMetadata;

/// Stripe Payment Link URL (includes £329 setup + £30/month subscription)
const STRIPE_CHECKOUT_URL: &str = "https://buy.stripe.com/14AcMYfxO5Ddak70Ik83C03";

#[component]
pub fn Packages() -> impl IntoView {
    view! {
        <SeoHead metadata=PageMetadata {
            title: "All-in-One Website Packages for Trades | Lead Gen Ready | XF Tradesmen".to_string(),
            description: "Complete website packages for electricians, plumbers, and builders. Includes custom design, local SEO, and booking system. Get started for £329.".to_string(),
            canonical_url: Some("https://xftradesman.com/packages".to_string()),
            og_image: None,
        }/>


        <div class="space-y-0 overflow-x-hidden">
            // Hero
            <section class="bg-void relative border-b border-void-highlight text-white py-32 px-4 overflow-hidden">
                <div class="absolute inset-0 bg-cyber-grid bg-[length:40px_40px] opacity-20"></div>
                <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[800px] h-[800px] bg-brand/5 blur-[120px] rounded-full pointer-events-none"></div>

                <div class="max-w-6xl mx-auto text-center relative z-10 animate-fade-in">
                    <h1 class="text-5xl md:text-7xl font-heading font-black mb-6 tracking-tighter text-white">"WEBSITE PACKAGES"</h1>
                </div>
            </section>

            // Pricing Card Section - Two Cards Layout
            <section class="py-20 px-4 bg-void-surface relative">
                <div class="max-w-6xl mx-auto">
                    <div class="grid md:grid-cols-2 gap-8">
                        // Card 1: Pricing & CTA
                        <div class="relative group">
                            // Glow Effect
                            <div class="absolute -inset-1 bg-gradient-to-r from-brand to-brand-dark rounded-2xl blur opacity-25 group-hover:opacity-50 transition duration-1000 group-hover:duration-200"></div>

                            <div class="relative bg-void border border-void-highlight rounded-2xl p-8 shadow-2xl h-full flex flex-col">
                                <h2 class="text-3xl font-bold text-white mb-2 font-heading">"Website"</h2>
                                <p class="text-gray-400 mb-8 border-b border-void-highlight pb-8">"Complete tradesman website with SEO, hosting, and support"</p>

                                // Pricing Display
                                <div class="mb-8 flex-grow">
                                    // One-time setup fee
                                    <div class="flex items-baseline gap-2 mb-4">
                                        <span class="text-5xl font-black text-white tracking-tighter">"£329"</span>
                                        <span class="text-lg text-gray-400">"one-time setup"</span>
                                    </div>

                                    // Monthly subscription
                                    <div class="flex items-baseline gap-2 p-4 bg-void-surface/50 rounded-lg border border-void-highlight">
                                        <span class="text-brand text-sm font-mono">"+THEN"</span>
                                        <span class="text-3xl font-bold text-white">"£30"</span>
                                        <span class="text-gray-400">"/ month"</span>
                                    </div>
                                </div>

                                // Direct Stripe checkout link
                                <a
                                    href=STRIPE_CHECKOUT_URL
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="w-full btn btn-primary btn-lg group"
                                >
                                    <span class="flex items-center justify-center gap-2">
                                        "GET STARTED NOW"
                                        <svg class="w-5 h-5 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/></svg>
                                    </span>
                                </a>

                                <p class="text-center text-gray-500 text-xs mt-4">"Secure payment via Stripe. Cancel anytime."</p>
                            </div>
                        </div>

                        // Card 2: Features
                        <div class="relative group">
                            // Glow Effect
                            <div class="absolute -inset-1 bg-gradient-to-r from-brand-dark to-brand/50 rounded-2xl blur opacity-15 group-hover:opacity-30 transition duration-1000 group-hover:duration-200"></div>

                            <div class="relative bg-void border border-void-highlight rounded-2xl p-8 shadow-2xl h-full">
                                <h3 class="text-2xl font-bold text-white mb-6 font-heading">"Everything Included"</h3>

                                <ul class="space-y-4">
                                    <li class="flex items-center gap-3 text-gray-300">
                                        <div class="w-6 h-6 rounded-full bg-brand/10 flex items-center justify-center text-brand flex-shrink-0">
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                        </div>
                                        "Custom domain setup (yourname.co.uk)"
                                    </li>
                                    <li class="flex items-center gap-3 text-gray-300">
                                        <div class="w-6 h-6 rounded-full bg-brand/10 flex items-center justify-center text-brand flex-shrink-0">
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                        </div>
                                        "Mobile-responsive design"
                                    </li>
                                    <li class="flex items-center gap-3 text-gray-300">
                                        <div class="w-6 h-6 rounded-full bg-brand/10 flex items-center justify-center text-brand flex-shrink-0">
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                        </div>
                                        "Local SEO optimization"
                                    </li>
                                    <li class="flex items-center gap-3 text-gray-300">
                                        <div class="w-6 h-6 rounded-full bg-brand/10 flex items-center justify-center text-brand flex-shrink-0">
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                        </div>
                                        "Contact forms & enquiry system"
                                    </li>
                                    <li class="flex items-center gap-3 text-gray-300">
                                        <div class="w-6 h-6 rounded-full bg-brand/10 flex items-center justify-center text-brand flex-shrink-0">
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                        </div>
                                        "Secure SSL hosting included"
                                    </li>
                                    <li class="flex items-center gap-3 text-gray-300">
                                        <div class="w-6 h-6 rounded-full bg-brand/10 flex items-center justify-center text-brand flex-shrink-0">
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                        </div>
                                        "Ongoing support & updates"
                                    </li>
                                    <li class="flex items-center gap-3 text-gray-300">
                                        <div class="w-6 h-6 rounded-full bg-brand/10 flex items-center justify-center text-brand flex-shrink-0">
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                        </div>
                                        "Google Business Profile setup"
                                    </li>
                                    <li class="flex items-center gap-3 text-gray-300">
                                        <div class="w-6 h-6 rounded-full bg-brand/10 flex items-center justify-center text-brand flex-shrink-0">
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                        </div>
                                        "Fast page load speeds"
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
            </section>


            // Features Grid
            <section class="bg-void border-t border-void-highlight py-24 px-4">
                <div class="max-w-6xl mx-auto">
                    <div class="text-center mb-16">
                        <h2 class="text-3xl font-bold text-white mb-4 font-heading">"Why Choose XFTradesmen?"</h2>
                        <div class="h-1 w-20 bg-brand mx-auto rounded-full"></div>
                    </div>

                    <div class="grid md:grid-cols-3 gap-8">
                        <div class="card-deep">
                            <div class="w-12 h-12 bg-void-highlight rounded-lg flex items-center justify-center text-brand mb-6">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"/></svg>
                            </div>
                            <h3 class="text-xl font-bold text-white mb-3">"Built for Tradespeople"</h3>
                            <p class="text-gray-400">"Designed specifically for electricians, plumbers, and contractors. Templates that showcase your services and skills."</p>
                        </div>
                        <div class="card-deep">
                             <div class="w-12 h-12 bg-void-highlight rounded-lg flex items-center justify-center text-brand mb-6">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/></svg>
                            </div>
                            <h3 class="text-xl font-bold text-white mb-3">"SEO Optimized"</h3>
                            <p class="text-gray-400">"Get found on Google. Built-in local SEO helps customers in your area find you when they need your services."</p>
                        </div>
                        <div class="card-deep">
                             <div class="w-12 h-12 bg-void-highlight rounded-lg flex items-center justify-center text-brand mb-6">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
                            </div>
                            <h3 class="text-xl font-bold text-white mb-3">"Simple Pricing"</h3>
                            <p class="text-gray-400">"No hidden fees. £329 setup then £30/month. Cancel anytime with no penalties."</p>
                        </div>
                    </div>
                </div>
            </section>

            // FAQ
            <section class="bg-void-surface py-24 px-4">
                <div class="max-w-4xl mx-auto">
                    <h2 class="text-3xl font-bold text-white mb-12 text-center font-heading">"Frequently Asked Questions"</h2>

                    <div class="grid gap-6">
                        <div class="border border-void-highlight rounded-xl p-6 bg-void hover:border-brand/30 transition-colors">
                            <h3 class="text-lg font-bold text-white mb-2">"What's included in the £329 setup?"</h3>
                            <p class="text-gray-400">"Custom design, domain setup, SEO configuration, contact forms, and full website build. Your site will be live within 72 hours."</p>
                        </div>

                        <div class="border border-void-highlight rounded-xl p-6 bg-void hover:border-brand/30 transition-colors">
                            <h3 class="text-lg font-bold text-white mb-2">"What does the £30/month cover?"</h3>
                            <p class="text-gray-400">"Secure hosting, SSL certificate, ongoing updates, technical support, and SEO maintenance. Everything to keep your site running perfectly."</p>
                        </div>

                        <div class="border border-void-highlight rounded-xl p-6 bg-void hover:border-brand/30 transition-colors">
                            <h3 class="text-lg font-bold text-white mb-2">"Can I cancel anytime?"</h3>
                            <p class="text-gray-400">"Yes, no contracts or lock-in periods. Cancel your monthly subscription whenever you want with no penalty."</p>
                        </div>

                         <div class="border border-void-highlight rounded-xl p-6 bg-void hover:border-brand/30 transition-colors">
                            <h3 class="text-lg font-bold text-white mb-2">"Do I get my own domain?"</h3>
                            <p class="text-gray-400">"Absolutely. We'll set up a professional domain like yourname.co.uk or use an existing domain you already own."</p>
                        </div>
                    </div>
                </div>

                <Script type_="application/ld+json">
                {r#"{
                  "@context": "https://schema.org",
                  "@type": "FAQPage",
                  "mainEntity": [{
                    "@type": "Question",
                    "name": "What's included in the £329 setup?",
                    "acceptedAnswer": {
                      "@type": "Answer",
                      "text": "Custom design, domain setup, SEO configuration, contact forms, and full website build. Your site will be live within 72 hours."
                    }
                  }, {
                    "@type": "Question",
                    "name": "What does the £30/month cover?",
                    "acceptedAnswer": {
                      "@type": "Answer",
                      "text": "Secure hosting, SSL certificate, ongoing updates, technical support, and SEO maintenance."
                    }
                  }, {
                    "@type": "Question",
                    "name": "Can I cancel anytime?",
                    "acceptedAnswer": {
                      "@type": "Answer",
                      "text": "Yes, no contracts or lock-in periods. Cancel your monthly subscription whenever you want with no penalty."
                    }
                  }]
                }"#}
                </Script>
            </section>

            <section class="bg-void border-t border-void-highlight py-20 px-4">
                <div class="max-w-2xl mx-auto text-center">
                    <h2 class="text-3xl md:text-4xl font-black text-white mb-6 uppercase tracking-tight">"Ready to Get Started?"</h2>
                    <p class="text-gray-400 mb-8 text-lg">"Launch your professional tradesman website today."</p>
                    <a href="/contact" class="btn btn-secondary">"Have questions? Contact us"</a>
                </div>
            </section>
        </div>
    }
}
