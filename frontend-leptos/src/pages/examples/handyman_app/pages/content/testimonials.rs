//! Handyman Testimonials Page

use crate::pages::examples::handyman_app::components::{GlassCard, SectionTitle};
use leptos::prelude::*;

#[component]
pub fn HandymanTestimonials() -> impl IntoView {
    view! {
        <div class="bg-slate-50 min-h-screen py-20 px-6">
            <SectionTitle subtitle="Success Stories" title="Trusted by Your Neighbors" align="center"/>

            <div class="max-w-7xl mx-auto grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                // Column 1
                <div class="space-y-8">
                    <TestimonialCard
                        name="Sarah Jenkins"
                        location="Earlsdon, Coventry"
                        text="Absolutely brilliant service. Arrived on time, fixed the leaking pipe in under an hour, and left the place spotless. Highly recommended!"
                        rating=5
                    />
                    <TestimonialCard
                        name="Mike Ross"
                        location="Tile Hill"
                        text="Used them for flat pack assembly. Saved me hours of frustration. Worth every penny."
                        rating=5
                    />
                </div>
                // Column 2
                <div class="space-y-8">
                     <GlassCard class="bg-blue-900 text-white !border-blue-800">
                        <div class="text-yellow-400 text-4xl mb-4">"*****"</div>
                        <h3 class="text-2xl font-bold mb-4">"Over 500+ Five Star Reviews"</h3>
                        <p class="text-blue-200 mb-6">"We take pride in every job. Check us out on Google and TrustPilot."</p>
                        <div class="flex gap-4 opacity-70">
                            <span class="font-bold">"Google"</span>
                            <span>"|"</span>
                            <span class="font-bold">"TrustPilot"</span>
                             <span>"|"</span>
                             <span class="font-bold">"Yell"</span>
                        </div>
                    </GlassCard>
                    <TestimonialCard
                        name="Emily Blunt"
                        location="Kenilworth"
                        text="Very professional. Provided a clear quote upfront and stuck to it. The new shelves actulaly look amazing."
                        rating=5
                    />
                </div>
                // Column 3
                <div class="space-y-8">
                     <TestimonialCard
                        name="David Chen"
                        location="Coventry City Centre"
                        text="Emergency call out for a stuck door. He was there within 45 mins. Lifesaver."
                        rating=5
                    />
                     <TestimonialCard
                        name="Mrs. P. Wilson"
                        location="Finham"
                        text="A polite young man who knows his trade. Fixed my garden fence after the storm. Will definitely book again."
                        rating=5
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn TestimonialCard(
    name: &'static str,
    location: &'static str,
    text: &'static str,
    rating: i32,
) -> impl IntoView {
    view! {
        <GlassCard>
            <div class="flex text-yellow-500 mb-4 text-lg">
                {(0..rating).map(|_| "*").collect::<String>()}
            </div>
            <p class="text-gray-600 leading-relaxed mb-6 italic">"\"" {text} "\""</p>
            <div class="flex items-center gap-4">
                <div class="w-10 h-10 bg-gray-200 rounded-full flex items-center justify-center font-bold text-gray-500">
                    {name.chars().next().unwrap()}
                </div>
                <div>
                    <div class="font-bold text-gray-900">{name}</div>
                    <div class="text-xs text-blue-500 font-semibold uppercase">{location}</div>
                </div>
            </div>
        </GlassCard>
    }
}
