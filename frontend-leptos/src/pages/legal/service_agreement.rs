//! Service Agreement page component.
//!
//! Client-friendly summary of service agreement.

use crate::components::seo::SeoHead;
use leptos::prelude::*;
use shared::PageMetadata;

#[component]
pub fn ServiceAgreement() -> impl IntoView {
    view! {
        <SeoHead metadata=PageMetadata {
            title: "Service Agreement Summary | XFSOLUTIONS Website Development".to_string(),
            description: "Partnership overview for our website development services. Clear explanation of what we provide, your investment, and our development process.".to_string(),
            canonical_url: Some("https://xftradesman.com/service-agreement".to_string()),
            og_image: None,
        }/>

        <div class="min-h-screen bg-void text-white">
            // Hero Section
            <section class="bg-void relative border-b border-void-highlight py-20 px-4">
                <div class="absolute inset-0 bg-cyber-grid bg-[length:40px_40px] opacity-20"></div>

                <div class="max-w-4xl mx-auto text-center relative z-10">
                    <span class="text-brand font-mono text-sm tracking-widest uppercase mb-4 block">"Partnership Overview"</span>
                    <h1 class="text-4xl md:text-6xl font-heading font-black mb-6 tracking-tighter">"SERVICE AGREEMENT" <span class="text-brand">"SUMMARY"</span></h1>
                    <p class="text-xl text-gray-400">"Your partnership with XFSOLUTIONS for professional website development"</p>
                </div>
            </section>

            // What We Provide
            <section class="py-16 px-4 bg-void-surface">
                <div class="max-w-6xl mx-auto">
                    <div class="mb-12">
                        <h2 class="text-3xl font-bold text-white mb-2 font-heading">"What We Provide"</h2>
                        <div class="h-1 w-20 bg-brand rounded-full"></div>
                    </div>

                    <div class="grid md:grid-cols-2 gap-6">
                        <div class="card-deep">
                            <div class="flex items-start gap-4">
                                <div class="w-12 h-12 rounded-lg bg-brand/10 flex items-center justify-center text-brand flex-shrink-0">
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>
                                    </svg>
                                </div>
                                <div>
                                    <h3 class="text-xl font-bold text-white mb-2">"Complete Website Development"</h3>
                                    <p class="text-gray-400 text-sm">"Professional design, complete coding, cross-browser compatibility, and responsive mobile design."</p>
                                </div>
                            </div>
                        </div>

                        <div class="card-deep">
                            <div class="flex items-start gap-4">
                                <div class="w-12 h-12 rounded-lg bg-brand/10 flex items-center justify-center text-brand flex-shrink-0">
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"/>
                                    </svg>
                                </div>
                                <div>
                                    <h3 class="text-xl font-bold text-white mb-2">"Domain Registration & Management"</h3>
                                    <p class="text-gray-400 text-sm">"Your business domain registered and managed for 12 months, including renewal reminders."</p>
                                </div>
                            </div>
                        </div>

                        <div class="card-deep">
                            <div class="flex items-start gap-4">
                                <div class="w-12 h-12 rounded-lg bg-brand/10 flex items-center justify-center text-brand flex-shrink-0">
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01"/>
                                    </svg>
                                </div>
                                <div>
                                    <h3 class="text-xl font-bold text-white mb-2">"Professional Hosting"</h3>
                                    <p class="text-gray-400 text-sm">"High-speed storage on professional servers designed for fast loading and low latency."</p>
                                </div>
                            </div>
                        </div>

                        <div class="card-deep">
                            <div class="flex items-start gap-4">
                                <div class="w-12 h-12 rounded-lg bg-brand/10 flex items-center justify-center text-brand flex-shrink-0">
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 5.636l-3.536 3.536m0 5.656l3.536 3.536M9.172 9.172L5.636 5.636m3.536 9.192l-3.536 3.536M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-5 0a4 4 0 11-8 0 4 4 0 018 0z"/>
                                    </svg>
                                </div>
                                <div>
                                    <h3 class="text-xl font-bold text-white mb-2">"Ongoing Technical Support"</h3>
                                    <p class="text-gray-400 text-sm">"Expert support with 2 hours monthly development time for updates and improvements."</p>
                                </div>
                            </div>
                        </div>

                        <div class="card-deep">
                            <div class="flex items-start gap-4">
                                <div class="w-12 h-12 rounded-lg bg-brand/10 flex items-center justify-center text-brand flex-shrink-0">
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                                    </svg>
                                </div>
                                <div>
                                    <h3 class="text-xl font-bold text-white mb-2">"Security & Maintenance"</h3>
                                    <p class="text-gray-400 text-sm">"Firewalls, virus protection, SSL certificates, weekly backups, and 24/7 uptime monitoring."</p>
                                </div>
                            </div>
                        </div>

                        <div class="card-deep">
                            <div class="flex items-start gap-4">
                                <div class="w-12 h-12 rounded-lg bg-brand/10 flex items-center justify-center text-brand flex-shrink-0">
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                                    </svg>
                                </div>
                                <div>
                                    <h3 class="text-xl font-bold text-white mb-2">"SEO Optimization"</h3>
                                    <p class="text-gray-400 text-sm">"Initial SEO setup with meta tags and keyword integration to help customers find you on Google."</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Development Process
            <section class="py-16 px-4 bg-void">
                <div class="max-w-6xl mx-auto">
                    <div class="mb-12">
                        <h2 class="text-3xl font-bold text-white mb-2 font-heading">"Our Development Process"</h2>
                        <div class="h-1 w-20 bg-brand rounded-full"></div>
                    </div>

                    <div class="relative">
                        // Timeline connector line
                        <div class="absolute left-8 top-0 bottom-0 w-0.5 bg-brand/30 hidden md:block"></div>

                        <div class="space-y-8">
                            <ProcessStep
                                number="1"
                                title="Specification Sheet Creation"
                                description="You provide your vision. We create a detailed specification showing your desired pages, graphics, and functionality. We iterate together until you're 100% satisfied."
                            />
                            <ProcessStep
                                number="2"
                                title="Design Approval"
                                description="We refine the specification with suggested improvements. You review and approve. Once both parties sign the Final Specification Sheet, development begins."
                            />
                            <ProcessStep
                                number="3"
                                title="Initial Version (72 HRS)"
                                description="We build your website. You test it at our facilities. You can suggest modifications, which we incorporate at no extra charge."
                            />
                            <ProcessStep
                                number="4"
                                title="Host Version (24 hours)"
                                description="We deploy your site to our live Internet server. You test it over the web. Final adjustments are made based on your feedback."
                            />
                            <ProcessStep
                                number="5"
                                title="Launch & Ongoing Support"
                                description="Your website goes live! We provide training on managing your site. Your monthly managed services begin, keeping your site secure and up-to-date."
                            />
                        </div>
                    </div>
                </div>
            </section>

            // Your Investment
            <section class="py-16 px-4 bg-void-surface">
                <div class="max-w-4xl mx-auto">
                    <div class="mb-12">
                        <h2 class="text-3xl font-bold text-white mb-2 font-heading">"Your Investment"</h2>
                        <div class="h-1 w-20 bg-brand rounded-full"></div>
                    </div>

                    <div class="grid md:grid-cols-2 gap-6">
                        // Initial Setup
                        <div class="relative group">
                            <div class="absolute -inset-1 bg-gradient-to-r from-brand to-brand-dark rounded-2xl blur opacity-25 group-hover:opacity-50 transition duration-500"></div>
                            <div class="relative bg-void border border-void-highlight rounded-2xl p-8">
                                <div class="text-center">
                                    <p class="text-gray-400 mb-2">"One-Time Setup"</p>
                                    <p class="text-5xl font-black text-white mb-4">"£329"</p>
                                    <div class="h-px bg-void-highlight mb-6"></div>
                                    <ul class="text-left space-y-2 text-gray-300 text-sm">
                                        <li class="flex items-center gap-2">
                                            <svg class="w-4 h-4 text-brand flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                            "Complete website design & coding"
                                        </li>
                                        <li class="flex items-center gap-2">
                                            <svg class="w-4 h-4 text-brand flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                            "Domain registration (12 months)"
                                        </li>
                                        <li class="flex items-center gap-2">
                                            <svg class="w-4 h-4 text-brand flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                            "Initial SEO setup"
                                        </li>
                                        <li class="flex items-center gap-2">
                                            <svg class="w-4 h-4 text-brand flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                            "SSL certificate setup"
                                        </li>
                                        <li class="flex items-center gap-2">
                                            <svg class="w-4 h-4 text-brand flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                            "75% refundable during startup"
                                        </li>
                                    </ul>
                                </div>
                            </div>
                        </div>

                        // Monthly Service
                        <div class="relative group">
                            <div class="absolute -inset-1 bg-gradient-to-r from-brand to-brand-dark rounded-2xl blur opacity-25 group-hover:opacity-50 transition duration-500"></div>
                            <div class="relative bg-void border border-void-highlight rounded-2xl p-8">
                                <div class="text-center">
                                    <p class="text-gray-400 mb-2">"Monthly Managed Services"</p>
                                    <p class="text-5xl font-black text-white mb-1">"£30"</p>
                                    <p class="text-brand text-sm font-mono mb-4">"1st MONTH FREE"</p>
                                    <div class="h-px bg-void-highlight mb-6"></div>
                                    <ul class="text-left space-y-2 text-gray-300 text-sm">
                                        <li class="flex items-center gap-2">
                                            <svg class="w-4 h-4 text-brand flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                            "Professional hosting"
                                        </li>
                                        <li class="flex items-center gap-2">
                                            <svg class="w-4 h-4 text-brand flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                            "2 hours development time"
                                        </li>
                                        <li class="flex items-center gap-2">
                                            <svg class="w-4 h-4 text-brand flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                            "Security & SSL management"
                                        </li>
                                        <li class="flex items-center gap-2">
                                            <svg class="w-4 h-4 text-brand flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                            "Weekly backups"
                                        </li>
                                        <li class="flex items-center gap-2">
                                            <svg class="w-4 h-4 text-brand flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                            "24/7 uptime monitoring"
                                        </li>
                                    </ul>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Intellectual Property
            <section class="py-16 px-4 bg-void">
                <div class="max-w-4xl mx-auto">
                    <div class="mb-12">
                        <h2 class="text-3xl font-bold text-white mb-2 font-heading">"Intellectual Property Rights"</h2>
                        <div class="h-1 w-20 bg-brand rounded-full"></div>
                    </div>

                    <div class="grid md:grid-cols-2 gap-8">
                        <div class="bg-void-surface border border-void-highlight rounded-lg p-6">
                            <h3 class="text-xl font-bold text-white mb-4 flex items-center gap-2">
                                <svg class="w-5 h-5 text-brand" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                                </svg>
                                "You Own"
                            </h3>
                            <ul class="space-y-2 text-gray-300">
                                <li class="flex items-start gap-2">
                                    <span class="text-brand mt-1">"•"</span>
                                    "Your domain name and URL"
                                </li>
                                <li class="flex items-start gap-2">
                                    <span class="text-brand mt-1">"•"</span>
                                    "Your content and branding"
                                </li>
                                <li class="flex items-start gap-2">
                                    <span class="text-brand mt-1">"•"</span>
                                    "Graphics and data you provide"
                                </li>
                            </ul>
                        </div>

                        <div class="bg-void-surface border border-void-highlight rounded-lg p-6">
                            <h3 class="text-xl font-bold text-white mb-4 flex items-center gap-2">
                                <svg class="w-5 h-5 text-brand" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"/>
                                </svg>
                                "You Can"
                            </h3>
                            <ul class="space-y-2 text-gray-300">
                                <li class="flex items-start gap-2">
                                    <span class="text-brand mt-1">"•"</span>
                                    "Use and display your website"
                                </li>
                                <li class="flex items-start gap-2">
                                    <span class="text-brand mt-1">"•"</span>
                                    "Modify with our help or on your own"
                                </li>
                                <li class="flex items-start gap-2">
                                    <span class="text-brand mt-1">"•"</span>
                                    "Create derivative works"
                                </li>
                            </ul>
                        </div>
                    </div>

                    <div class="mt-6 bg-blue-900/20 border border-blue-500/50 rounded-lg p-4">
                        <p class="text-sm text-gray-300">
                            <span class="font-bold text-blue-400">"Note: "</span>
                            "We retain ownership of the underlying code and design framework but grant you non-exclusive, perpetual rights to use it. You cannot sublicense the code to third parties, but you have full control over your website."
                        </p>
                    </div>
                </div>
            </section>

            // CTA
            <section class="bg-void-surface border-t border-void-highlight py-20 px-4">
                <div class="max-w-2xl mx-auto text-center">
                    <h2 class="text-3xl md:text-4xl font-black text-white mb-6 uppercase tracking-tight">"Ready to Get Started?"</h2>
                    <p class="text-gray-400 mb-8 text-lg">"Launch your professional website with a trusted partner."</p>
                    <div class="flex flex-col sm:flex-row gap-4 justify-center">
                        <a href="/pricing" class="btn-primary">"View Pricing"</a>
                        <a href="/terms" class="btn-secondary">"Read Full Terms"</a>
                    </div>

                    <div class="mt-12 pt-8 border-t border-void-highlight">
                        <a
                            href="https://drive.google.com/uc?export=download&id=1ikW8JRqM2r5p_mvsnfeKlLqyoMDRtExU"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="inline-flex items-center gap-2 text-brand hover:text-brand-dark transition-colors"
                        >
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                            </svg>
                            "Download Full Contract (PDF)"
                        </a>
                    </div>
                </div>
            </section>
        </div>
    }
}

#[component]
fn ProcessStep(
    number: &'static str,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="relative flex items-start gap-6 md:pl-20">
            // Number circle
            <div class="absolute left-0 md:left-6 w-16 h-16 rounded-full bg-brand/10 border-4 border-brand flex items-center justify-center z-10">
                <span class="text-2xl font-black text-brand">{number}</span>
            </div>

            // Content
            <div class="ml-20 md:ml-0 flex-1">
                <div class="bg-void-surface border border-void-highlight rounded-lg p-6 hover:border-brand/50 transition-colors">
                    <h3 class="text-xl font-bold text-white mb-2">{title}</h3>
                    <p class="text-gray-400 text-sm">{description}</p>
                </div>
            </div>
        </div>
    }
}
