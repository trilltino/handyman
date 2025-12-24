//! Lead capture component for email list building.
//!
//! Provides popup/inline forms to capture leads with a free checklist offer.

use leptos::prelude::*;

/// Exit intent popup for lead capture
#[component]
pub fn LeadCapturePopup() -> impl IntoView {
    let (show_popup, set_show_popup) = signal(false);
    let (email, set_email) = signal(String::new());
    let (submitted, set_submitted) = signal(false);

    // Note: In real app, would use JS for exit intent detection
    // For now, provide a manual trigger after 10 seconds

    view! {
        // Popup overlay
        <Show when=move || show_popup.get() fallback=|| ()>
            <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4">
                <div class="bg-white rounded-2xl shadow-2xl max-w-md w-full overflow-hidden relative">
                    // Close button
                    <button
                        class="absolute top-4 right-4 text-slate-400 hover:text-slate-600"
                        on:click=move |_| set_show_popup.set(false)
                    >
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                        </svg>
                    </button>

                    <Show when=move || !submitted.get() fallback=move || view! {
                        <div class="p-8 text-center">
                            <div class="w-16 h-16 mx-auto mb-4 bg-green-100 rounded-full flex items-center justify-center">
                                <svg class="w-8 h-8 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                </svg>
                            </div>
                            <h3 class="text-2xl font-bold text-slate-900 mb-2">"Check Your Inbox!"</h3>
                            <p class="text-slate-600">"Your free checklist is on its way."</p>
                        </div>
                    }>
                        // Header
                        <div class="bg-gradient-to-r from-blue-900 to-blue-800 p-6 text-white">
                            <h3 class="text-2xl font-bold">"Free Home Maintenance Checklist"</h3>
                            <p class="text-blue-200 mt-1">"Keep your home in top shape year-round"</p>
                        </div>

                        // Form
                        <div class="p-6">
                            <ul class="space-y-2 mb-6 text-sm text-slate-600">
                                <li class="flex items-center gap-2">
                                    <svg class="w-4 h-4 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                    </svg>
                                    "Seasonal maintenance tasks"
                                </li>
                                <li class="flex items-center gap-2">
                                    <svg class="w-4 h-4 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                    </svg>
                                    "DIY vs. pro guidance"
                                </li>
                                <li class="flex items-center gap-2">
                                    <svg class="w-4 h-4 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                    </svg>
                                    "Money-saving tips"
                                </li>
                            </ul>

                            <input
                                type="email"
                                placeholder="Enter your email"
                                class="w-full px-4 py-3 rounded-lg border border-slate-300 mb-3 focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20"
                                prop:value=move || email.get()
                                on:input=move |ev| set_email.set(event_target_value(&ev))
                            />

                            <button
                                class="w-full py-3 bg-yellow-500 text-blue-900 font-bold rounded-lg hover:bg-yellow-400 transition"
                                on:click=move |_| {
                                    // TODO: Send to backend
                                    set_submitted.set(true);
                                }
                            >
                                "Get My Free Checklist"
                            </button>

                            <p class="text-xs text-slate-400 text-center mt-3">
                                "No spam, ever. Unsubscribe anytime."
                            </p>
                        </div>
                    </Show>
                </div>
            </div>
        </Show>

        // Trigger button (for demo - in production would use exit intent)
        <button
            class="fixed bottom-6 right-6 z-40 px-4 py-2 bg-blue-600 text-white rounded-full shadow-lg hover:bg-blue-700 transition flex items-center gap-2"
            on:click=move |_| set_show_popup.set(true)
        >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v13m0-13V6a2 2 0 112 2h-2zm0 0V5.5A2.5 2.5 0 109.5 8H12zm-7 4h14M5 12a2 2 0 110-4h14a2 2 0 110 4M5 12v7a2 2 0 002 2h10a2 2 0 002-2v-7"/>
            </svg>
            "Free Checklist"
        </button>
    }
}

/// Inline lead capture form for embedding in pages
#[component]
pub fn InlineLeadCapture() -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (submitted, set_submitted) = signal(false);

    view! {
        <div class="bg-gradient-to-r from-blue-900 to-blue-800 rounded-xl p-8 text-white">
            <Show when=move || !submitted.get() fallback=move || view! {
                <div class="text-center">
                    <h3 class="text-2xl font-bold mb-2">"You're In!"</h3>
                    <p class="text-blue-200">"Check your email for the free checklist."</p>
                </div>
            }>
                <div class="grid md:grid-cols-2 gap-8 items-center">
                    <div>
                        <h3 class="text-2xl font-bold mb-2">"Free Home Maintenance Checklist"</h3>
                        <p class="text-blue-200">
                            "Get our complete seasonal maintenance guide and never miss an important home task again."
                        </p>
                    </div>
                    <div class="flex gap-3">
                        <input
                            type="email"
                            placeholder="Your email"
                            class="flex-1 px-4 py-3 rounded-lg text-slate-900"
                            prop:value=move || email.get()
                            on:input=move |ev| set_email.set(event_target_value(&ev))
                        />
                        <button
                            class="px-6 py-3 bg-yellow-500 text-blue-900 font-bold rounded-lg hover:bg-yellow-400 transition whitespace-nowrap"
                            on:click=move |_| set_submitted.set(true)
                        >
                            "Get It Free"
                        </button>
                    </div>
                </div>
            </Show>
        </div>
    }
}

/// Referral banner for existing customers
#[component]
pub fn ReferralBanner() -> impl IntoView {
    view! {
        <div class="bg-gradient-to-r from-green-500 to-emerald-600 text-white py-4 px-6">
            <div class="max-w-6xl mx-auto flex flex-wrap items-center justify-between gap-4">
                <div class="flex items-center gap-3">
                    <div class="w-10 h-10 bg-white/20 rounded-full flex items-center justify-center">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                        </svg>
                    </div>
                    <div>
                        <div class="font-bold">"Refer a Friend, Get £15 Off"</div>
                        <div class="text-sm text-emerald-100">"Your friend gets £10 off their first booking too!"</div>
                    </div>
                </div>
                <a href="/handyman-coventry/referral" class="px-6 py-2 bg-white text-green-600 font-bold rounded-lg hover:bg-green-50 transition">
                    "Get Your Referral Link"
                </a>
            </div>
        </div>
    }
}
