//! Emergency services page with prominent CTAs.

use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn HandymanEmergency() -> impl IntoView {
    view! {
        <Title text="Emergency Handyman Services | Coventry"/>

        <div class="min-h-screen bg-slate-50">
            // Hero with big phone number
            <section class="bg-gradient-to-br from-red-600 to-red-700 text-white py-16 px-6 text-center">
                <div class="max-w-3xl mx-auto">
                    <div class="inline-flex items-center gap-2 bg-white/20 px-4 py-2 rounded-full text-sm font-medium mb-6">
                        <span class="w-3 h-3 bg-green-400 rounded-full animate-pulse"></span>
                        "Available Now"
                    </div>
                    <h1 class="text-4xl md:text-5xl font-black mb-4">"Emergency Handyman"</h1>
                    <p class="text-xl text-red-100 mb-8">
                        "Same-day service for urgent repairs in Coventry and surrounding areas."
                    </p>

                    // Big phone button
                    <a
                        href="tel:+447833263486"
                        class="inline-flex items-center gap-3 px-8 py-5 bg-white text-red-600 text-2xl font-black rounded-xl hover:bg-red-50 transition shadow-xl"
                    >
                        <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"/>
                        </svg>
                        "07833 263486"
                    </a>
                </div>
            </section>

            // Emergency services list
            <section class="py-12 px-6">
                <div class="max-w-4xl mx-auto">
                    <h2 class="text-2xl font-bold text-slate-900 text-center mb-8">"Emergency Services We Cover"</h2>

                    <div class="grid md:grid-cols-2 gap-4">
                        <div class="bg-white rounded-xl p-6 shadow-sm border border-slate-100 flex items-start gap-4">
                            <div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center text-blue-600 shrink-0">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"/>
                                </svg>
                            </div>
                            <div>
                                <h3 class="font-bold text-slate-900">"Burst Pipes & Leaks"</h3>
                                <p class="text-sm text-slate-600">"Quick response to stop water damage"</p>
                            </div>
                        </div>

                        <div class="bg-white rounded-xl p-6 shadow-sm border border-slate-100 flex items-start gap-4">
                            <div class="w-12 h-12 bg-yellow-100 rounded-lg flex items-center justify-center text-yellow-600 shrink-0">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                                </svg>
                            </div>
                            <div>
                                <h3 class="font-bold text-slate-900">"Power Outages"</h3>
                                <p class="text-sm text-slate-600">"Electrical fault finding and repair"</p>
                            </div>
                        </div>

                        <div class="bg-white rounded-xl p-6 shadow-sm border border-slate-100 flex items-start gap-4">
                            <div class="w-12 h-12 bg-red-100 rounded-lg flex items-center justify-center text-red-600 shrink-0">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"/>
                                </svg>
                            </div>
                            <div>
                                <h3 class="font-bold text-slate-900">"Lock Issues"</h3>
                                <p class="text-sm text-slate-600">"Broken locks, forced entry repair"</p>
                            </div>
                        </div>

                        <div class="bg-white rounded-xl p-6 shadow-sm border border-slate-100 flex items-start gap-4">
                            <div class="w-12 h-12 bg-amber-100 rounded-lg flex items-center justify-center text-amber-600 shrink-0">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/>
                                </svg>
                            </div>
                            <div>
                                <h3 class="font-bold text-slate-900">"Storm Damage"</h3>
                                <p class="text-sm text-slate-600">"Temporary fixes and repairs"</p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Pricing transparency
            <section class="py-12 px-6 bg-white">
                <div class="max-w-4xl mx-auto text-center">
                    <h2 class="text-2xl font-bold text-slate-900 mb-4">"Emergency Pricing"</h2>
                    <p class="text-slate-600 mb-8">"Transparent pricing with no hidden fees."</p>

                    <div class="grid md:grid-cols-3 gap-6">
                        <div class="bg-slate-50 rounded-xl p-6">
                            <div class="text-3xl font-black text-blue-600 mb-2">"£45"</div>
                            <div class="text-sm text-slate-600">"Emergency call-out fee"</div>
                        </div>
                        <div class="bg-slate-50 rounded-xl p-6">
                            <div class="text-3xl font-black text-blue-600 mb-2">"£60/hr"</div>
                            <div class="text-sm text-slate-600">"Emergency hourly rate"</div>
                        </div>
                        <div class="bg-slate-50 rounded-xl p-6">
                            <div class="text-3xl font-black text-green-600 mb-2">"Free"</div>
                            <div class="text-sm text-slate-600">"Quote on arrival"</div>
                        </div>
                    </div>

                    <p class="text-sm text-slate-500 mt-6">"Prices may vary for out-of-hours (after 6pm and weekends)."</p>
                </div>
            </section>
        </div>
    }
}
