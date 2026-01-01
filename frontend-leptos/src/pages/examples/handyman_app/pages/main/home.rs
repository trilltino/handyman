//! Handyman Home Page.

use crate::components::seo::{LocalBusinessSchema, SeoHead};
use leptos::prelude::*;
use leptos_router::components::A;
use shared::PageMetadata;

#[component]
pub fn HandymanHome() -> impl IntoView {
    view! {
        <SeoHead metadata=PageMetadata {
            title: "Coventry Handyman Services | Reliable & Local Examples".to_string(),
            description: "Meet Rick, the renowned local handyman making home repairs simple again. 10 years experience, affordable rates, attentive workmanship.".to_string(),
            canonical_url: Some("https://xftradesman.com/handyman-coventry".to_string()),
            og_image: Some("/images/hero_assets/hero_bg.png".to_string()),
        }/>
        <LocalBusinessSchema />

        <div class="font-sans antialiased text-slate-900 bg-white">
            // Hero Section
            <section class="relative min-h-[85vh] flex items-center overflow-hidden">
                // Background Image
                <div class="absolute inset-0 z-0">
                    <img
                        src="/images/hero_assets/hero_bg.png"
                        alt="Rick the Handyman"
                        class="w-full h-full object-cover object-center"
                    />
                    // Strong Blue Overlay (Handleman style)
                    <div class="absolute inset-0 bg-blue-900/80 mix-blend-multiply"></div>
                    <div class="absolute inset-0 bg-black/30"></div>
                </div>

                // Content
                <div class="relative z-10 max-w-7xl mx-auto px-6 w-full pt-20">
                    <div class="max-w-4xl">
                        <h1 class="text-5xl md:text-7xl font-black !text-white leading-[1.1] mb-8 drop-shadow-xl font-heading">
                            "Meet the renown local handyman everyone is Talking about!"
                        </h1>
                        <p class="text-xl md:text-3xl !text-white font-medium leading-relaxed mb-12 max-w-3xl drop-shadow-md">
                            "Rick makes fixing things around your home simple again with 10 years of experience in the local area enjoy repairs at affordable rates with attentive workmanship"
                        </p>

                        <div class="flex flex-col sm:flex-row gap-6">
                            <A href="/handyman-coventry/booking" attr:class="inline-block px-10 py-5 bg-yellow-400 text-blue-900 text-lg font-black uppercase tracking-wide rounded hover:bg-yellow-300 transition shadow-xl hover:scale-105 transform duration-200">
                                "Get Your Free Quote"
                            </A>

                        </div>
                    </div>
                </div>
            </section>

            // 3. What Sets Us Apart (Deep Blue Section)
            <section class="bg-blue-900 py-20 px-6 text-white border-t border-blue-800">
                <div class="max-w-7xl mx-auto">
                    <div class="grid md:grid-cols-4 gap-12 text-center md:text-left">
                        // Feature 1
                        <div class="space-y-4">
                            <div class="w-16 h-16 bg-yellow-400 rounded-lg flex items-center justify-center text-blue-900 text-3xl mb-4 mx-auto md:mx-0 shadow-lg">
                                <span class="font-black">"10"</span>
                            </div>
                            <h3 class="text-2xl font-bold">"Years Experience"</h3>
                            <p class="text-blue-200 leading-relaxed">"Decades of hands-on problem solving in homes just like yours."</p>
                        </div>
                        // Feature 2
                         <div class="space-y-4">
                            <div class="w-16 h-16 bg-yellow-400 rounded-lg flex items-center justify-center text-blue-900 text-3xl mb-4 mx-auto md:mx-0 shadow-lg">
                                <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
                            </div>
                            <h3 class="text-2xl font-bold">"Fully Insured"</h3>
                            <p class="text-blue-200 leading-relaxed">"Total peace of mind with comprehensive liability coverage."</p>
                        </div>
                        // Feature 3
                         <div class="space-y-4">
                            <div class="w-16 h-16 bg-yellow-400 rounded-lg flex items-center justify-center text-blue-900 text-3xl mb-4 mx-auto md:mx-0 shadow-lg">
                                <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
                            </div>
                            <h3 class="text-2xl font-bold">"Punctual"</h3>
                            <p class="text-blue-200 leading-relaxed">"We respect your time. We turn up when we say we will."</p>
                        </div>
                        // Feature 4
                         <div class="space-y-4">
                            <div class="w-16 h-16 bg-yellow-400 rounded-lg flex items-center justify-center text-blue-900 text-3xl mb-4 mx-auto md:mx-0 shadow-lg">
                                <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
                            </div>
                            <h3 class="text-2xl font-bold">"Friendly Service"</h3>
                            <p class="text-blue-200 leading-relaxed">"Professionalism with a smile. No jargon, just results."</p>
                        </div>
                    </div>
                </div>
            </section>

            // 4. Problem / Solution Split
            <section class="py-24 bg-white overflow-hidden">
                <div class="max-w-7xl mx-auto px-6">
                    <div class="flex flex-col md:flex-row items-center gap-16">
                        // Image Side
                        <div class="w-full md:w-1/2 relative group">
                            <div class="absolute -inset-4 bg-yellow-400 rounded-2xl transform rotate-2 group-hover:rotate-1 transition duration-300"></div>
                            <img
                                src="/images/hero_assets/problem.png"
                                alt="Broken Hinge"
                                class="relative rounded-xl shadow-2xl w-full object-cover transform -rotate-1 group-hover:rotate-0 transition duration-300 border-4 border-white"
                            />
                        </div>

                        // Text Side
                        <div class="w-full md:w-1/2 space-y-8">
                            <div>
                                <h3 class="text-blue-600 font-bold uppercase tracking-widest mb-2">"Is your home falling apart?"</h3>
                                <h2 class="text-5xl font-black text-slate-900 leading-tight">"Small Repairs shouldn't be a Big Headache."</h2>
                            </div>
                            <p class="text-slate-600 text-lg leading-relaxed">
                                "We know the feeling. That wobbly cabinet door, the dripping tap, or the flat-pack boxes piling up in the corner. You're too busy, or maybe you just don't have the right tools."
                            </p>
                            <p class="text-slate-900 text-xl font-medium">
                                "That's where Rick comes in. We tackle the jobs you hate, so you can love your home again."
                            </p>
                            <div class="pt-4">
                                <A href="/handyman-coventry/booking" attr:class="inline-flex items-center gap-2 text-blue-700 font-bold text-lg hover:gap-4 transition-all">
                                    "See What We Fix" <span class="bg-blue-100 p-1 rounded-full"><svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/></svg></span>
                                </A>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // 5. Meet Rick Section
             <section class="py-24 bg-slate-50">
                <div class="max-w-5xl mx-auto px-6 text-center">
                    <div class="w-48 h-48 mx-auto mb-8 relative">
                         <div class="absolute inset-0 bg-blue-600 rounded-full animate-pulse opacity-20"></div>
                         <img src="/images/hero_assets/rick.png" alt="Rick" class="w-full h-full object-cover rounded-full border-4 border-white shadow-xl relative z-10" />
                         <div class="absolute bottom-2 right-2 bg-yellow-400 text-blue-900 font-bold px-3 py-1 rounded-full text-sm z-20 shadow-sm">"Owner"</div>
                    </div>
                    <h2 class="text-4xl font-black text-slate-900 mb-6">"Hi, I'm Rick."</h2>
                    <p class="text-xl text-slate-600 max-w-2xl mx-auto leading-relaxed mb-10">
                        "I started this business with a simple mission directly from my garage in Coventry: to provide honest, high-quality work without the 'tradesman markup'. When you call, you speak to me. When I work, I treat your home like my own."
                    </p>
                </div>
            </section>


        </div>
    }
}
