//! Contact form page component.
//!
//! Contact form with validation and API submission.

use crate::components::seo::SeoHead;
use leptos::ev::SubmitEvent;
use leptos::html::{Input, Textarea};
use leptos::prelude::*;
use leptos::task::spawn_local;
use shared::PageMetadata;

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

        let n = name_ref.get().map(|el| el.value()).unwrap_or_default();
        let e = email_ref.get().map(|el| el.value()).unwrap_or_default();
        let m = message_ref.get().map(|el| el.value()).unwrap_or_default();

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
                }
                Err(err) => {
                    set_error_msg.set(Some(err));
                }
            }
            set_sending.set(false);
        });
    };

    view! {
        <SeoHead metadata=PageMetadata {
            title: "Contact XF Tradesmen | Start Your Digital Growth Today".to_string(),
            description: "Ready to get more leads? Contact our team for a free consultation. We build high-performance websites for UK tradespeople.".to_string(),
            canonical_url: Some("https://xftradesman.com/contact".to_string()),
            og_image: None,
        }/>


            <section class="bg-black border-t border-white/10 py-24 px-6 relative font-sans min-h-screen flex items-center">
                 <div class="absolute inset-0 bg-[radial-gradient(circle_at_center,_var(--tw-gradient-stops))] from-brand/10 via-black to-black opacity-50"></div>

                <div class="max-w-7xl mx-auto relative z-10 w-full">

                    <div class="mb-16 text-center">
                         <h1 class="text-[5rem] md:text-[9rem] lg:text-[11rem] font-black tracking-tighter mb-4 leading-[0.85] uppercase text-transparent bg-clip-text bg-gradient-to-b from-white via-white to-gray-600">
                            "CONTACT US"
                        </h1>
                    </div>

                    <div class="grid lg:grid-cols-12 gap-8 items-start">
                        // LEFT COLUMN: Context & Contact info
                        <div class="lg:col-span-4 space-y-6">
                            <p class="text-xl text-gray-400 font-medium mb-8 leading-relaxed">
                                "Deploying a professional website changes everything. Let's start the process."
                            </p>

                            <div class="space-y-4">
                                // Phone Call - Compact
                                <a href="tel:+447833263486" class="block bg-white/5 hover:bg-white/10 p-6 rounded-xl border border-white/10 hover:border-brand/50 transition-all duration-300 group">
                                    <div class="flex items-center gap-5">
                                        <div class="w-12 h-12 rounded-full bg-brand/10 flex items-center justify-center text-brand group-hover:scale-110 transition-transform duration-300">
                                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"/>
                                            </svg>
                                        </div>
                                        <div>
                                            <h3 class="text-sm font-bold text-gray-400 group-hover:text-brand-light transition mb-0.5 uppercase tracking-wider">"Call Direct"</h3>
                                            <p class="text-lg font-black text-white">+44 7833 263486</p>
                                        </div>
                                    </div>
                                </a>

                                // WhatsApp - Compact
                                <a href="https://wa.me/447833263486" target="_blank" rel="noopener noreferrer" class="block bg-white/5 hover:bg-white/10 p-6 rounded-xl border border-white/10 hover:border-[#25D366] transition-all duration-300 group">
                                    <div class="flex items-center gap-5">
                                        <div class="w-12 h-12 rounded-full bg-[#25D366]/10 flex items-center justify-center text-[#25D366] group-hover:scale-110 transition-transform duration-300">
                                            <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
                                                <path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51-.173-.008-.371-.01-.57-.01-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 01-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 01-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 012.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0012.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 005.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 00-3.48-8.413z"/>
                                            </svg>
                                        </div>
                                        <div>
                                            <h3 class="text-sm font-bold text-gray-400 group-hover:text-[#25D366] transition mb-0.5 uppercase tracking-wider">"Or WhatsApp"</h3>
                                            <p class="text-sm text-gray-500">"Message anytime"</p>
                                        </div>
                                    </div>
                                </a>
                            </div>
                        </div>

                        // RIGHT COLUMN: Glassy Form
                        <div class="lg:col-span-8">
                             <div class="bg-white/5 border border-white/10 p-8 md:p-12 rounded-3xl backdrop-blur-2xl shadow-2xl relative overflow-hidden group">
                                // Soft Glow effect on hover/focus
                                <div class="absolute -top-40 -right-40 w-80 h-80 bg-brand/10 blur-[100px] rounded-full pointer-events-none group-hover:bg-brand/20 transition-all duration-700"></div>

                                <form on:submit=submit_form class="space-y-6 relative z-10">
                                    {move || success_msg.get().map(|msg| {
                                        view! {
                                            <div class="bg-green-500/10 border border-green-500/20 text-green-400 p-4 rounded-xl flex items-center gap-3 animate-fade-in font-medium">
                                                <div class="w-6 h-6 rounded-full bg-green-500/20 flex items-center justify-center flex-shrink-0">
                                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                                </div>
                                                {msg}
                                            </div>
                                        }
                                    })}

                                    {move || error_msg.get().map(|msg| {
                                        view! {
                                            <div class="bg-brand/10 border border-brand/20 text-brand-light p-4 rounded-xl flex items-center gap-3 animate-fade-in font-medium">
                                                 <div class="w-6 h-6 rounded-full bg-brand/20 flex items-center justify-center flex-shrink-0">
                                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
                                                </div>
                                                {msg}
                                            </div>
                                        }
                                    })}

                                    <div class="space-y-6">
                                        <div class="group/input">
                                            <label class="block text-xs font-bold text-gray-500 mb-2 uppercase tracking-widest group-focus-within/input:text-white transition-colors">"Name"</label>
                                            <input
                                                type="text"
                                                class="w-full bg-black/40 border border-white/10 rounded-xl px-5 py-4 text-white focus:border-white focus:ring-1 focus:ring-white outline-none transition-all placeholder:text-gray-700 font-medium hover:bg-black/60"
                                                required
                                                placeholder="Enter your name"
                                                node_ref=name_ref
                                                disabled=move || sending.get()
                                            />
                                        </div>

                                        <div class="group/input">
                                            <label class="block text-xs font-bold text-gray-500 mb-2 uppercase tracking-widest group-focus-within/input:text-white transition-colors">"Email"</label>
                                            <input
                                                type="email"
                                                class="w-full bg-black/40 border border-white/10 rounded-xl px-5 py-4 text-white focus:border-white focus:ring-1 focus:ring-white outline-none transition-all placeholder:text-gray-700 font-medium hover:bg-black/60"
                                                required
                                                placeholder="Enter your email"
                                                node_ref=email_ref
                                                disabled=move || sending.get()
                                            />
                                        </div>
                                    </div>

                                    <div class="group/input">
                                        <div class="flex justify-between items-center mb-2">
                                             <label class="block text-xs font-bold text-gray-500 uppercase tracking-widest group-focus-within/input:text-white transition-colors">"Query"</label>
                                             <a href="/terms" class="text-[10px] uppercase font-bold text-gray-600 hover:text-white transition-colors tracking-widest">"Terms & Conditions"</a>
                                        </div>
                                        <textarea
                                            class="w-full bg-black/40 border border-white/10 rounded-xl px-5 py-4 text-white focus:border-white focus:ring-1 focus:ring-white outline-none transition-all h-40 resize-none placeholder:text-gray-700 font-medium leading-relaxed hover:bg-black/60"
                                            required
                                            placeholder="How can we help you?"
                                            node_ref=message_ref
                                            disabled=move || sending.get()
                                        ></textarea>
                                    </div>

                                    <div class="pt-2">
                                        <button
                                            type="submit"
                                            class="w-full border border-white/20 bg-transparent text-white font-black uppercase tracking-widest py-4 rounded-xl hover:bg-white hover:text-black transition-all disabled:opacity-50 disabled:cursor-not-allowed group text-sm"
                                            disabled=move || sending.get()
                                        >
                                            <span class="flex items-center justify-center gap-3">
                                                {move || if sending.get() { "SENDING..." } else { "SEND MESSAGE" }}
                                                <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"/></svg>
                                            </span>
                                        </button>
                                    </div>
                                </form>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
    }
}
