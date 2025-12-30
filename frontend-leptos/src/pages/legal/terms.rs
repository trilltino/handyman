//! Terms and Conditions page component.
//!
//! Legal terms and conditions for website development services.

use crate::components::seo::SeoHead;
use leptos::prelude::*;
use shared::PageMetadata;

#[component]
pub fn Terms() -> impl IntoView {
    let (expanded_section, set_expanded_section) = signal(Option::<usize>::None);
    let expanded_signal = Signal::derive(move || expanded_section.get());

    let toggle_section = move |index: usize| {
        set_expanded_section.update(|current| {
            *current = if *current == Some(index) {
                None
            } else {
                Some(index)
            };
        });
    };

    view! {
        <SeoHead metadata=PageMetadata {
            title: "Terms and Conditions | XFSOLUTIONS Website Development".to_string(),
            description: "Website development terms and conditions for XFSOLUTIONS services. Transparent legal terms for our £329 setup and £30/month managed services.".to_string(),
            canonical_url: Some("https://xftradesman.com/terms".to_string()),
            og_image: None,
        }/>

        <div class="min-h-screen bg-void text-white">
            // Hero Section
            <section class="bg-void relative border-b border-void-highlight py-20 px-4">
                <div class="absolute inset-0 bg-cyber-grid bg-[length:40px_40px] opacity-20"></div>

                <div class="max-w-4xl mx-auto relative z-10">
                    <span class="text-brand font-mono text-sm tracking-widest uppercase mb-4 block">"Legal Documentation"</span>
                    <h1 class="text-4xl md:text-6xl font-heading font-black mb-6 tracking-tighter">"TERMS & CONDITIONS"</h1>
                    <p class="text-xl text-gray-400 mb-8">"Website Development Service Agreement between XFSOLUTIONS and Client"</p>

                    // PDF Download Button
                    <a
                        href="https://drive.google.com/uc?export=download&id=1ikW8JRqM2r5p_mvsnfeKlLqyoMDRtExU"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="inline-flex items-center gap-3 px-6 py-3 bg-brand hover:bg-brand-dark text-void font-bold rounded-lg transition-all transform hover:scale-105 shadow-lg"
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                        </svg>
                        "Download Full Contract (PDF)"
                    </a>
                </div>
            </section>

            // Main Content
            <section class="py-16 px-4">
                <div class="max-w-4xl mx-auto space-y-4">

                    // Service Overview
                    <AccordionSection
                        title="Service Overview"
                        index=0
                        expanded_section=expanded_signal
                        toggle_section=toggle_section
                    >
                        <div class="space-y-4 text-gray-300">
                            <h3 class="text-xl font-bold text-white">"Services Provided"</h3>
                            <ul class="list-disc list-inside space-y-2 ml-4">
                                <li>"Business Website Creation and Development Services"</li>
                                <li>"Registration and management of your business domain name for the first 12 months"</li>
                                <li>"Complete coding and development of your website with out-of-the-box optimization"</li>
                                <li>"Cross-browser compatibility (Chrome, Safari, Edge, Firefox)"</li>
                                <li>"Initial SEO setup including meta tags and keyword integration"</li>
                            </ul>

                            <h3 class="text-xl font-bold text-white mt-6">"Pricing"</h3>
                            <div class="bg-void-surface border border-void-highlight rounded-lg p-6">
                                <div class="space-y-3">
                                    <div class="flex justify-between items-center">
                                        <span class="text-gray-400">"Initial Setup Fee:"</span>
                                        <span class="text-2xl font-bold text-brand">"£329"</span>
                                    </div>
                                    <div class="flex justify-between items-center">
                                        <span class="text-gray-400">"Monthly Managed Services:"</span>
                                        <span class="text-2xl font-bold text-brand">"£30/month"</span>
                                    </div>
                                    <div class="pt-3 border-t border-void-highlight">
                                        <p class="text-sm text-gray-400">"✓ 1 Month Free trial of managed services"</p>
                                        <p class="text-sm text-gray-400">"✓ 75% refund available during startup period"</p>
                                    </div>
                                </div>
                            </div>

                            <h3 class="text-xl font-bold text-white mt-6">"Monthly Managed Services (£30/month)"</h3>
                            <ul class="list-disc list-inside space-y-2 ml-4">
                                <li>"High-speed storage on professional Internet server"</li>
                                <li>"Up to 2 hours per month of expert development time"</li>
                                <li>"Industry-standard firewalls and virus protection"</li>
                                <li>"Full SSL Certificate management and renewal"</li>
                                <li>"Weekly automated backups with 30-day retention"</li>
                                <li>"24/7 Uptime monitoring with intrusion detection"</li>
                            </ul>
                        </div>
                    </AccordionSection>

                    // Website Development Process
                    <AccordionSection
                        title="Website Development Process"
                        index=1
                        expanded_section=expanded_signal
                        toggle_section=toggle_section
                    >
                        <div class="space-y-4 text-gray-300">
                            <h3 class="text-xl font-bold text-white">"Design Phase"</h3>
                            <p>"Client provides a Preliminary Specification Sheet outlining desired web pages, graphics, and functionality. Company will review and create a Modified Specification Sheet with improvements. Both parties will iterate until a Final Specification Sheet is mutually signed."</p>

                            <h3 class="text-xl font-bold text-white mt-6">"Development Timeline"</h3>
                            <div class="space-y-3">
                                <div class="bg-void-surface border-l-4 border-brand p-4 rounded">
                                    <p class="font-bold text-white">"Initial Version"</p>
                                    <p class="text-sm text-gray-400">"Delivered within 72 HRS after Final Specification Sheet agreement"</p>
                                </div>
                                <div class="bg-void-surface border-l-4 border-brand p-4 rounded">
                                    <p class="font-bold text-white">"Host Version"</p>
                                    <p class="text-sm text-gray-400">"Deployed within 24 hours after Initial Version acceptance"</p>
                                </div>
                                <div class="bg-void-surface border-l-4 border-brand p-4 rounded">
                                    <p class="font-bold text-white">"Final Version"</p>
                                    <p class="text-sm text-gray-400">"Posted to live server after Host Version acceptance"</p>
                                </div>
                            </div>

                            <h3 class="text-xl font-bold text-white mt-6">"Post-Launch Modifications"</h3>
                            <p>"All modifications are permissible after the 1st month. Bug fixes and conformance with Final Specification Sheet are provided at no additional charge. Enhancement modifications are included in the monthly managed services."</p>
                        </div>
                    </AccordionSection>

                    // Intellectual Property
                    <AccordionSection
                        title="Intellectual Property & Copyright"
                        index=2
                        expanded_section=expanded_signal
                        toggle_section=toggle_section
                    >
                        <div class="space-y-4 text-gray-300">
                            <h3 class="text-xl font-bold text-white">"Company's Rights"</h3>
                            <p>"Company is the sole author and owner of all CGI, HTML Code, graphics, and data incorporated into the Website. Company grants Client non-exclusive, perpetual rights to reproduce, create derivative works, and publicly display the Website."</p>

                            <h3 class="text-xl font-bold text-white mt-6">"Client's Rights"</h3>
                            <p>"Client owns their Domain Name, uniform resource locator, and any graphics or data they provided. Client receives non-exclusive rights to use and modify the Website but cannot sublicense the underlying code."</p>
                        </div>
                    </AccordionSection>

                    // Warranties
                    <AccordionSection
                        title="Warranties & Guarantees"
                        index=3
                        expanded_section=expanded_signal
                        toggle_section=toggle_section
                    >
                        <div class="space-y-4 text-gray-300">
                            <div class="bg-green-900/20 border border-green-500/50 rounded-lg p-4">
                                <h4 class="font-bold text-green-400 mb-2">"✓ Company Warranties"</h4>
                                <ul class="list-disc list-inside space-y-1 ml-4 text-sm">
                                    <li>"Company is the sole creator of Website design"</li>
                                    <li>"Website will function with major browsers (Chrome, Safari, Edge, Firefox)"</li>
                                    <li>"Work completed in good faith and workmanlike manner"</li>
                                </ul>
                            </div>
                        </div>
                    </AccordionSection>

                    // Disclaimers
                    <AccordionSection
                        title="Disclaimers & Limitations"
                        index=4
                        expanded_section=expanded_signal
                        toggle_section=toggle_section
                    >
                        <div class="space-y-4 text-gray-300">
                            <div class="bg-yellow-900/20 border border-yellow-500/50 rounded-lg p-4">
                                <p class="text-sm">"Services provided 'AS IS' without warranty of any kind. Company is not liable for loss of profits, business interruption, or consequential damages. Some jurisdictions do not permit limitation of consequential damages."</p>
                            </div>

                            <h3 class="text-xl font-bold text-white mt-6">"Force Majeure"</h3>
                            <p class="text-sm">"Company not liable for delays due to: Acts of God, government actions, natural disasters, epidemics, labor disputes, supplier delays, or equipment failure."</p>

                            <h3 class="text-xl font-bold text-white mt-6">"Third Party Transactions"</h3>
                            <p class="text-sm">"Company does not endorse third-party services accessed through the Internet. Client assumes all risk for transactions with third parties."</p>
                        </div>
                    </AccordionSection>

                    // Termination
                    <AccordionSection
                        title="Termination & Refund Policy"
                        index=5
                        expanded_section=expanded_signal
                        toggle_section=toggle_section
                    >
                        <div class="space-y-4 text-gray-300">
                            <h3 class="text-xl font-bold text-white">"Termination by Company"</h3>
                            <p>"Company may terminate services with 30 days' notice. If terminated before Final Specification Sheet delivery, full refund provided. If terminated after specification but before final acceptance, pro-rata refund applies."</p>

                            <h3 class="text-xl font-bold text-white mt-6">"Termination for Cause"</h3>
                            <p>"Company may immediately cancel if Client fails to fulfill material obligations. No refund provided under these circumstances."</p>

                            <h3 class="text-xl font-bold text-white mt-6">"Refund Policy"</h3>
                            <div class="bg-void-surface border border-void-highlight rounded-lg p-4">
                                <p class="text-sm">"75% of the £329 setup fee is refundable during the website startup period (before final acceptance). Refunds are at Company's sole discretion on a case-by-case basis."</p>
                            </div>
                        </div>
                    </AccordionSection>

                    // Confidentiality
                    <AccordionSection
                        title="Confidentiality & Data Security"
                        index=6
                        expanded_section=expanded_signal
                        toggle_section=toggle_section
                    >
                        <div class="space-y-4 text-gray-300">
                            <p>"Both parties agree to keep Confidential Information private during the agreement term and for 2 years following termination. Information is only shared with employees who need access to perform services."</p>

                            <h3 class="text-xl font-bold text-white mt-6">"Security Measures"</h3>
                            <ul class="list-disc list-inside space-y-2 ml-4 text-sm">
                                <li>"User identification and access controls"</li>
                                <li>"Industry standard firewalls"</li>
                                <li>"Virus protection programs"</li>
                                <li>"Intrusion detection systems"</li>
                                <li>"Industry standard encryption for Internet transmission"</li>
                            </ul>

                            <h3 class="text-xl font-bold text-white mt-6">"Non-Solicitation"</h3>
                            <p class="text-sm">"Neither party shall solicit or employ the other party's employees during the agreement term and for 1 year after termination."</p>
                        </div>
                    </AccordionSection>

                    // Dispute Resolution
                    <AccordionSection
                        title="Dispute Resolution & Governing Law"
                        index=7
                        expanded_section=expanded_signal
                        toggle_section=toggle_section
                    >
                        <div class="space-y-4 text-gray-300">
                            <h3 class="text-xl font-bold text-white">"Dispute Resolution Process"</h3>
                            <div class="space-y-3">
                                <div class="flex items-start gap-3">
                                    <div class="w-8 h-8 rounded-full bg-brand/10 flex items-center justify-center text-brand flex-shrink-0 font-bold">1</div>
                                    <div>
                                        <p class="font-bold text-white">"Good-Faith Negotiation"</p>
                                        <p class="text-sm text-gray-400">"30 days to resolve through direct negotiation"</p>
                                    </div>
                                </div>
                                <div class="flex items-start gap-3">
                                    <div class="w-8 h-8 rounded-full bg-brand/10 flex items-center justify-center text-brand flex-shrink-0 font-bold">2</div>
                                    <div>
                                        <p class="font-bold text-white">"Mediation (CEDR)"</p>
                                        <p class="text-sm text-gray-400">"Centre for Effective Dispute Resolution Model Mediation Procedure"</p>
                                    </div>
                                </div>
                                <div class="flex items-start gap-3">
                                    <div class="w-8 h-8 rounded-full bg-brand/10 flex items-center justify-center text-brand flex-shrink-0 font-bold">3</div>
                                    <div>
                                        <p class="font-bold text-white">"Legal Action"</p>
                                        <p class="text-sm text-gray-400">"English Courts (Small Claims Track preferred for applicable amounts)"</p>
                                    </div>
                                </div>
                            </div>

                            <h3 class="text-xl font-bold text-white mt-6">"Governing Law"</h3>
                            <p>"This Agreement is governed by and construed in accordance with the law of England and Wales."</p>

                            <h3 class="text-xl font-bold text-white mt-6">"Jurisdiction"</h3>
                            <p>"Courts of England and Wales have exclusive jurisdiction for any disputes."</p>
                        </div>
                    </AccordionSection>

                    // General Provisions
                    <AccordionSection
                        title="General Provisions"
                        index=8
                        expanded_section=expanded_signal
                        toggle_section=toggle_section
                    >
                        <div class="space-y-4 text-gray-300">
                            <h3 class="text-xl font-bold text-white">"Entire Agreement"</h3>
                            <p class="text-sm">"This Agreement supersedes all prior agreements and contains all covenants between the parties."</p>

                            <h3 class="text-xl font-bold text-white mt-4">"Assignment"</h3>
                            <p class="text-sm">"Neither party may assign this Agreement without prior written consent."</p>

                            <h3 class="text-xl font-bold text-white mt-4">"Independent Contractor"</h3>
                            <p class="text-sm">"Company is an independent contractor, not an employee, partner, or joint venturer of Client."</p>

                            <h3 class="text-xl font-bold text-white mt-4">"Insurance"</h3>
                            <p class="text-sm">"Client agrees to obtain necessary insurance coverage during the agreement term."</p>

                            <h3 class="text-xl font-bold text-white mt-4">"Notices"</h3>
                            <p class="text-sm">"All notices must be in writing. Electronic delivery via email is agreed upon."</p>

                            <h3 class="text-xl font-bold text-white mt-4">"Severability"</h3>
                            <p class="text-sm">"If any provision is invalid, remaining provisions continue in full force."</p>
                        </div>
                    </AccordionSection>

                </div>
            </section>

            // Contact Section
            <section class="bg-void-surface border-t border-void-highlight py-16 px-4">
                <div class="max-w-2xl mx-auto text-center">
                    <h2 class="text-3xl font-black text-white mb-4">"QUESTIONS ABOUT OUR TERMS?"</h2>
                    <p class="text-gray-400 mb-8">"We're here to help. Contact us for clarification on any of these terms."</p>
                    <div class="flex flex-col sm:flex-row gap-4 justify-center">
                        <a href="/contact" class="btn-primary">"Contact Us"</a>
                        <a href="/pricing" class="btn-secondary">"View Pricing"</a>
                    </div>
                </div>
            </section>
        </div>
    }
}

#[component]
fn AccordionSection(
    title: &'static str,
    index: usize,
    expanded_section: Signal<Option<usize>>,
    toggle_section: impl Fn(usize) + 'static + Copy,
    children: Children,
) -> impl IntoView {
    let is_expanded = move || expanded_section.get() == Some(index);

    view! {
        <div class="border border-void-highlight rounded-lg overflow-hidden bg-void-surface transition-all hover:border-brand/50">
            <button
                on:click=move |_| toggle_section(index)
                class="w-full px-6 py-4 flex items-center justify-between bg-void-highlight/20 hover:bg-void-highlight/40 transition-colors"
            >
                <h2 class="text-xl font-bold text-white text-left">{title}</h2>
                <svg
                    class="w-6 h-6 text-brand transition-transform"
                    class:rotate-180=is_expanded
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                </svg>
            </button>

            <div
                class="transition-all duration-300 ease-in-out overflow-hidden"
                class:max-h-0=move || !is_expanded()
                class:max-h-screen=is_expanded
            >
                <div class="p-6">
                    {children()}
                </div>
            </div>
        </div>
    }
}
