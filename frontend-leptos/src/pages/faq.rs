//! FAQ page component.
//!
//! Frequently Asked Questions about website development services.

use crate::components::seo::SeoHead;
use crate::components::ui::FaqCard;
use leptos::prelude::*;
use leptos_meta::Script;
use shared::PageMetadata;

#[component]
pub fn Faq() -> impl IntoView {
    view! {
        <SeoHead metadata=PageMetadata {
            title: "FAQ - Website Development Questions | XFSOLUTIONS".to_string(),
            description: "Common questions about our £329 website development and £30/month managed services. Get answers about pricing, timelines, SEO, and technical terms.".to_string(),
            canonical_url: Some("https://xftradesman.com/faq".to_string()),
            og_image: None,
        }/>

        <div class="min-h-screen bg-void text-white">
            // Hero Section
            <section class="bg-void relative border-b border-void-highlight py-24 px-4 overflow-hidden">
                <div class="absolute inset-0 bg-cyber-grid bg-[length:50px_50px] opacity-10"></div>
                <div class="absolute top-0 left-1/2 -translate-x-1/2 w-full h-full bg-gradient-to-b from-brand/5 to-transparent pointer-events-none"></div>

                <div class="max-w-5xl mx-auto text-center relative z-10">
                    <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-brand/10 border border-brand/20 text-brand text-xs font-mono uppercase tracking-widest mb-6">
                        <span class="relative flex h-2 w-2">
                            <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-brand opacity-75"></span>
                            <span class="relative inline-flex rounded-full h-2 w-2 bg-brand"></span>
                        </span>
                        "Knowledge Base"
                    </div>
                    <h1 class="text-5xl md:text-7xl font-heading font-black mb-8 tracking-tighter uppercase italic">
                        "Got" <span class="text-brand">"Questions?"</span> <br/> "We Have Answers."
                    </h1>
                    <p class="text-xl text-gray-400 max-w-2xl mx-auto">
                        "Everything you need to know about our premium website development and managed services."
                    </p>
                </div>
            </section>

            // FAQ Grid Section
            <section class="py-24 px-4 bg-void relative">
                <div class="max-w-7xl mx-auto">
                    <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                        // Category: Getting Started
                        <div class="card-deep group">
                            <div class="w-14 h-14 bg-brand/10 rounded-2xl flex items-center justify-center text-brand mb-6 border border-brand/20 group-hover:bg-brand/20 transition-all duration-300">
                                <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                                </svg>
                            </div>
                            <h2 class="text-2xl font-bold text-white mb-6 font-heading uppercase tracking-tight">"Getting Started"</h2>
                            <div class="space-y-6">
                                <div class="faq-item">
                                    <h3 class="text-brand font-bold text-lg mb-2">"What is included in the £329 fee?"</h3>
                                    <p class="text-gray-400 text-sm leading-relaxed">"Complete design, domain registration (12mo), SEO setup, SSL, and 1 month FREE managed services."</p>
                                </div>
                                <div class="faq-item">
                                    <h3 class="text-brand font-bold text-lg mb-2">"How long does it take?"</h3>
                                    <p class="text-gray-400 text-sm leading-relaxed">"Typically 3-5 days from specification to launch. We deliver an initial version within 36 hours."</p>
                                </div>
                                <div class="faq-item">
                                    <h3 class="text-brand font-bold text-lg mb-2">"Is there a refund policy?"</h3>
                                    <p class="text-gray-400 text-sm leading-relaxed">"Yes, 75% of the setup fee is refundable during the startup period before final acceptance."</p>
                                </div>
                            </div>
                        </div>

                        // Category: Technical Terms
                        <div class="card-deep group">
                            <div class="w-14 h-14 bg-blue-500/10 rounded-2xl flex items-center justify-center text-blue-500 mb-6 border border-blue-500/20 group-hover:bg-blue-500/20 transition-all duration-300">
                                <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"/>
                                </svg>
                            </div>
                            <h2 class="text-2xl font-bold text-white mb-6 font-heading uppercase tracking-tight">"Technical Specs"</h2>
                            <div class="space-y-6">
                                <div class="faq-item">
                                    <h3 class="text-blue-500 font-bold text-lg mb-2">"What is SSL/Encryption?"</h3>
                                    <p class="text-gray-400 text-sm leading-relaxed">"It secures visitor data and shows the padlock icon. We handle all certificates and renewals."</p>
                                </div>
                                <div class="faq-item">
                                    <h3 class="text-blue-500 font-bold text-lg mb-2">"Will it work on mobile?"</h3>
                                    <p class="text-gray-400 text-sm leading-relaxed">"Guaranteed perfect function on Chrome, Safari, Firefox, and Edge across all screen sizes."</p>
                                </div>
                                <div class="faq-item">
                                    <h3 class="text-blue-500 font-bold text-lg mb-2">"What about Google rankings?"</h3>
                                    <p class="text-gray-400 text-sm leading-relaxed">"Initial SEO setup includes meta tags, descriptions, and keyword integration specifically for your local area."</p>
                                </div>
                            </div>
                        </div>

                        // Category: Monthly Managed
                        <div class="card-deep group lg:border-brand/20">
                            <div class="w-14 h-14 bg-emerald-500/10 rounded-2xl flex items-center justify-center text-emerald-500 mb-6 border border-emerald-500/20 group-hover:bg-emerald-500/20 transition-all duration-300">
                                <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>
                                </svg>
                            </div>
                            <h2 class="text-2xl font-bold text-white mb-6 font-heading uppercase tracking-tight">"Managed Service"</h2>
                            <div class="space-y-6">
                                <div class="faq-item">
                                    <h3 class="text-emerald-500 font-bold text-lg mb-2">"What's in the £30/mo?"</h3>
                                    <p class="text-gray-400 text-sm leading-relaxed">"Expert updates (2h/mo), high-speed hosting, SSL management, weekly backups, and 24/7 monitoring."</p>
                                </div>
                                <div class="faq-item">
                                    <h3 class="text-emerald-500 font-bold text-lg mb-2">"How often are backups?"</h3>
                                    <p class="text-gray-400 text-sm leading-relaxed">"Fully automated weekly backups with a 30-day retention period for maximum safety."</p>
                                </div>
                                <div class="faq-item">
                                    <h3 class="text-emerald-500 font-bold text-lg mb-2">"Who fixes bugs?"</h3>
                                    <p class="text-gray-400 text-sm leading-relaxed">"We fix all bugs at no charge. It's part of our ongoing commitment to your site's health."</p>
                                </div>
                            </div>
                        </div>

                        // Category: Ownership & Cancellation
                        <div class="card-deep group">
                            <div class="w-14 h-14 bg-amber-500/10 rounded-2xl flex items-center justify-center text-amber-500 mb-6 border border-amber-500/20 group-hover:bg-amber-500/20 transition-all duration-300">
                                <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                                </svg>
                            </div>
                            <h2 class="text-2xl font-bold text-white mb-6 font-heading uppercase tracking-tight">"Business & Legal"</h2>
                            <div class="space-y-6">
                                <div class="faq-item">
                                    <h3 class="text-amber-500 font-bold text-lg mb-2">"Who owns the site?"</h3>
                                    <p class="text-gray-400 text-sm leading-relaxed">"You own the domain, branding, and content. We grant you perpetual rights to use the site framework."</p>
                                </div>
                                <div class="faq-item">
                                    <h3 class="text-amber-500 font-bold text-lg mb-2">"Can I cancel anytime?"</h3>
                                    <p class="text-gray-400 text-sm leading-relaxed">"Yes, with 30 days notice. No long-term lock-ins or hidden penalties for leaving."</p>
                                </div>
                                <div class="faq-item">
                                    <h3 class="text-amber-500 font-bold text-lg mb-2">"Can I edit the site?"</h3>
                                    <p class="text-gray-400 text-sm leading-relaxed">"You can, but we recommend using your 2h/mo included time so our pros can do it safely."</p>
                                </div>
                            </div>
                        </div>

                        // Category: Security
                        <div class="card-deep group">
                            <div class="w-14 h-14 bg-rose-500/10 rounded-2xl flex items-center justify-center text-rose-500 mb-6 border border-rose-500/20 group-hover:bg-rose-500/20 transition-all duration-300">
                                <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 11c0 3.517-1.009 6.799-2.753 9.571m-3.44-2.04l.054-.09A10.003 10.003 0 0012 3v1m0 16c.54 0 1.062-.072 1.56-.208M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                </svg>
                            </div>
                            <h2 class="text-2xl font-bold text-white mb-6 font-heading uppercase tracking-tight">"Security & Data"</h2>
                            <div class="space-y-6">
                                <div class="faq-item">
                                    <h3 class="text-rose-500 font-bold text-lg mb-2">"How is my data safe?"</h3>
                                    <p class="text-gray-400 text-sm leading-relaxed">"We maintain strict confidentiality and use industry-standard encryption for all internet traffic."</p>
                                </div>
                                <div class="faq-item">
                                    <h3 class="text-rose-500 font-bold text-lg mb-2">"Active protection?"</h3>
                                    <p class="text-gray-400 text-sm leading-relaxed">"Intrusion detection, professional firewalls, and active virus scanning are running 24/7."</p>
                                </div>
                            </div>
                        </div>

                        // Final Contact CTA Card
                        <div class="card-deep bg-void-highlight/50 border-brand/20 flex flex-col justify-center text-center p-12">
                            <h2 class="text-2xl font-bold text-white mb-4 uppercase tracking-tight line-clamp-1">"More Questions?"</h2>
                            <p class="text-gray-400 mb-8 max-w-xs mx-auto text-sm">"Get in touch for personalized answers about your specific website needs."</p>
                            <a href="/contact" class="btn-primary w-full">"Chat With Us"</a>
                        </div>
                    </div>
                </div>
            </section>

            // CTA Secondary Section
            <section class="bg-void-surface border-t border-void-highlight py-24 px-4 overflow-hidden relative">
                <div class="absolute inset-0 bg-cyber-grid bg-[length:30px_30px] opacity-10"></div>
                <div class="max-w-4xl mx-auto text-center relative z-10">
                    <h2 class="text-3xl md:text-5xl font-black text-white mb-8 uppercase tracking-tighter italic">"Ready to Build" <span class="text-brand">"Something Great?"</span></h2>
                    <div class="flex flex-col sm:flex-row gap-6 justify-center">
                        <a href="/pricing" class="btn-primary px-12">"View Packages"</a>
                        <a href="/terms" class="btn-secondary px-12">"Read Terms"</a>
                    </div>
                </div>
            </section>
        </div>

        // Schema.org FAQ markup for SEO
        <Script type_="application/ld+json">
        {r#"{
          "@context": "https://schema.org",
          "@type": "FAQPage",
          "mainEntity": [{
            "@type": "Question",
            "name": "What is included in the £329 development fee?",
            "acceptedAnswer": {
              "@type": "Answer",
              "text": "The £329 one-time fee includes: complete website design and coding, cross-browser compatibility, domain registration and management for 12 months, initial SEO setup with meta tags and keyword integration, SSL certificate setup, and 1 month FREE of all managed services."
            }
          }, {
            "@type": "Question",
            "name": "How long does website development take?",
            "acceptedAnswer": {
              "@type": "Answer",
              "text": "We deliver an Initial Version within 36 hours after you approve the Final Specification Sheet. Total typical timeline: 3-5 days from specification to launch."
            }
          }, {
            "@type": "Question",
            "name": "What is SSL and why do I need it?",
            "acceptedAnswer": {
              "@type": "Answer",
              "text": "SSL encrypts data transmitted between your website and visitors. It's essential for customer trust, SEO rankings, and protecting sensitive information. We manage your SSL certificate and renewal."
            }
          }, {
            "@type": "Question",
            "name": "What's included in the £30/month service?",
            "acceptedAnswer": {
              "@type": "Answer",
              "text": "Your monthly subscription includes: high-speed hosting, up to 2 hours of development time, firewalls and virus protection, SSL certificate management, weekly automated backups, and 24/7 uptime monitoring."
            }
          }, {
            "@type": "Question",
            "name": "Who owns my website?",
            "acceptedAnswer": {
              "@type": "Answer",
              "text": "You own your domain name, branding, and content. We retain ownership of the code but grant you non-exclusive, perpetual rights to use, modify, and display your website."
            }
          }]
        }"#}
        </Script>
    }
}
