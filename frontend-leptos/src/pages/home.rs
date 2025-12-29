//! Homepage component.
//!
//! Main landing page with hero, features, and CTA sections.

use crate::components::seo::{LocalBusinessSchema, SeoHead};
use leptos::prelude::*;
use shared::PageMetadata;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <SeoHead metadata=PageMetadata::for_homepage() />
        <LocalBusinessSchema />

        <div class="bg-grain min-h-screen text-white font-sans selection:bg-brand selection:text-white overflow-x-hidden">
            // Fixed Edge Glows for Ambient depth
            <div class="edge-glow-tl opacity-20 md:opacity-30"></div>
            <div class="edge-glow-br opacity-20 md:opacity-30"></div>

            // HERO SECTION
            <section class="relative bg-bold-void min-h-screen flex items-center pt-20 pb-32 px-6 overflow-hidden">
                <div class="relative max-w-7xl mx-auto text-center z-10 animate-fade-in">
                    <h1 class="text-6xl md:text-8xl lg:text-[10rem] font-black tracking-tighter mb-8 leading-[0.9] uppercase">
                        "GROW YOUR" <br/>
                        <span class="text-transparent bg-clip-text bg-gradient-to-b from-white via-white to-gray-500">
                            "TRADESMAN BUSINESS IN 2026."
                        </span>
                    </h1>

                    <p class="text-xl md:text-2xl text-gray-400 max-w-4xl mx-auto leading-relaxed font-medium">
                        "Most agency sites are generic. We build "
                        <span class="text-white font-bold">"Visibility Engines"</span>
                        " tailored to the specific jobs that keep your business profitable."
                    </p>
                </div>

                // Background floating elements for parallax feel
                <div class="absolute top-[20%] right-[10%] w-64 h-64 bg-red-500/20 blur-3xl rounded-full animate-pulse-slow"></div>
                <div class="absolute bottom-[20%] left-[5%] w-96 h-96 bg-orange-600/15 blur-[120px] rounded-full"></div>
                <div class="absolute top-[50%] left-[15%] w-80 h-80 bg-red-600/10 blur-[100px] rounded-full"></div>
            </section>

            // FEATURES SECTION - Completely black with white text
            <section class="py-32 px-6 bg-black">
                <div class="max-w-7xl mx-auto">
                    <div class="grid md:grid-cols-2 gap-12 items-center mb-24">
                        <h2 class="text-3xl md:text-4xl lg:text-5xl font-black tracking-tight leading-tight text-white">
                            "Built to Expand Your Business."<br/>"Connect With Your Customers."
                        </h2>
                        <p class="text-lg font-medium leading-relaxed text-gray-400">
                            "We don't do 'cowboy' websites that break the second you look at them. We build bulletproof local presences that look as professional as the work you do. We optimize every tag and link so you dominate the local search, and we provide 24/7 monitoring to make sure your doors are always open for business—all with a 36-hour handover."
                        </p>
                    </div>

                    <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
                        // Card 1
                        <div class="group relative bg-gradient-to-br from-white/5 to-transparent p-8 rounded-2xl border border-white/10 hover:border-brand/50 transition-all duration-500 hover:transform hover:scale-[1.02] hover:shadow-2xl hover:shadow-brand/10">
                            <div class="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-brand via-brand-light to-transparent rounded-t-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
                            <div class="flex items-center gap-4 mb-6">
                                <span class="w-12 h-12 rounded-xl bg-brand/20 flex items-center justify-center text-brand font-black text-lg">"01"</span>
                                <span class="text-white/50 font-black text-xs tracking-widest uppercase">"LOCAL VISIBILITY"</span>
                            </div>
                            <h3 class="text-2xl font-bold mb-6 text-white group-hover:text-brand transition-colors duration-300">"Be Found Locally"</h3>
                            <ul class="space-y-3">
                                <li class="flex items-start gap-3">
                                    <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                    <span class="text-gray-400 text-sm">"Rank higher in local Google searches"</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                    <span class="text-gray-400 text-sm">"Target your specific service areas"</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                    <span class="text-gray-400 text-sm">"Customers find YOU, not competitors"</span>
                                </li>
                            </ul>
                        </div>

                        // Card 2
                        <div class="group relative bg-gradient-to-br from-white/5 to-transparent p-8 rounded-2xl border border-white/10 hover:border-brand/50 transition-all duration-500 hover:transform hover:scale-[1.02] hover:shadow-2xl hover:shadow-brand/10">
                            <div class="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-brand via-brand-light to-transparent rounded-t-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
                            <div class="flex items-center gap-4 mb-6">
                                <span class="w-12 h-12 rounded-xl bg-brand/20 flex items-center justify-center text-brand font-black text-lg">"02"</span>
                                <span class="text-white/50 font-black text-xs tracking-widest uppercase">"GOOGLE MAPS"</span>
                            </div>
                            <h3 class="text-2xl font-bold mb-6 text-white group-hover:text-brand transition-colors duration-300">"Map Pack"</h3>
                            <ul class="space-y-3">
                                <li class="flex items-start gap-3">
                                    <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                    <span class="text-gray-400 text-sm">"Fully optimized Google integration"</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                    <span class="text-gray-400 text-sm">"Appear in 'Near Me' searches"</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                    <span class="text-gray-400 text-sm">"Collect solid 5-star reviews"</span>
                                </li>
                            </ul>
                        </div>

                        // Card 3
                        <div class="group relative bg-gradient-to-br from-white/5 to-transparent p-8 rounded-2xl border border-white/10 hover:border-brand/50 transition-all duration-500 hover:transform hover:scale-[1.02] hover:shadow-2xl hover:shadow-brand/10">
                            <div class="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-brand via-brand-light to-transparent rounded-t-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
                            <div class="flex items-center gap-4 mb-6">
                                <span class="w-12 h-12 rounded-xl bg-brand/20 flex items-center justify-center text-brand font-black text-lg">"03"</span>
                                <span class="text-white/50 font-black text-xs tracking-widest uppercase">"GROWTH"</span>
                            </div>
                            <h3 class="text-2xl font-bold mb-6 text-white group-hover:text-brand transition-colors duration-300">"Built to Scale"</h3>
                            <ul class="space-y-3">
                                <li class="flex items-start gap-3">
                                    <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                    <span class="text-gray-400 text-sm">"Ready for new services anytime"</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                    <span class="text-gray-400 text-sm">"Handles increased traffic easily"</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                    <span class="text-gray-400 text-sm">"Expand your service area painlessly"</span>
                                </li>
                            </ul>
                        </div>

                        // Card 4
                        <div class="group relative bg-gradient-to-br from-white/5 to-transparent p-8 rounded-2xl border border-white/10 hover:border-brand/50 transition-all duration-500 hover:transform hover:scale-[1.02] hover:shadow-2xl hover:shadow-brand/10">
                            <div class="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-brand via-brand-light to-transparent rounded-t-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
                            <div class="flex items-center gap-4 mb-6">
                                <span class="w-12 h-12 rounded-xl bg-brand/20 flex items-center justify-center text-brand font-black text-lg">"04"</span>
                                <span class="text-white/50 font-black text-xs tracking-widest uppercase">"ZERO HASSLE"</span>
                            </div>
                            <h3 class="text-2xl font-bold mb-6 text-white group-hover:text-brand transition-colors duration-300">"We Handle It All"</h3>
                            <ul class="space-y-3">
                                <li class="flex items-start gap-3">
                                    <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                    <span class="text-gray-400 text-sm">"Domain & hosting managed for you"</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                    <span class="text-gray-400 text-sm">"24/7 Security & Backups included"</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                    <span class="text-gray-400 text-sm">"We make updates, you do the work"</span>
                                </li>
                            </ul>
                        </div>

                        // Card 5
                        <div class="group relative bg-gradient-to-br from-white/5 to-transparent p-8 rounded-2xl border border-white/10 hover:border-brand/50 transition-all duration-500 hover:transform hover:scale-[1.02] hover:shadow-2xl hover:shadow-brand/10">
                            <div class="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-brand via-brand-light to-transparent rounded-t-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
                            <div class="flex items-center gap-4 mb-6">
                                <span class="w-12 h-12 rounded-xl bg-brand/20 flex items-center justify-center text-brand font-black text-lg">"05"</span>
                                <span class="text-white/50 font-black text-xs tracking-widest uppercase">"CREDIBILITY"</span>
                            </div>
                            <h3 class="text-2xl font-bold mb-6 text-white group-hover:text-brand transition-colors duration-300">"Win More Trust"</h3>
                            <ul class="space-y-3">
                                <li class="flex items-start gap-3">
                                    <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                    <span class="text-gray-400 text-sm">"Clean, modern professional design"</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                    <span class="text-gray-400 text-sm">"Showcase your portfolio expertly"</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                    <span class="text-gray-400 text-sm">"Look bigger & better than the rest"</span>
                                </li>
                            </ul>
                        </div>

                        // Card 6
                        <div class="group relative bg-gradient-to-br from-white/5 to-transparent p-8 rounded-2xl border border-white/10 hover:border-brand/50 transition-all duration-500 hover:transform hover:scale-[1.02] hover:shadow-2xl hover:shadow-brand/10">
                            <div class="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-brand via-brand-light to-transparent rounded-t-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
                            <div class="flex items-center gap-4 mb-6">
                                <span class="w-12 h-12 rounded-xl bg-brand/20 flex items-center justify-center text-brand font-black text-lg">"06"</span>
                                <span class="text-white/50 font-black text-xs tracking-widest uppercase">"LEADS"</span>
                            </div>
                            <h3 class="text-2xl font-bold mb-6 text-white group-hover:text-brand transition-colors duration-300">"Turn Clicks to Calls"</h3>
                            <ul class="space-y-3">
                                <li class="flex items-start gap-3">
                                    <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                    <span class="text-gray-400 text-sm">"'Call Now' buttons that actually work"</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                    <span class="text-gray-400 text-sm">"Easy quote request forms"</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                    <span class="text-gray-400 text-sm">"Fast loading for busy customers"</span>
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>
            </section>

            // MOCKUP SHOWCASE SECTION - 3D Angled Display
            <section class="py-32 px-6 bg-gradient-to-b from-black via-gray-950 to-black relative overflow-hidden">
                // Background glow
                <div class="absolute inset-0 flex items-center justify-center">
                    <div class="w-[800px] h-[400px] bg-brand/10 blur-[150px] rounded-full"></div>
                </div>

                <div class="max-w-7xl mx-auto relative z-10">
                    <div class="mb-12 text-center relative z-20">
                        <h2 class="text-5xl md:text-7xl font-black tracking-tighter leading-none uppercase mb-6 text-white">
                            "WEBSITE PACKAGES"
                        </h2>
                        <a href="/terms" class="inline-block text-xs text-brand font-bold uppercase tracking-widest hover:text-white transition-colors border-b border-brand/30 hover:border-white pb-1 mb-12">
                            "Terms & Conditions"
                        </a>


                    </div>

                    // 3D Angled Stack Container
                    <div class="relative min-h-[1400px] flex justify-center items-center perspective-[2500px] py-12">
                        // Stack Group - Smaller Max Width
                        <div class="relative w-full max-w-2xl h-[600px] preserve-3d">

                            // Bottom Card (Roofer)
                            <div class="absolute top-0 left-0 w-full transform transition-all duration-1000 ease-out hover:translate-x-2 hover:translate-y-2 reveal-on-scroll delay-100"
                                 style="transform: rotateX(20deg) rotateZ(-10deg) skewY(5deg) translateZ(-80px) translateX(40px) translateY(40px);">
                                <div class="relative overflow-hidden rounded-lg shadow-2xl shadow-black aspect-video bg-gray-900 border border-white/5">
                                    <img src="/images/mockups/roofer.png" alt="Roofer Website" class="w-full h-full object-cover object-top opacity-60 hover:opacity-100 transition-opacity duration-500"/>
                                </div>
                            </div>

                            // Middle Card (Carpenter)
                            <div class="absolute top-0 left-0 w-full transform transition-all duration-1000 ease-out hover:translate-x-1 hover:translate-y-1 reveal-on-scroll delay-300"
                                 style="transform: rotateX(20deg) rotateZ(-10deg) skewY(5deg) translateZ(-40px) translateX(20px) translateY(20px);">
                                <div class="relative overflow-hidden rounded-lg shadow-2xl shadow-black aspect-video bg-gray-900 border border-white/5">
                                    <img src="/images/mockups/carpenter.png" alt="Carpenter Website" class="w-full h-full object-cover object-top opacity-80 hover:opacity-100 transition-opacity duration-500"/>
                                </div>
                            </div>

                            // Top Card (Plumber)
                            <div class="absolute top-0 left-0 w-full transform transition-all duration-1000 ease-out z-10 reveal-on-scroll delay-500"
                                 style="transform: rotateX(20deg) rotateZ(-10deg) skewY(5deg);">
                                <div class="relative overflow-hidden rounded-lg shadow-2xl shadow-black aspect-video bg-gray-900 border border-white/5">
                                    <img src="/images/mockups/plumber.png" alt="Plumber Website" class="w-full h-full object-cover object-top"/>
                                </div>
                            </div>

                        </div>
                    </div>
                </div>
            </section>

            // AREAS SECTION (Sleek minimalist marquee)
            <section class="bg-black py-20 px-6">
                <div class="max-w-7xl mx-auto flex flex-col items-center justify-center gap-8 text-center">
                    <span class="text-sm font-black text-white/30 uppercase tracking-[0.3em] whitespace-nowrap">"OPERATING IN"</span>
                    <div class="flex flex-wrap justify-center gap-x-8 gap-y-4 text-gray-600 font-bold uppercase text-xs tracking-widest">
                        <span class="hover:text-white transition-colors cursor-default">"London"</span>
                        <span class="hover:text-white transition-colors cursor-default">"Manchester"</span>
                        <span class="hover:text-white transition-colors cursor-default">"Birmingham"</span>
                        <span class="hover:text-white transition-colors cursor-default">"Liverpool"</span>
                        <span class="hover:text-white transition-colors cursor-default">"Bristol"</span>
                        <span class="hover:text-white transition-colors cursor-default">"Dartford"</span>
                        <span class="hover:text-white transition-colors cursor-default">"Coventry"</span>
                        <span class="hover:text-white transition-colors cursor-default">"Leeds"</span>
                        <span class="hover:text-white transition-colors cursor-default">"Kenilworth"</span>
                        <span class="hover:text-white transition-colors cursor-default">"Leamington"</span>
                        <span class="hover:text-white transition-colors cursor-default">"Warwick"</span>
                    </div>
                </div>
            </section>
        </div>
    }
}
