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

        // Local wrapper (was background wrapper)
        // Kept div for structure but removed BG classes
        <div>
            // Fixed Edge Glows for Ambient depth
            <div class="edge-glow-tl opacity-20 md:opacity-30"></div>
            <div class="edge-glow-br opacity-20 md:opacity-30"></div>

            // HERO SECTION - transparent to show parent gradient
            <section class="relative min-h-screen flex items-center pt-20 pb-32 px-6 overflow-hidden">
                <div class="relative max-w-7xl mx-auto text-center z-10 animate-fade-in">
                    <h1 class="text-6xl md:text-8xl lg:text-[10rem] font-black tracking-tighter mb-8 leading-[0.9] uppercase">
                        <span class="text-transparent bg-clip-text bg-gradient-to-b from-white via-white to-gray-500">
                            "EXPAND YOUR TRADE IN 2026."
                        </span>
                    </h1>

                    <p class="text-xl md:text-2xl text-gray-400 max-w-4xl mx-auto leading-relaxed font-medium">
                        <span class="text-white font-bold">"72 HRS from signature to live."</span>
                        " We engineer websites built for trust and growth, show your good work off."
                    </p>
                </div>

                // Background floating elements for parallax feel
                <div class="absolute top-[20%] right-[10%] w-64 h-64 bg-red-500/20 blur-3xl rounded-full animate-pulse-slow"></div>
                <div class="absolute bottom-[20%] left-[5%] w-96 h-96 bg-orange-600/15 blur-[120px] rounded-full"></div>
                <div class="absolute top-[50%] left-[15%] w-80 h-80 bg-red-600/10 blur-[100px] rounded-full"></div>
            </section>

            // FEATURES SECTION - transparent to show parent gradient
            <section class="py-32 px-6">
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

            // WEBSITE PACKAGES SECTION
            <section class="py-24 px-6 relative overflow-hidden">
                // Background glow
                <div class="absolute inset-0 flex items-center justify-center">
                    <div class="w-[800px] h-[400px] bg-brand/10 blur-[150px] rounded-full"></div>
                </div>

                <div class="max-w-7xl mx-auto relative z-10">
                    <div class="mb-12 text-center">
                        <h2 class="text-5xl md:text-7xl font-black tracking-tighter leading-none uppercase mb-6 text-white">
                            "WEBSITE PACKAGES"
                        </h2>
                        <a href="/terms" class="inline-block text-xs text-brand font-bold uppercase tracking-widest hover:text-white transition-colors border-b border-brand/30 hover:border-white pb-1">
                            "Terms & Conditions"
                        </a>
                    </div>

                    // Pricing Card
                    <div class="max-w-4xl mx-auto mt-12">
                        <div class="group relative bg-gradient-to-br from-white/10 to-transparent p-8 md:p-12 rounded-3xl border border-white/10 hover:border-brand/50 transition-all duration-500 shadow-2xl overflow-hidden">
                            // Glowing background effect
                            <div class="absolute top-0 right-0 w-96 h-96 bg-brand/10 blur-[100px] rounded-full -translate-y-1/2 translate-x-1/2"></div>

                            <div class="relative z-10 flex flex-col md:flex-row gap-12 items-center">
                                // Price Side
                                <div class="w-full md:w-1/3 text-center md:text-left border-b md:border-b-0 md:border-r border-white/10 pb-8 md:pb-0 md:pr-8">
                                    <h3 class="text-brand font-black text-lg tracking-widest uppercase mb-2">"Zero Down Plan"</h3>
                                    <div class="flex items-baseline justify-center md:justify-start gap-1 mb-4">
                                        <span class="text-4xl font-bold text-white">"£"</span>
                                        <span class="text-7xl font-black text-white tracking-tighter">"120"</span>
                                        <span class="text-gray-400 font-medium">"/mo"</span>
                                    </div>
                                    <div class="inline-block bg-brand/20 text-brand font-bold px-4 py-2 rounded-full text-sm mb-6">
                                        "£0 Upfront Cost"
                                    </div>
                                    <p class="text-gray-400 text-sm italic">"1-month free trial included"</p>
                                </div>

                                // Features Side
                                <div class="w-full md:w-2/3">
                                    <h4 class="text-2xl font-bold text-white mb-6">"Everything You Need To Grow"</h4>

                                    <div class="grid sm:grid-cols-2 gap-x-8 gap-y-4 mb-8">
                                        <div class="space-y-4">
                                            <div class="flex items-start gap-3">
                                                <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                                <span class="text-gray-300 text-sm">"Custom Coding & Development"</span>
                                            </div>
                                            <div class="flex items-start gap-3">
                                                <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                                <span class="text-gray-300 text-sm">"Mobile & SEO Optimization"</span>
                                            </div>
                                            <div class="flex items-start gap-3">
                                                <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                                <span class="text-gray-300 text-sm">"High-Speed Hosting"</span>
                                            </div>
                                            <div class="flex items-start gap-3">
                                                <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                                <span class="text-gray-300 text-sm">"Domain Management (12 Mo)"</span>
                                            </div>
                                        </div>

                                        <div class="space-y-4">
                                            <div class="flex items-start gap-3">
                                                <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                                <span class="text-gray-300 text-sm">"Daily Backups & Security"</span>
                                            </div>
                                            <div class="flex items-start gap-3">
                                                <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                                <span class="text-gray-300 text-sm">"SSL Certificate Renewal"</span>
                                            </div>
                                            <div class="flex items-start gap-3">
                                                <svg class="w-5 h-5 text-brand shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                                <span class="text-gray-300 text-sm">"24/7 Uptime Monitoring"</span>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="bg-white/5 rounded-xl p-4 border border-white/5">
                                        <div class="flex items-center gap-3 mb-2">
                                            <svg class="w-5 h-5 text-yellow-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                            <span class="text-white font-bold text-sm">"Included Support"</span>
                                        </div>
                                        <p class="text-gray-400 text-xs leading-relaxed">
                                            "Includes up to 2 hours/month of expert development time for content updates, text changes, or image swaps."
                                        </p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // AREAS SECTION - transparent to show parent gradient
            <section class="py-20 px-6">
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
