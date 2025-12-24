//! Handyman Booking Integration

use crate::pages::examples::handyman_app::components::{GlassCard, SectionTitle};
use leptos::prelude::*;

#[component]
pub fn HandymanBooking() -> impl IntoView {
    let (step, set_step) = signal(1);
    // 1 = Service, 2 = Details, 3 = Confirmation

    view! {
        <div class="bg-slate-50 min-h-screen py-20 px-6">
            <SectionTitle subtitle="Online Booking" title="Schedule Your Expert" align="center"/>

            <div class="max-w-4xl mx-auto">
                // Progress Bar
                <div class="flex items-center justify-between mb-12 relative max-w-2xl mx-auto">
                    <div class="absolute h-1 bg-gray-200 top-1/2 left-0 right-0 -z-10 transform -translate-y-1/2"></div>
                    <div class={format!("absolute h-1 bg-blue-900 top-1/2 left-0 -z-10 transform -translate-y-1/2 transition-all duration-500 ease-in-out w-0 {}",
                        if step.get() == 1 { "w-0" } else if step.get() == 2 { "w-1/2" } else { "w-full" }
                    )}></div>

                    <StepIndicator num=1 active={move || step.get() >= 1} label="Service" />
                    <StepIndicator num=2 active={move || step.get() >= 2} label="Details" />
                    <StepIndicator num=3 active={move || step.get() >= 3} label="Confirm" />
                </div>

                <GlassCard class="min-h-[400px]">
                    {move || match step.get() {
                        1 => view! {
                            <div class="animate-fade-in space-y-8">
                                <h3 class="text-2xl font-bold text-center text-blue-900">"What do you need help with?"</h3>
                                <div class="grid grid-cols-2 md:grid-cols-3 gap-4">
                                     <ServiceSelectBox icon="[R]" label="General Repair" on_click=move |_| set_step.set(2) />
                                     <ServiceSelectBox icon="[P]" label="Plumbing" on_click=move |_| set_step.set(2) />
                                     <ServiceSelectBox icon="[E]" label="Electrical" on_click=move |_| set_step.set(2) />
                                     <ServiceSelectBox icon="[A]" label="Assembly" on_click=move |_| set_step.set(2) />
                                     <ServiceSelectBox icon="[D]" label="Painting" on_click=move |_| set_step.set(2) />
                                     <ServiceSelectBox icon="[?]" label="Other" on_click=move |_| set_step.set(2) />
                                </div>
                            </div>
                        }.into_any(),
                        2 => view! {
                            <div class="animate-fade-in space-y-6">
                                <h3 class="text-2xl font-bold text-center text-blue-900">"Your Contact Details"</h3>
                                <div class="grid md:grid-cols-2 gap-6">
                                    <div class="space-y-2">
                                        <label class="block text-sm font-bold text-gray-700">"Full Name"</label>
                                        <input type="text" class="w-full p-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-900 focus:border-transparent outline-none transition" placeholder="John Doe"/>
                                    </div>
                                    <div class="space-y-2">
                                        <label class="block text-sm font-bold text-gray-700">"Phone Number"</label>
                                        <input type="tel" class="w-full p-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-900 focus:border-transparent outline-none transition" placeholder="07123 456789"/>
                                    </div>
                                     <div class="space-y-2 md:col-span-2">
                                        <label class="block text-sm font-bold text-gray-700">"Job Description"</label>
                                        <textarea class="w-full p-3 border border-gray-300 rounded-lg h-32 focus:ring-2 focus:ring-blue-900 focus:border-transparent outline-none transition" placeholder="Briefly describe what needs fixing..."></textarea>
                                    </div>
                                </div>
                                <div class="flex justify-between pt-6">
                                    <button on:click=move |_| set_step.set(1) class="text-gray-500 hover:text-blue-900 font-bold">"Back"</button>
                                    <button on:click=move |_| set_step.set(3) class="bg-blue-900 text-white px-8 py-3 rounded-lg font-bold hover:bg-blue-800 transition shadow-lg">"Review Booking"</button>
                                </div>
                            </div>
                        }.into_any(),
                        3 => view! {
                             <div class="animate-fade-in text-center space-y-6 py-8">
                                <div class="w-20 h-20 bg-green-100 text-green-600 rounded-full flex items-center justify-center mx-auto mb-6">
                                    <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                                </div>
                                <h3 class="text-3xl font-black text-blue-900">"Booking Requested!"</h3>
                                <p class="text-gray-600 max-w-md mx-auto">"Thanks, we have received your request. One of our team members will call you shortly to confirm the appointment."</p>
                                <div class="pt-8">
                                    <a href="/handyman-coventry" class="bg-yellow-500 text-blue-900 px-8 py-3 rounded-lg font-bold hover:bg-yellow-400 transition shadow-lg">"Back to Home"</a>
                                </div>
                            </div>
                        }.into_any(),
                        _ => ().into_any()
                    }}
                </GlassCard>
            </div>
        </div>
    }
}

#[component]
fn StepIndicator(
    num: i32,
    active: impl Fn() -> bool + Send + Copy + 'static,
    label: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center bg-slate-50 px-2 z-10">
            <div class={move || format!("w-10 h-10 rounded-full flex items-center justify-center font-bold border-2 transition-all duration-300 {}",
                if active() { "bg-blue-900 border-blue-900 text-white scale-110" } else { "bg-white border-gray-300 text-gray-400" })}>
                {num}
            </div>
            <span class={move || format!("text-xs font-bold mt-2 uppercase tracking-wide transition-colors {}",
                if active() { "text-blue-900" } else { "text-gray-400" })}>
                {label}
            </span>
        </div>
    }
}

#[component]
fn ServiceSelectBox(
    icon: &'static str,
    label: &'static str,
    on_click: impl Fn(web_sys::MouseEvent) + 'static,
) -> impl IntoView {
    view! {
        <button on:click=on_click class="flex flex-col items-center justify-center p-6 border-2 border-gray-100 rounded-xl hover:border-blue-500 hover:bg-blue-50/50 transition-all duration-300 group">
            <span class="text-4xl mb-3 group-hover:scale-110 transition-transform">{icon}</span>
            <span class="font-bold text-gray-700 group-hover:text-blue-900">{label}</span>
        </button>
    }
}
