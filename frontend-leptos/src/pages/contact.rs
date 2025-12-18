//! Contact form page component.
//!
//! Contact form with validation and API submission.

use leptos::prelude::*;
use leptos::html::{Input, Textarea};
use leptos::task::spawn_local;
use leptos::ev::SubmitEvent;
use shared::PageMetadata;
use crate::components::seo::SeoHead;

#[component]
pub fn Contact() -> impl IntoView {
    let name_ref: NodeRef<Input> = NodeRef::new();
    let email_ref: NodeRef<Input> = NodeRef::new();
    let message_ref: NodeRef<Textarea> = NodeRef::new();
    let (sending, set_sending) = signal(false);
    let (success_msg, set_success_msg) = signal(Option::<String>::None);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);

    let submit_form = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_sending.set(true);
        set_success_msg.set(None);
        set_error_msg.set(None);

        let n = name_ref.get().map(|el| el.value()).unwrap_or_else(String::new);
        let e = email_ref.get().map(|el| el.value()).unwrap_or_else(String::new);
        let m = message_ref.get().map(|el| el.value()).unwrap_or_else(String::new);

        spawn_local(async move {
            match crate::api::contact::submit_contact_form(n, e, m).await {
                Ok(msg) => {
                    set_success_msg.set(Some(msg));
                    if let Some(el) = name_ref.get() { 
                        el.set_value(""); 
                    }
                    if let Some(el) = email_ref.get() { 
                        el.set_value(""); 
                    }
                    if let Some(el) = message_ref.get() { 
                        el.set_value(""); 
                    }
                },
                Err(err) => {
                    set_error_msg.set(Some(err));
                }
            }
            set_sending.set(false);
        });
    };

    view! {
        <SeoHead metadata=PageMetadata {
            title: "Contact Us - Free Handyman Website Consultation | XF Tradesmen".to_string(),
            description: "Questions about our handyman website builder? Get a free consultation. Our team responds within 24 hours.".to_string(),
            canonical_url: Some("https://xftradesmen.com/contact".to_string()),
            og_image: None,
        }/>


        <div class="space-y-0 overflow-x-hidden">
             // Hero
             <section class="bg-void relative border-b border-void-highlight text-white py-32 px-4 overflow-hidden">
                <div class="absolute inset-0 bg-cyber-grid bg-[length:40px_40px] opacity-20"></div>
                <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[800px] h-[800px] bg-brand/5 blur-[120px] rounded-full pointer-events-none"></div>

                <div class="max-w-6xl mx-auto text-center relative z-10 animate-fade-in">
                    <span class="text-brand font-mono text-sm tracking-widest uppercase mb-4 block">"Communication Uplink"</span>
                    <h1 class="text-5xl md:text-7xl font-heading font-black mb-6 tracking-tighter">"INITIATE" <span class="text-brand">"CONTACT"</span></h1>
                     <p class="text-xl text-gray-400 max-w-2xl mx-auto font-light">
                        "Deploying a professional website changes everything. Let's start the process."
                    </p>
                </div>
            </section>

            <section class="bg-void-surface border-t border-void-highlight py-24 px-4 relative">
                 <div class="absolute inset-0 bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-void-highlight/10 via-transparent to-transparent"></div>
                
                <div class="max-w-2xl mx-auto relative z-10">
                    <div class="card-deep relative overflow-hidden">
                        // Glow
                         <div class="absolute top-0 right-0 w-64 h-64 bg-brand/5 blur-[80px] rounded-full pointer-events-none"></div>

                        <h2 class="text-3xl font-bold text-white mb-8 text-center font-heading">"Send Transmission"</h2>
                        
                        <form on:submit=submit_form class="space-y-6">
                            {move || success_msg.get().map(|msg| {
                                view! {
                                    <div class="bg-green-900/20 border border-green-500/30 text-green-300 p-4 rounded-lg flex items-center gap-3 animate-fade-in">
                                        <div class="w-8 h-8 rounded-full bg-green-500/20 flex items-center justify-center flex-shrink-0">
                                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                        </div>
                                        {msg}
                                    </div>
                                }
                            })}

                            {move || error_msg.get().map(|msg| {
                                view! {
                                    <div class="bg-brand-deep/20 border border-brand/30 text-brand-light p-4 rounded-lg flex items-center gap-3 animate-fade-in">
                                         <div class="w-8 h-8 rounded-full bg-brand/20 flex items-center justify-center flex-shrink-0">
                                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
                                        </div>
                                        {msg}
                                    </div>
                                }
                            })}

                            <div class="group">
                                <label class="block text-xs font-mono font-semibold text-gray-400 mb-2 uppercase tracking-wider group-focus-within:text-brand transition-colors">"Identity // Name"</label>
                                <input
                                    type="text"
                                    class="input-base"
                                    required
                                    placeholder="Execute Name Entry..."
                                    node_ref=name_ref
                                    disabled=move || sending.get()
                                />
                            </div>

                            <div class="group">
                                <label class="block text-xs font-mono font-semibold text-gray-400 mb-2 uppercase tracking-wider group-focus-within:text-brand transition-colors">"Frequency // Email"</label>
                                <input
                                    type="email"
                                    class="input-base"
                                    required
                                    placeholder="Execute Email Entry..."
                                    node_ref=email_ref
                                    disabled=move || sending.get()
                                />
                            </div>

                            <div class="group">
                                <label class="block text-xs font-mono font-semibold text-gray-400 mb-2 uppercase tracking-wider group-focus-within:text-brand transition-colors">"Packet Data // Message"</label>
                                <textarea
                                    class="input-base h-32 resize-none"
                                    required
                                    placeholder="Enter transmission data..."
                                    node_ref=message_ref
                                    disabled=move || sending.get()
                                ></textarea>
                            </div>

                            <div class="pt-4">
                                <button
                                    type="submit"
                                    class="w-full btn-primary disabled:opacity-50 disabled:cursor-not-allowed group"
                                    disabled=move || sending.get()
                                >
                                    <span class="flex items-center justify-center gap-2">
                                        {move || if sending.get() { "TRANSMITTING..." } else { "SEND SIGNAL" }}
                                        <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"/></svg>
                                    </span>
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
            </section>
        </div>
    }
}
