//! Trust signals component for handyman pages.
//!
//! Displays badges and trust indicators to build customer confidence.

use leptos::prelude::*;

/// Trust bar component displaying key trust signals.
///
/// Shows rating, insurance status, DBS check, and locality.
#[component]
pub fn TrustBar() -> impl IntoView {
    view! {
        <div class="bg-gradient-to-r from-blue-900 to-blue-800 py-3 px-4 flex flex-wrap items-center justify-center gap-4 md:gap-8 text-white text-sm font-medium shadow-md">
            <span class="flex items-center gap-2">
                <svg class="w-5 h-5 text-yellow-400" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/>
                </svg>
                "4.9 Rating (127 reviews)"
            </span>
            <span class="flex items-center gap-2">
                <svg class="w-5 h-5 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>
                </svg>
                "Fully Insured"
            </span>
            <span class="flex items-center gap-2">
                <svg class="w-5 h-5 text-blue-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                </svg>
                "DBS Checked"
            </span>
            <span class="flex items-center gap-2">
                <svg class="w-5 h-5 text-yellow-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"/>
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"/>
                </svg>
                "Local to Coventry"
            </span>
        </div>
    }
}

/// Full trust signals section with certifications.
#[component]
pub fn TrustSignals() -> impl IntoView {
    view! {
        <section class="py-16 px-6 bg-gradient-to-b from-slate-100 to-white">
            <div class="max-w-6xl mx-auto">
                <h2 class="text-3xl font-bold text-center text-slate-900 mb-12">"Why Trust Us?"</h2>

                <div class="grid md:grid-cols-4 gap-8">
                    // Fully Insured
                    <div class="bg-white rounded-xl p-6 shadow-lg border border-slate-100 text-center hover:shadow-xl transition">
                        <div class="w-16 h-16 mx-auto mb-4 bg-green-100 rounded-full flex items-center justify-center">
                            <svg class="w-8 h-8 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>
                            </svg>
                        </div>
                        <h3 class="font-bold text-lg text-slate-900 mb-2">"Fully Insured"</h3>
                        <p class="text-slate-600 text-sm">"£2M public liability insurance for your peace of mind"</p>
                    </div>

                    // DBS Checked
                    <div class="bg-white rounded-xl p-6 shadow-lg border border-slate-100 text-center hover:shadow-xl transition">
                        <div class="w-16 h-16 mx-auto mb-4 bg-blue-100 rounded-full flex items-center justify-center">
                            <svg class="w-8 h-8 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                            </svg>
                        </div>
                        <h3 class="font-bold text-lg text-slate-900 mb-2">"DBS Checked"</h3>
                        <p class="text-slate-600 text-sm">"Enhanced background checks for your security"</p>
                    </div>

                    // Top Rated
                    <div class="bg-white rounded-xl p-6 shadow-lg border border-slate-100 text-center hover:shadow-xl transition">
                        <div class="w-16 h-16 mx-auto mb-4 bg-yellow-100 rounded-full flex items-center justify-center">
                            <svg class="w-8 h-8 text-yellow-600" fill="currentColor" viewBox="0 0 20 20">
                                <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/>
                            </svg>
                        </div>
                        <h3 class="font-bold text-lg text-slate-900 mb-2">"4.9 Star Rating"</h3>
                        <p class="text-slate-600 text-sm">"127+ verified reviews from happy customers"</p>
                    </div>

                    // Local
                    <div class="bg-white rounded-xl p-6 shadow-lg border border-slate-100 text-center hover:shadow-xl transition">
                        <div class="w-16 h-16 mx-auto mb-4 bg-purple-100 rounded-full flex items-center justify-center">
                            <svg class="w-8 h-8 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"/>
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"/>
                            </svg>
                        </div>
                        <h3 class="font-bold text-lg text-slate-900 mb-2">"Local to Coventry"</h3>
                        <p class="text-slate-600 text-sm">"Based in Coventry, serving the West Midlands"</p>
                    </div>
                </div>

                // Stats row
                <div class="mt-12 grid md:grid-cols-3 gap-6 text-center">
                    <div class="bg-blue-900 rounded-xl p-6 text-white">
                        <div class="text-4xl font-black text-yellow-400">"375+"</div>
                        <div class="text-blue-200">"Happy Customers"</div>
                    </div>
                    <div class="bg-blue-900 rounded-xl p-6 text-white">
                        <div class="text-4xl font-black text-yellow-400">"1,200+"</div>
                        <div class="text-blue-200">"Jobs Completed"</div>
                    </div>
                    <div class="bg-blue-900 rounded-xl p-6 text-white">
                        <div class="text-4xl font-black text-yellow-400">"5+"</div>
                        <div class="text-blue-200">"Years Experience"</div>
                    </div>
                </div>
            </div>
        </section>
    }
}
