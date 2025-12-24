//! Enhanced Service Detail page with pricing, FAQs, and schema markup.
//!
//! Displays detailed service information with pricing guides,
//! FAQs for rich snippets, and booking options.

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

/// Service data structure
#[derive(Clone, Debug, PartialEq)]
struct ServiceData {
    name: &'static str,
    slug: &'static str,
    description: &'static str,
    long_description: &'static str,
    whats_included: Vec<&'static str>,
    price_from: i32,
    price_to: i32,
    common_jobs: Vec<(&'static str, i32, i32)>, // (name, price_low, price_high)
    faqs: Vec<(&'static str, &'static str)>,    // (question, answer)
}

fn get_service_data(slug: &str) -> ServiceData {
    match slug {
        "plumbing" => ServiceData {
            name: "Plumbing Services",
            slug: "plumbing",
            description: "Expert plumbing repairs and installations in Coventry",
            long_description: "Our qualified plumbers handle everything from leaky taps to complete bathroom installations. We arrive on time, keep your home clean, and get the job done right first time.",
            whats_included: vec![
                "Free quote on arrival",
                "All work guaranteed",
                "Clean-up after every job",
                "Same-day service available",
                "No call-out fee for booked jobs",
            ],
            price_from: 45,
            price_to: 150,
            common_jobs: vec![
                ("Leaky tap repair", 45, 75),
                ("Toilet repair/replacement", 75, 150),
                ("Unblock drain/sink", 55, 95),
                ("Shower/bath installation", 150, 350),
                ("Radiator replacement", 120, 200),
            ],
            faqs: vec![
                ("How quickly can you come out?", "We offer same-day service for urgent plumbing issues. For routine work, we can usually schedule within 2-3 days."),
                ("Do you provide a guarantee?", "Yes, all our work is guaranteed for 12 months. If there's an issue, we'll come back and fix it free of charge."),
                ("Are your plumbers qualified?", "All our plumbers are fully qualified with relevant certifications and are Gas Safe registered where required."),
                ("Do you charge for quotes?", "No, we provide free quotes on arrival for all jobs. You'll know the cost before we start any work."),
            ],
        },
        "electrical" => ServiceData {
            name: "Electrical Services",
            slug: "electrical",
            description: "Safe and certified electrical work in Coventry",
            long_description: "Our electricians are fully certified and insured. From light fitting to full rewires, we ensure all work meets current regulations and is completed to the highest safety standards.",
            whats_included: vec![
                "Certified electrical work",
                "Safety certificates provided",
                "Part P compliant",
                "Emergency service available",
                "All materials included in quote",
            ],
            price_from: 50,
            price_to: 200,
            common_jobs: vec![
                ("Light fitting installation", 40, 70),
                ("Socket installation", 60, 100),
                ("Fuse box inspection", 80, 120),
                ("Outdoor lighting", 100, 250),
                ("Electric shower installation", 150, 300),
            ],
            faqs: vec![
                ("Are you Part P certified?", "Yes, all our electrical work is Part P compliant and we provide certificates for notifiable work."),
                ("Can you install EV chargers?", "Yes, we install home EV charging points with OZEV grant eligibility assistance."),
                ("Do you do emergency callouts?", "Yes, we offer 24/7 emergency electrical services for urgent safety issues."),
                ("How long does light fitting take?", "A single light fitting typically takes 30-45 minutes. Multiple fittings can usually be done in a half-day."),
            ],
        },
        "carpentry" => ServiceData {
            name: "Carpentry Services",
            slug: "carpentry",
            description: "Quality carpentry and woodwork in Coventry",
            long_description: "Our skilled carpenters create beautiful, functional woodwork. From door hanging to custom shelving, we take pride in craftsmanship and attention to detail.",
            whats_included: vec![
                "Precision measurements",
                "Quality materials sourced",
                "Finishing and painting prep",
                "Dust-free cutting methods",
                "Waste removal included",
            ],
            price_from: 60,
            price_to: 200,
            common_jobs: vec![
                ("Door hanging", 65, 100),
                ("Shelf installation", 35, 60),
                ("Skirting board fitting", 80, 150),
                ("Deck repair", 150, 400),
                ("Custom wardrobes", 300, 800),
            ],
            faqs: vec![
                ("Can you match existing woodwork?", "Yes, we carefully match wood types and finishes to blend seamlessly with your existing features."),
                ("Do you supply materials?", "We can supply all materials or work with customer-provided items. We get trade discounts on quality timber."),
                ("How long to hang a door?", "A standard interior door takes about 1-2 hours. External doors may take longer due to security requirements."),
                ("Can you fix squeaky floors?", "Absolutely! We can fix squeaky floorboards and reinforce loose areas."),
            ],
        },
        "assembly" => ServiceData {
            name: "Furniture Assembly",
            slug: "assembly",
            description: "Professional flatpack assembly in Coventry",
            long_description: "Save time and frustration with our expert assembly service. We build IKEA, Wayfair, and all flatpack furniture quickly and correctly. All fixings tightened, all pieces aligned.",
            whats_included: vec![
                "All tools provided",
                "Wall anchoring if needed",
                "Packaging removal",
                "Position adjustment",
                "Operating instructions check",
            ],
            price_from: 35,
            price_to: 100,
            common_jobs: vec![
                ("Small furniture (chairs, side tables)", 25, 40),
                ("Medium (desks, dressers)", 45, 75),
                ("Large (wardrobes, beds)", 65, 120),
                ("Complex (PAX systems)", 100, 200),
                ("Office furniture setup", 80, 150),
            ],
            faqs: vec![
                ("Do you assemble IKEA furniture?", "Yes! IKEA is our specialty. We've assembled thousands of IKEA items and know the quirks of every product."),
                ("Will you take away packaging?", "Yes, we remove all packaging and dispose of it responsibly."),
                ("What if parts are missing?", "We'll identify missing parts and can either wait while you get replacements or return to complete the job."),
                ("Can you mount to walls?", "Yes, we securely anchor tall furniture to walls for safety, especially important for families with children."),
            ],
        },
        _ => ServiceData {
            name: "General Handyman Services",
            slug: "general",
            description: "Reliable general repairs in Coventry",
            long_description: "Our skilled handymen can tackle almost any job around your home. From small repairs to larger projects, we bring the right tools and expertise.",
            whats_included: vec![
                "Free assessment",
                "All common tools",
                "Quality workmanship",
                "Tidy workspace",
                "Honest advice",
            ],
            price_from: 40,
            price_to: 100,
            common_jobs: vec![
                ("TV wall mounting", 55, 95),
                ("Picture hanging (5+)", 35, 60),
                ("Blind/curtain fitting", 40, 80),
                ("Lock changing", 60, 120),
                ("Gutter cleaning", 50, 100),
            ],
            faqs: vec![
                ("Is there a minimum charge?", "Our minimum charge is £40 (1 hour). If the job takes less, we're happy to tackle other small tasks."),
                ("Do you work weekends?", "Yes, we offer Saturday appointments at no extra charge. Sunday is by special arrangement."),
                ("Can you give fixed price quotes?", "For most jobs, yes. We'll assess what's needed and give you a fixed price before starting."),
                ("Are you insured?", "Yes, we carry £2 million public liability insurance for your protection."),
            ],
        },
    }
}

#[component]
pub fn HandymanServiceDetail() -> impl IntoView {
    let params = use_params_map();
    let service_slug = move || params.get().get("slug").unwrap_or_default();

    // Get service data reactively
    let service = Memo::new(move |_| get_service_data(&service_slug()));

    view! {
        <Title text=move || format!("{} | Coventry Handyman", service.get().name)/>

        <div class="bg-slate-50 min-h-screen">
            // Hero Section
            <section class="bg-gradient-to-br from-blue-900 to-blue-800 text-white py-16 px-6">
                <div class="max-w-6xl mx-auto">
                    <a href="/handyman-coventry/services" class="text-yellow-400 hover:text-yellow-300 text-sm font-bold uppercase tracking-wide mb-4 inline-flex items-center gap-2">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                        </svg>
                        "Back to Services"
                    </a>
                    <h1 class="text-4xl md:text-5xl font-black mb-4">{move || service.get().name}</h1>
                    <p class="text-xl text-blue-200 max-w-2xl">{move || service.get().description}</p>

                    // Price range badge
                    <div class="mt-8 inline-flex items-center gap-2 bg-white/10 backdrop-blur px-6 py-3 rounded-lg">
                        <span class="text-blue-200">"Typical price range:"</span>
                        <span class="text-2xl font-black text-yellow-400">
                            {move || format!("£{} - £{}", service.get().price_from, service.get().price_to)}
                        </span>
                    </div>
                </div>
            </section>

            // Trust Bar
            <div class="bg-blue-800 py-3 px-4 flex flex-wrap items-center justify-center gap-4 md:gap-8 text-white text-sm font-medium">
                <span class="flex items-center gap-2">
                    <svg class="w-5 h-5 text-yellow-400" fill="currentColor" viewBox="0 0 20 20">
                        <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/>
                    </svg>
                    "4.9 Rating"
                </span>
                <span class="flex items-center gap-2">
                    <svg class="w-5 h-5 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>
                    </svg>
                    "Fully Insured"
                </span>
                <span class="flex items-center gap-2">
                    <svg class="w-5 h-5 text-blue-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
                    </svg>
                    "Same-Day Available"
                </span>
            </div>

            <div class="max-w-6xl mx-auto py-12 px-6">
                <div class="grid lg:grid-cols-3 gap-8">
                    // Main Content
                    <div class="lg:col-span-2 space-y-8">
                        // Overview
                        <div class="bg-white rounded-xl p-8 shadow-sm border border-slate-100">
                            <h2 class="text-2xl font-bold text-slate-900 mb-4">"About This Service"</h2>
                            <p class="text-slate-600 text-lg leading-relaxed mb-6">
                                {move || service.get().long_description}
                            </p>

                            <h3 class="text-lg font-bold text-slate-900 mb-3">"What's Included"</h3>
                            <ul class="grid md:grid-cols-2 gap-3">
                                {move || service.get().whats_included.iter().map(|item| {
                                    view! {
                                        <li class="flex items-center gap-2 text-slate-600">
                                            <svg class="w-5 h-5 text-green-500 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                            </svg>
                                            {item.to_string()}
                                        </li>
                                    }
                                }).collect_view()}
                            </ul>
                        </div>

                        // Pricing Guide
                        <div class="bg-white rounded-xl p-8 shadow-sm border border-slate-100">
                            <h2 class="text-2xl font-bold text-slate-900 mb-6">"Pricing Guide"</h2>
                            <div class="overflow-x-auto">
                                <table class="w-full">
                                    <thead>
                                        <tr class="border-b border-slate-200">
                                            <th class="text-left py-3 text-slate-600 font-medium">"Service"</th>
                                            <th class="text-right py-3 text-slate-600 font-medium">"Price Range"</th>
                                        </tr>
                                    </thead>
                                    <tbody class="divide-y divide-slate-100">
                                        {move || service.get().common_jobs.iter().map(|(name, low, high)| {
                                            view! {
                                                <tr class="hover:bg-slate-50">
                                                    <td class="py-4 text-slate-900">{name.to_string()}</td>
                                                    <td class="py-4 text-right font-bold text-blue-600">
                                                        {format!("£{} - £{}", low, high)}
                                                    </td>
                                                </tr>
                                            }
                                        }).collect_view()}
                                    </tbody>
                                </table>
                            </div>
                            <p class="text-sm text-slate-500 mt-4 italic">
                                "Prices are estimates. Final price depends on job complexity and materials required."
                            </p>
                        </div>

                        // FAQs
                        <div class="bg-white rounded-xl p-8 shadow-sm border border-slate-100">
                            <h2 class="text-2xl font-bold text-slate-900 mb-6">"Frequently Asked Questions"</h2>
                            <div class="space-y-4">
                                {move || service.get().faqs.iter().map(|(question, answer)| {
                                    view! {
                                        <details class="group border border-slate-200 rounded-lg">
                                            <summary class="flex items-center justify-between p-4 cursor-pointer hover:bg-slate-50">
                                                <span class="font-medium text-slate-900">{question.to_string()}</span>
                                                <svg class="w-5 h-5 text-slate-400 group-open:rotate-180 transition" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                                                </svg>
                                            </summary>
                                            <div class="p-4 pt-0 text-slate-600 border-t border-slate-100">
                                                {answer.to_string()}
                                            </div>
                                        </details>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    </div>

                    // Sidebar
                    <aside class="space-y-6">
                        // Book Now Card
                        <div class="bg-gradient-to-br from-blue-900 to-blue-800 text-white rounded-xl p-6 shadow-lg sticky top-24">
                            <h3 class="text-xl font-bold mb-4">"Ready to Book?"</h3>
                            <p class="text-blue-200 text-sm mb-6">
                                "Get in touch for a free quote or book your service online."
                            </p>
                            <div class="space-y-3">
                                <a
                                    href="/handyman-coventry/booking"
                                    class="block w-full text-center py-4 bg-yellow-500 text-blue-900 font-bold rounded-lg hover:bg-yellow-400 transition"
                                >
                                    "Book Online"
                                </a>
                                <a
                                    href="tel:+447833263486"
                                    class="block w-full text-center py-4 bg-white/10 backdrop-blur border border-white/20 text-white font-bold rounded-lg hover:bg-white/20 transition"
                                >
                                    "Call: 07833 263486"
                                </a>
                                <a
                                    href="/handyman-coventry/quote"
                                    class="block w-full text-center py-3 text-yellow-400 font-medium hover:text-yellow-300 transition"
                                >
                                    "Get Instant Quote"
                                </a>
                            </div>
                        </div>

                        // Service Area
                        <div class="bg-white rounded-xl p-6 shadow-sm border border-slate-100">
                            <h3 class="font-bold text-slate-900 mb-4">"Service Areas"</h3>
                            <ul class="space-y-2 text-sm text-slate-600">
                                <li class="flex items-center gap-2">
                                    <svg class="w-4 h-4 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                    </svg>
                                    "Coventry City Centre"
                                </li>
                                <li class="flex items-center gap-2">
                                    <svg class="w-4 h-4 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                    </svg>
                                    "Earlsdon, Tile Hill, Canley"
                                </li>
                                <li class="flex items-center gap-2">
                                    <svg class="w-4 h-4 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                    </svg>
                                    "Kenilworth, Warwick"
                                </li>
                                <li class="flex items-center gap-2">
                                    <svg class="w-4 h-4 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                    </svg>
                                    "Solihull, Birmingham"
                                </li>
                            </ul>
                            <a href="/handyman-coventry/service-area" class="text-blue-600 text-sm font-medium inline-block mt-4 hover:underline">
                                "View full service area map"
                            </a>
                        </div>
                    </aside>
                </div>
            </div>
        </div>
    }
}
