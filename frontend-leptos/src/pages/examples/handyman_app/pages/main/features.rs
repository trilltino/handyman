//! Handyman Features & Certifications Page

use crate::pages::examples::handyman_app::components::{GlassCard, SectionTitle};
use leptos::prelude::*;

#[component]
pub fn HandymanFeatures() -> impl IntoView {
    view! {
        <div class="bg-slate-50 min-h-screen py-20 px-6">
             <SectionTitle subtitle="Why Choose Us" title="Certified & Guaranteed" align="center"/>

             <div class="max-w-6xl mx-auto space-y-20">
                // Certifications Section
                <div class="grid md:grid-cols-2 gap-12 items-center">
                    <div class="space-y-6">
                        <h3 class="text-3xl font-bold text-blue-900">"Fully Insured & Qualified"</h3>
                        <p class="text-gray-600 text-lg">"We don't take risks with your home. We hold comprehensive public liability insurance and relevant trade qualifications."</p>

                        <div class="space-y-4 pt-4">
                            <FeatureRow icon="shield-check" title="£2M Public Liability Insurance" desc="Covered by AXA Business Insurance"/>
                            <FeatureRow icon="academic-cap" title="City & Guilds Qualified" desc="Level 2 Diploma in Maintenance Operations"/>
                            <FeatureRow icon="user-group" title="DBS Checked Staff" desc="For your peace of mind and safety"/>
                        </div>
                    </div>
                    <div class="relative">
                        <div class="absolute inset-0 bg-gradient-to-tr from-blue-900/20 to-yellow-500/20 rounded-2xl transform rotate-3"></div>
                        <GlassCard class="relative transform -rotate-2">
                             <div class="flex items-center justify-between border-b pb-4 mb-4">
                                <span class="font-bold text-gray-500">"CERTIFICATE OF INSURANCE"</span>
                                <span class="text-green-600 font-bold text-sm">"● ACTIVE"</span>
                             </div>
                             <div class="space-y-4 opacity-50 blur-[1px]">
                                <div class="h-4 bg-gray-200 rounded w-3/4"></div>
                                <div class="h-4 bg-gray-200 rounded w-full"></div>
                                <div class="h-4 bg-gray-200 rounded w-5/6"></div>
                                <div class="h-4 bg-gray-200 rounded w-1/2"></div>
                             </div>
                             // Wax Seal Effect
                             <div class="absolute bottom-8 right-8 w-24 h-24 bg-yellow-500 rounded-full flex items-center justify-center text-blue-900 font-black rotate-12 shadow-lg border-4 border-yellow-400/50">
                                <div class="text-center text-xs leading-tight">
                                    "OFFICIAL"<br/>"VERIFIED"<br/>"2026"
                                </div>
                             </div>
                        </GlassCard>
                    </div>
                </div>

                // Guarantee Section
                <div class="bg-blue-900 rounded-3xl p-12 text-white relative overflow-hidden shadow-2xl">
                    <div class="absolute top-0 right-0 w-96 h-96 bg-yellow-500/10 rounded-full blur-3xl transform translate-x-1/2 -translate-y-1/2"></div>

                    <div class="relative z-10 text-center max-w-3xl mx-auto space-y-8">
                        <h2 class="text-4xl font-bold">"Our 100% Satisfaction Guarantee"</h2>
                        <p class="text-blue-100 text-xl leading-relaxed">
                            "If you aren't completely happy with our workmanship, we will return and fix it for free. No quibbles, no arguments. We value our reputation above all else."
                        </p>
                        <div class="grid md:grid-cols-3 gap-6 text-center pt-8">
                             <div class="p-6 bg-white/5 rounded-xl backdrop-blur">
                                <div class="text-3xl mb-2">"12"</div>
                                <div class="text-sm font-bold uppercase text-yellow-400">"Month Warranty"</div>
                             </div>
                             <div class="p-6 bg-white/5 rounded-xl backdrop-blur">
                                <div class="text-3xl mb-2">"0"</div>
                                <div class="text-sm font-bold uppercase text-yellow-400">"Call out Fees"</div>
                             </div>
                             <div class="p-6 bg-white/5 rounded-xl backdrop-blur">
                                <div class="text-3xl mb-2">"24/7"</div>
                                <div class="text-sm font-bold uppercase text-yellow-400">"Support"</div>
                             </div>
                        </div>
                    </div>
                </div>
             </div>
        </div>
    }
}

#[component]
fn FeatureRow(icon: &'static str, title: &'static str, desc: &'static str) -> impl IntoView {
    let _ = icon;
    view! {
        <div class="flex items-start gap-4 p-4 bg-white rounded-xl shadow-sm border border-gray-100">
            <div class="w-12 h-12 bg-blue-100 text-blue-800 rounded-lg flex items-center justify-center shrink-0">
                // Simulating generic icon placeholder or mapping needed
                 <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
            </div>
            <div>
                <h4 class="font-bold text-gray-900 text-lg">{title}</h4>
                <p class="text-sm text-gray-500">{desc}</p>
            </div>
        </div>
    }
}
