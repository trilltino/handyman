//! Instant Quote Calculator page.
//!
//! Allows customers to get an instant estimate for their job.

use leptos::prelude::*;
use leptos_meta::Title;

/// Service category for quote calculation
#[derive(Clone, Debug, PartialEq)]
struct ServiceOption {
    value: &'static str,
    label: &'static str,
    base_low: i32,
    base_high: i32,
}

const SERVICES: &[ServiceOption] = &[
    ServiceOption {
        value: "plumbing",
        label: "Plumbing",
        base_low: 45,
        base_high: 120,
    },
    ServiceOption {
        value: "electrical",
        label: "Electrical",
        base_low: 50,
        base_high: 150,
    },
    ServiceOption {
        value: "carpentry",
        label: "Carpentry",
        base_low: 60,
        base_high: 180,
    },
    ServiceOption {
        value: "assembly",
        label: "Furniture Assembly",
        base_low: 35,
        base_high: 85,
    },
    ServiceOption {
        value: "painting",
        label: "Painting & Decorating",
        base_low: 80,
        base_high: 250,
    },
    ServiceOption {
        value: "general",
        label: "General Repairs",
        base_low: 40,
        base_high: 100,
    },
];

#[component]
pub fn HandymanQuote() -> impl IntoView {
    // Form state
    let (service, set_service) = signal("plumbing".to_string());
    let (urgency, set_urgency) = signal("within_3_days".to_string());
    let (description, set_description) = signal(String::new());

    // Calculate estimates based on selections
    let estimate = Memo::new(move |_| {
        let svc = service.get();
        let urg = urgency.get();

        let (base_low, base_high) = SERVICES
            .iter()
            .find(|s| s.value == svc)
            .map(|s| (s.base_low, s.base_high))
            .unwrap_or((40, 100));

        let urgency_modifier = match urg.as_str() {
            "same_day" => 15,
            "within_3_days" => 0,
            "flexible" => -5,
            _ => 0,
        };

        (base_low + urgency_modifier, base_high + urgency_modifier)
    });

    view! {
        <Title text="Get a Quote | Coventry Handyman"/>

        <div class="min-h-screen bg-gradient-to-b from-slate-50 to-white">
            // Hero
            <section class="bg-blue-900 text-white py-16 px-6">
                <div class="max-w-4xl mx-auto text-center">
                    <h1 class="text-4xl md:text-5xl font-black mb-4">"Get an Instant Estimate"</h1>
                    <p class="text-xl text-blue-200 max-w-2xl mx-auto">
                        "Tell us about your job and get an estimated price range in seconds. No obligation, no hassle."
                    </p>
                </div>
            </section>

            // Calculator
            <section class="py-16 px-6 -mt-8">
                <div class="max-w-2xl mx-auto">
                    <div class="bg-white rounded-2xl shadow-xl border border-slate-100 overflow-hidden">
                        <div class="p-8 space-y-6">
                            // Service Type
                            <div>
                                <label class="block text-sm font-bold text-slate-700 mb-2">
                                    "What do you need help with?"
                                </label>
                                <select
                                    class="w-full px-4 py-3 rounded-lg border border-slate-300 focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 transition text-lg"
                                    on:change=move |ev| {
                                        set_service.set(event_target_value(&ev));
                                    }
                                >
                                    {SERVICES.iter().map(|svc| {
                                        view! {
                                            <option value=svc.value selected=svc.value == "plumbing">
                                                {svc.label}
                                            </option>
                                        }
                                    }).collect_view()}
                                </select>
                            </div>

                            // Job Description
                            <div>
                                <label class="block text-sm font-bold text-slate-700 mb-2">
                                    "Describe the job (optional)"
                                </label>
                                <textarea
                                    class="w-full px-4 py-3 rounded-lg border border-slate-300 focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 transition"
                                    rows="3"
                                    placeholder="e.g., Fix a leaky kitchen tap"
                                    prop:value=move || description.get()
                                    on:input=move |ev| {
                                        set_description.set(event_target_value(&ev));
                                    }
                                ></textarea>
                            </div>

                            // Urgency
                            <div>
                                <label class="block text-sm font-bold text-slate-700 mb-3">
                                    "How urgent is this?"
                                </label>
                                <div class="grid grid-cols-3 gap-3">
                                    <button
                                        class=move || format!(
                                            "p-4 rounded-lg border-2 text-center transition {}",
                                            if urgency.get() == "same_day" {
                                                "border-yellow-500 bg-yellow-50"
                                            } else {
                                                "border-slate-200 hover:border-slate-300"
                                            }
                                        )
                                        on:click=move |_| set_urgency.set("same_day".to_string())
                                    >
                                        <div class="font-bold text-slate-900">"Same Day"</div>
                                        <div class="text-sm text-slate-500">"+£15"</div>
                                    </button>
                                    <button
                                        class=move || format!(
                                            "p-4 rounded-lg border-2 text-center transition {}",
                                            if urgency.get() == "within_3_days" {
                                                "border-blue-500 bg-blue-50"
                                            } else {
                                                "border-slate-200 hover:border-slate-300"
                                            }
                                        )
                                        on:click=move |_| set_urgency.set("within_3_days".to_string())
                                    >
                                        <div class="font-bold text-slate-900">"Within 3 Days"</div>
                                        <div class="text-sm text-slate-500">"Standard"</div>
                                    </button>
                                    <button
                                        class=move || format!(
                                            "p-4 rounded-lg border-2 text-center transition {}",
                                            if urgency.get() == "flexible" {
                                                "border-green-500 bg-green-50"
                                            } else {
                                                "border-slate-200 hover:border-slate-300"
                                            }
                                        )
                                        on:click=move |_| set_urgency.set("flexible".to_string())
                                    >
                                        <div class="font-bold text-slate-900">"Flexible"</div>
                                        <div class="text-sm text-green-600">"-£5"</div>
                                    </button>
                                </div>
                            </div>
                        </div>

                        // Estimate Display
                        <div class="bg-gradient-to-r from-blue-900 to-blue-800 p-8 text-white">
                            <div class="text-center">
                                <div class="text-sm uppercase tracking-wide text-blue-200 mb-2">"Estimated Cost"</div>
                                <div class="text-5xl font-black mb-2">
                                    {move || {
                                        let (low, high) = estimate.get();
                                        format!("£{} - £{}", low, high)
                                    }}
                                </div>
                                <div class="text-blue-200 text-sm">
                                    "Final price depends on job complexity and materials needed"
                                </div>
                            </div>
                        </div>

                        // CTA Buttons
                        <div class="p-8 bg-slate-50 grid md:grid-cols-2 gap-4">
                            <a
                                href="/handyman-coventry/booking"
                                class="block text-center px-6 py-4 bg-yellow-500 text-blue-900 rounded-lg font-bold text-lg hover:bg-yellow-400 hover:shadow-lg transition"
                            >
                                "Book Online Now"
                            </a>
                            <a
                                href="tel:+447833263486"
                                class="block text-center px-6 py-4 bg-white border-2 border-blue-900 text-blue-900 rounded-lg font-bold text-lg hover:bg-blue-50 transition"
                            >
                                "Call to Discuss"
                            </a>
                        </div>
                    </div>

                    // Trust signals
                    <div class="mt-8 flex flex-wrap justify-center gap-4 text-sm text-slate-600">
                        <span class="flex items-center gap-2">
                            <svg class="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                            </svg>
                            "No obligation"
                        </span>
                        <span class="flex items-center gap-2">
                            <svg class="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                            </svg>
                            "Free estimates"
                        </span>
                        <span class="flex items-center gap-2">
                            <svg class="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                            </svg>
                            "No hidden fees"
                        </span>
                    </div>
                </div>
            </section>

            // Pricing Guide
            <section class="py-16 px-6 bg-slate-100">
                <div class="max-w-4xl mx-auto">
                    <h2 class="text-3xl font-bold text-center text-slate-900 mb-12">"Pricing Guide"</h2>
                    <div class="grid md:grid-cols-2 gap-6">
                        <div class="bg-white rounded-xl p-6 shadow-sm">
                            <h3 class="font-bold text-lg text-slate-900 mb-4">"Standard Rates"</h3>
                            <ul class="space-y-3 text-slate-600">
                                <li class="flex justify-between">
                                    <span>"Call-out fee"</span>
                                    <span class="font-bold text-slate-900">"£30"</span>
                                </li>
                                <li class="flex justify-between">
                                    <span>"Hourly rate"</span>
                                    <span class="font-bold text-slate-900">"£45/hr"</span>
                                </li>
                                <li class="flex justify-between">
                                    <span>"Materials markup"</span>
                                    <span class="font-bold text-slate-900">"Cost + 15%"</span>
                                </li>
                            </ul>
                        </div>
                        <div class="bg-white rounded-xl p-6 shadow-sm">
                            <h3 class="font-bold text-lg text-slate-900 mb-4">"Common Jobs"</h3>
                            <ul class="space-y-3 text-slate-600">
                                <li class="flex justify-between">
                                    <span>"Leaky tap repair"</span>
                                    <span class="font-bold text-slate-900">"£45 - £85"</span>
                                </li>
                                <li class="flex justify-between">
                                    <span>"Light fitting"</span>
                                    <span class="font-bold text-slate-900">"£40 - £70"</span>
                                </li>
                                <li class="flex justify-between">
                                    <span>"TV wall mount"</span>
                                    <span class="font-bold text-slate-900">"£55 - £95"</span>
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}
