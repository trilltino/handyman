//! Location/area pages for the handyman site.
//!
//! Geo-targeted landing pages for local SEO.

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

#[derive(Clone, Debug, PartialEq)]
struct AreaData {
    name: &'static str,
    slug: &'static str,
    description: &'static str,
    travel_time: &'static str,
    postcodes: Vec<&'static str>,
    testimonial: (&'static str, &'static str), // (quote, name)
}

fn get_area_data(slug: &str) -> AreaData {
    match slug {
        "coventry" => AreaData {
            name: "Coventry",
            slug: "coventry",
            description: "We're based in Coventry and serve all areas of the city, from the city centre to suburbs like Earlsdon, Tile Hill, and Canley.",
            travel_time: "Under 20 mins",
            postcodes: vec!["CV1", "CV2", "CV3", "CV4", "CV5", "CV6"],
            testimonial: ("Brilliant service! Fixed our leaky tap in no time. Very professional.", "John S., CV6"),
        },
        "birmingham" => AreaData {
            name: "Birmingham",
            slug: "birmingham",
            description: "We regularly serve South Birmingham areas including Solihull borders, Hall Green, and Acocks Green. Quick access via A45.",
            travel_time: "30-45 mins",
            postcodes: vec!["B26", "B27", "B28", "B33", "B34", "B36"],
            testimonial: ("Great handyman, arrived on time and did an excellent job. Will use again!", "Sarah M., B27"),
        },
        "solihull" => AreaData {
            name: "Solihull",
            slug: "solihull",
            description: "Solihull is one of our most popular service areas. We cover Shirley, Olton, Knowle, and surrounding areas.",
            travel_time: "25-35 mins",
            postcodes: vec!["B90", "B91", "B92", "B93", "B94"],
            testimonial: ("Second time using them, excellent as always. Highly recommend.", "David K., B91"),
        },
        _ => AreaData {
            name: "Coventry",
            slug: "coventry",
            description: "We serve Coventry and surrounding areas.",
            travel_time: "Under 20 mins",
            postcodes: vec!["CV1", "CV2", "CV3"],
            testimonial: ("Great service!", "Customer"),
        },
    }
}

#[component]
pub fn HandymanAreaPage() -> impl IntoView {
    let params = use_params_map();
    let area_slug = move || params.get().get("area").unwrap_or_default();
    let area = Memo::new(move |_| get_area_data(&area_slug()));

    view! {
        <Title text=move || format!("Handyman Services in {} | Coventry Handyman", area.get().name)/>

        <div class="min-h-screen bg-slate-50">
            // Hero
            <section class="bg-gradient-to-br from-blue-900 to-blue-800 text-white py-16 px-6">
                <div class="max-w-6xl mx-auto">
                    <a href="/handyman-coventry/service-area" class="text-yellow-400 hover:text-yellow-300 text-sm font-bold mb-4 inline-flex items-center gap-2">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                        </svg>
                        "All Service Areas"
                    </a>
                    <h1 class="text-4xl md:text-5xl font-black mb-4">
                        "Handyman Services in "{move || area.get().name}
                    </h1>
                    <p class="text-xl text-blue-200 max-w-2xl">
                        {move || area.get().description}
                    </p>
                    <div class="mt-6 flex items-center gap-6 text-sm">
                        <div class="flex items-center gap-2">
                            <svg class="w-5 h-5 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
                            </svg>
                            "Response time: "{move || area.get().travel_time}
                        </div>
                        <div class="flex items-center gap-2">
                            <svg class="w-5 h-5 text-yellow-400" fill="currentColor" viewBox="0 0 20 20">
                                <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/>
                            </svg>
                            "4.9 rating"
                        </div>
                    </div>
                </div>
            </section>

            <div class="max-w-6xl mx-auto py-12 px-6">
                <div class="grid lg:grid-cols-3 gap-8">
                    // Main content
                    <div class="lg:col-span-2 space-y-8">
                        // Services
                        <div class="bg-white rounded-xl p-8 shadow-sm">
                            <h2 class="text-2xl font-bold text-slate-900 mb-6">
                                "Services We Offer in "{move || area.get().name}
                            </h2>
                            <div class="grid md:grid-cols-2 gap-4">
                                <a href="/handyman-coventry/services/plumbing" class="flex items-center gap-3 p-4 rounded-lg border border-slate-200 hover:border-blue-300 hover:bg-blue-50 transition">
                                    <div class="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center text-blue-600">
                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"/>
                                        </svg>
                                    </div>
                                    <span class="font-medium text-slate-900">"Plumbing"</span>
                                </a>
                                <a href="/handyman-coventry/services/electrical" class="flex items-center gap-3 p-4 rounded-lg border border-slate-200 hover:border-blue-300 hover:bg-blue-50 transition">
                                    <div class="w-10 h-10 bg-yellow-100 rounded-lg flex items-center justify-center text-yellow-600">
                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                                        </svg>
                                    </div>
                                    <span class="font-medium text-slate-900">"Electrical"</span>
                                </a>
                                <a href="/handyman-coventry/services/carpentry" class="flex items-center gap-3 p-4 rounded-lg border border-slate-200 hover:border-blue-300 hover:bg-blue-50 transition">
                                    <div class="w-10 h-10 bg-amber-100 rounded-lg flex items-center justify-center text-amber-600">
                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/>
                                        </svg>
                                    </div>
                                    <span class="font-medium text-slate-900">"Carpentry"</span>
                                </a>
                                <a href="/handyman-coventry/services/assembly" class="flex items-center gap-3 p-4 rounded-lg border border-slate-200 hover:border-blue-300 hover:bg-blue-50 transition">
                                    <div class="w-10 h-10 bg-green-100 rounded-lg flex items-center justify-center text-green-600">
                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>
                                        </svg>
                                    </div>
                                    <span class="font-medium text-slate-900">"Furniture Assembly"</span>
                                </a>
                            </div>
                        </div>

                        // Postcodes
                        <div class="bg-white rounded-xl p-8 shadow-sm">
                            <h2 class="text-2xl font-bold text-slate-900 mb-4">"Postcodes We Cover"</h2>
                            <div class="flex flex-wrap gap-2">
                                {move || area.get().postcodes.iter().map(|pc| {
                                    view! {
                                        <span class="px-4 py-2 bg-blue-50 text-blue-700 rounded-lg font-medium">
                                            {pc.to_string()}
                                        </span>
                                    }
                                }).collect_view()}
                            </div>
                        </div>

                        // Testimonial
                        <div class="bg-gradient-to-br from-blue-50 to-indigo-50 rounded-xl p-8">
                            <div class="flex gap-1 text-yellow-500 mb-4">
                                {(0..5).map(|_| view! {
                                    <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                                        <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/>
                                    </svg>
                                }).collect_view()}
                            </div>
                            <blockquote class="text-lg text-slate-700 italic mb-4">
                                "\"{move || area.get().testimonial.0}\""
                            </blockquote>
                            <p class="font-bold text-slate-900">{move || area.get().testimonial.1}</p>
                        </div>
                    </div>

                    // Sidebar
                    <aside class="space-y-6">
                        <div class="bg-gradient-to-br from-blue-900 to-blue-800 text-white rounded-xl p-6 shadow-lg sticky top-24">
                            <h3 class="text-xl font-bold mb-4">
                                "Book in "{move || area.get().name}
                            </h3>
                            <p class="text-blue-200 text-sm mb-6">
                                "Get a quote or book now for fast service."
                            </p>
                            <div class="space-y-3">
                                <a href="/handyman-coventry/booking" class="block w-full text-center py-4 bg-yellow-500 text-blue-900 font-bold rounded-lg hover:bg-yellow-400 transition">
                                    "Book Online"
                                </a>
                                <a href="tel:+447833263486" class="block w-full text-center py-4 bg-white/10 backdrop-blur border border-white/20 text-white font-bold rounded-lg hover:bg-white/20 transition">
                                    "Call: 07833 263486"
                                </a>
                            </div>
                        </div>

                        // Other areas
                        <div class="bg-white rounded-xl p-6 shadow-sm">
                            <h3 class="font-bold text-slate-900 mb-4">"Other Areas"</h3>
                            <ul class="space-y-2 text-sm">
                                <li>
                                    <a href="/handyman-coventry/areas/coventry" class="text-blue-600 hover:underline">"Coventry"</a>
                                </li>
                                <li>
                                    <a href="/handyman-coventry/areas/birmingham" class="text-blue-600 hover:underline">"Birmingham"</a>
                                </li>
                                <li>
                                    <a href="/handyman-coventry/areas/solihull" class="text-blue-600 hover:underline">"Solihull"</a>
                                </li>
                            </ul>
                        </div>
                    </aside>
                </div>
            </div>
        </div>
    }
}
