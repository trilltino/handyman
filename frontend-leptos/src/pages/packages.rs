//! Packages page component.
//!
//! Service packages and Stripe payment integration.

use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::api::stripe::get_stripe_config;

use crate::components::seo::SeoHead;
use shared::PageMetadata;
use leptos_meta::Script;

#[component]
pub fn Packages() -> impl IntoView {
    let (stripe_config, set_stripe_config) = signal(Option::<Result<crate::api::stripe::StripeConfig, String>>::None);
    
    {
        spawn_local(async move {
            set_stripe_config.set(Some(get_stripe_config().await));
        });
    }

    view! {
        <SeoHead metadata=PageMetadata {
            title: "Website Plans & Pricing - Affordable for Handymen | Handyman Marketplace".to_string(),
            description: "Choose your perfect handyman website plan. Simple pricing starting at $29/month. Launch in minutes with our proven templates.".to_string(),
            canonical_url: Some("https://handymanmarketplace.com/pricing".to_string()),
            og_image: None,
        }/>


        <div class="space-y-0 overflow-x-hidden">
            // Hero
            <section class="bg-void relative border-b border-void-highlight text-white py-32 px-4 overflow-hidden">
                <div class="absolute inset-0 bg-cyber-grid bg-[length:40px_40px] opacity-20"></div>
                <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[800px] h-[800px] bg-brand/5 blur-[120px] rounded-full pointer-events-none"></div>

                <div class="max-w-6xl mx-auto text-center relative z-10 animate-fade-in">
                    <span class="text-brand font-mono text-sm tracking-widest uppercase mb-4 block">"System Access"</span>
                    <h1 class="text-5xl md:text-7xl font-heading font-black mb-6 tracking-tighter">"SIMPLE," <span class="text-brand">"TRANSPARENT"</span> " PRICING"</h1>
                    <p class="text-xl text-gray-400 max-w-2xl mx-auto font-light">"One plan. All the features you need to get found and book more jobs."</p>
                </div>
            </section>

            // Pricing Card Section
            <section class="py-20 px-4 bg-void-surface relative">
                <div class="max-w-6xl mx-auto">
                    <div class="max-w-2xl mx-auto relative group">
                        // Glow Effect
                        <div class="absolute -inset-1 bg-gradient-to-r from-brand to-brand-dark rounded-2xl blur opacity-25 group-hover:opacity-50 transition duration-1000 group-hover:duration-200"></div>
                        
                        <div class="relative bg-void border border-void-highlight rounded-2xl p-8 md:p-12 shadow-2xl">
                            <div class="absolute top-0 right-0 p-4">
                                <span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-mono font-bold bg-brand/10 text-brand border border-brand/20 uppercase tracking-wide">"Most Popular"</span>
                            </div>

                            <h2 class="text-3xl font-bold text-white mb-2 font-heading">"Professional Website"</h2>
                            <p class="text-gray-400 mb-8 border-b border-void-highlight pb-8">"Everything you need to get found and book more jobs"</p>
                            
                            <div class="flex items-baseline mb-8">
                                <span class="text-6xl font-black text-white tracking-tighter">"$29"</span>
                                <span class="text-xl text-gray-400 ml-2">" / month"</span>
                            </div>

                            <ul class="space-y-4 mb-10">
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
                                    "Contact forms"
                                </li>
                                <li class="flex items-center gap-3 text-gray-300">
                                    <div class="w-6 h-6 rounded-full bg-brand/10 flex items-center justify-center text-brand flex-shrink-0">
                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                    </div>
                                    "24/7 customer support"
                                </li>
                                <li class="flex items-center gap-3 text-gray-300">
                                    <div class="w-6 h-6 rounded-full bg-brand/10 flex items-center justify-center text-brand flex-shrink-0">
                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                    </div>
                                    "Free updates and improvements"
                                </li>
                            </ul>

                            <Suspense fallback=move || view! { <div class="text-gray-400 text-center py-4 animate-pulse">"Loading payment gateway..."</div> }>
                                {move || {
                                    stripe_config.get().map(|result| match &result {
                                        Ok(config) => {
                                            let product_id = config.product_id.clone();
                                            view! {
                                                <button
                                                    on:click=move |_| {
                                                        let checkout_url = format!(
                                                            "https://checkout.stripe.com/pay/{}",
                                                            product_id.clone()
                                                        );
                                                        if let Some(window) = web_sys::window() {
                                                            let _ = window.location().set_href(&checkout_url);
                                                        }
                                                    }
                                                    class="w-full btn-primary group relative overflow-hidden"
                                                >
                                                    <span class="relative z-10 flex items-center justify-center gap-2">
                                                        "INITIATE SUBSCRIPTION"
                                                        <svg class="w-5 h-5 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/></svg>
                                                    </span>
                                                </button>
                                            }.into_any()
                                        },
                                        Err(e) => view! {
                                            <div class="p-4 bg-red-900/20 border border-red-500/50 rounded text-red-400 text-center text-sm">{format!("Error loading pricing: {}", e)}</div>
                                        }.into_any()
                                    })
                                }}
                            </Suspense>
                            
                            <p class="text-center text-gray-500 text-xs mt-4">"Secure SSL Encryption. Cancel Anytime."</p>
                        </div>
                    </div>
                </div>
            </section>

            // Features Grid
            <section class="bg-void border-t border-void-highlight py-24 px-4">
                <div class="max-w-6xl mx-auto">
                    <div class="text-center mb-16">
                        <h2 class="text-3xl font-bold text-white mb-4 font-heading">"Why Choose Handyman Marketplace?"</h2>
                        <div class="h-1 w-20 bg-brand mx-auto rounded-full"></div>
                    </div>
                    
                    <div class="grid md:grid-cols-3 gap-8">
                        <div class="card-deep">
                            <div class="w-12 h-12 bg-void-highlight rounded-lg flex items-center justify-center text-brand mb-6">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"/></svg>
                            </div>
                            <h3 class="text-xl font-bold text-white mb-3">"Built for Handymen"</h3>
                            <p class="text-gray-400">"Designed specifically for trades professionals, with templates that highlight your expertise and services."</p>
                        </div>
                        <div class="card-deep">
                             <div class="w-12 h-12 bg-void-highlight rounded-lg flex items-center justify-center text-brand mb-6">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/></svg>
                            </div>
                            <h3 class="text-xl font-bold text-white mb-3">"SEO Optimized"</h3>
                            <p class="text-gray-400">"Built-in local SEO optimization helps you rank higher in search results and get found by customers in your area."</p>
                        </div>
                        <div class="card-deep">
                             <div class="w-12 h-12 bg-void-highlight rounded-lg flex items-center justify-center text-brand mb-6">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
                            </div>
                            <h3 class="text-xl font-bold text-white mb-3">"No Setup Fee"</h3>
                            <p class="text-gray-400">"Cancel anytime, no contracts or hidden fees. Start with a fully functional website for just $29/month."</p>
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
                            <h3 class="text-lg font-bold text-white mb-2">"Do I need to sign a contract?"</h3>
                            <p class="text-gray-400">"No, there are no long-term contracts. Our service is month-to-month, and you can cancel at any time without penalty."</p>
                        </div>
                        
                        <div class="border border-void-highlight rounded-xl p-6 bg-void hover:border-brand/30 transition-colors">
                            <h3 class="text-lg font-bold text-white mb-2">"Is hosting included?"</h3>
                            <p class="text-gray-400">"Yes, fast and secure cloud hosting is included in your £29/month subscription. We handle all the technical maintenance."</p>
                        </div>

                        <div class="border border-void-highlight rounded-xl p-6 bg-void hover:border-brand/30 transition-colors">
                            <h3 class="text-lg font-bold text-white mb-2">"Can I use my own domain name?"</h3>
                            <p class="text-gray-400">"Absolutely. We can use a domain you already own, or assist you in purchasing a new one for your business."</p>
                        </div>

                         <div class="border border-void-highlight rounded-xl p-6 bg-void hover:border-brand/30 transition-colors">
                            <h3 class="text-lg font-bold text-white mb-2">"How long does it take to launch?"</h3>
                            <p class="text-gray-400">"Most sites are live within 24-48 hours. Once you choose a template and send us your details, we get to work immediately."</p>
                        </div>
                    </div>
                </div>
                
                <Script type_="application/ld+json">
                {r#"{
                  "@context": "https://schema.org",
                  "@type": "FAQPage",
                  "mainEntity": [{
                    "@type": "Question",
                    "name": "Do I need to sign a contract?",
                    "acceptedAnswer": {
                      "@type": "Answer",
                      "text": "No, there are no long-term contracts. Our service is month-to-month, and you can cancel at any time without penalty."
                    }
                  }, {
                    "@type": "Question",
                    "name": "Is hosting included?",
                    "acceptedAnswer": {
                      "@type": "Answer",
                      "text": "Yes, fast and secure cloud hosting is included in your £29/month subscription. We handle all the technical maintenance."
                    }
                  }, {
                    "@type": "Question",
                    "name": "Can I use my own domain name?",
                    "acceptedAnswer": {
                      "@type": "Answer",
                      "text": "Absolutely. We can use a domain you already own, or assist you in purchasing a new one for your business."
                    }
                  }, {
                    "@type": "Question",
                    "name": "How long does it take to launch?",
                    "acceptedAnswer": {
                      "@type": "Answer",
                      "text": "Most sites are live within 24-48 hours. Once you choose a template and send us your details, we get to work immediately."
                    }
                  }]
                }"#}
                </Script>
            </section>

            <section class="bg-void border-t border-void-highlight py-20 px-4">
                <div class="max-w-2xl mx-auto text-center">
                    <h2 class="text-3xl md:text-4xl font-black text-white mb-6 uppercase tracking-tight">"Ready to Get Started?"</h2>
                    <p class="text-gray-400 mb-8 text-lg">"Launch your professional handyman website in minutes. Get found online and book more jobs."</p>
                    <a href="/contact" class="btn-secondary">"Have questions? Contact us"</a>
                </div>
            </section>
        </div>
    }
}
