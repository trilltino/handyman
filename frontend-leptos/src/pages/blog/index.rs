//! Blog index page.
//!
//! Lists all blog posts.

use leptos::prelude::*;
use crate::components::seo::SeoHead;
use shared::PageMetadata;
use crate::components::ui::{Button, ButtonVariant};

#[component]
pub fn BlogIndex() -> impl IntoView {
    view! {
        <SeoHead metadata=PageMetadata {
            title: "XF Tradesmen Blog - Tips for Tradesmen | XF Tradesmen".to_string(),
            description: "Expert advice for plumbers, electricians, and handymen on growing their business online. Marketing tips, SEO guides, and website best practices.".to_string(),
            canonical_url: Some("https://xftradesmen.com/blog".to_string()),
            og_image: None,
        }/>

        <div class="space-y-0 overflow-x-hidden">
            // Hero
            <section class="bg-void border-b border-void-highlight text-white py-32 px-4 overflow-hidden relative">
                 <div class="absolute inset-0 bg-cyber-grid bg-[length:40px_40px] opacity-20"></div>
                 <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[800px] h-[800px] bg-brand-deep/10 blur-[120px] rounded-full pointer-events-none"></div>

                 <div class="max-w-6xl mx-auto text-center relative z-10 animate-fade-in">
                    <div class="inline-block mb-6 px-4 py-1.5 rounded-full bg-void-surface border border-void-highlight text-brand-light text-sm font-mono tracking-widest uppercase">
                        "Knowledge Base"
                    </div>
                    <h1 class="text-5xl md:text-7xl font-heading font-black mb-6 tracking-tighter">"TRADESMAN" <br/> <span class="text-transparent bg-clip-text bg-gradient-to-r from-brand-light to-brand">"INTELLIGENCE"</span></h1>
                    <p class="text-xl text-gray-400 max-w-2xl mx-auto font-light">
                        "Tactical guides to dominate your local market. SEO, conversion, and digital growth strategies."
                    </p>
                </div>
            </section>

            <section class="py-24 px-4 bg-void-surface relative">
                <div class="max-w-6xl mx-auto">
                    <div class="flex items-center gap-4 mb-12">
                         <div class="h-1 w-12 bg-gradient-to-r from-brand to-transparent rounded-full"></div>
                         <h2 class="text-2xl font-bold text-white tracking-widest uppercase font-mono">"Latest Transmissions"</h2>
                    </div>
                    
                    <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                        // Article 1
                        <div class="card-deep overflow-hidden group p-0 border-void-highlight hover:border-brand/50 transition-all duration-300">
                            <div class="h-56 bg-void relative overflow-hidden group-hover:opacity-90 transition">
                                 <div class="absolute inset-0 bg-cyber-grid opacity-30"></div>
                                 <div class="absolute inset-0 flex items-center justify-center">
                                    <div class="w-16 h-16 rounded-full bg-brand/10 flex items-center justify-center text-brand group-hover:scale-110 transition-transform duration-500">
                                        <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-9 9-4-4-6 6"/></svg>
                                    </div>
                                 </div>
                            </div>
                            <div class="p-8 relative">
                                <span class="inline-block px-2 py-1 rounded bg-brand/10 text-brand text-[10px] font-mono font-bold uppercase tracking-wider mb-4">"Revenue Data"</span>
                                <h3 class="text-xl font-bold text-white mb-4 leading-tight group-hover:text-brand-light transition font-heading">
                                    <a href="/blog/why-tradesmen-need-websites" class="inset-0">"Why Tradesmen with Websites Earn 40% More"</a>
                                </h3>
                                <p class="text-gray-400 text-sm mb-6 line-clamp-3 leading-relaxed">
                                    "New 2024 statistics show the massive gap in earnings between tradesmen with a professional online presence and those without."
                                </p>
                                <a href="/blog/why-tradesmen-need-websites" class="text-white hover:text-brand-light text-xs font-bold uppercase tracking-widest flex items-center gap-2 group/link">
                                    "Read Analysis" <svg class="w-4 h-4 group-hover/link:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/></svg>
                                </a>
                            </div>
                        </div>

                         // Article 2
                         <div class="card-deep overflow-hidden group p-0 border-void-highlight hover:border-brand/50 transition-all duration-300">
                            <div class="h-56 bg-void relative overflow-hidden group-hover:opacity-90 transition">
                                 <div class="absolute inset-0 bg-cyber-grid opacity-30"></div>
                                 <div class="absolute inset-0 flex items-center justify-center">
                                    <div class="w-16 h-16 rounded-full bg-blue-500/10 flex items-center justify-center text-blue-500 group-hover:scale-110 transition-transform duration-500">
                                        <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"/><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"/></svg>
                                    </div>
                                 </div>
                            </div>
                            <div class="p-8 relative">
                                 <span class="inline-block px-2 py-1 rounded bg-blue-500/10 text-blue-400 text-[10px] font-mono font-bold uppercase tracking-wider mb-4">"Local SEO"</span>
                                <h3 class="text-xl font-bold text-white mb-4 leading-tight group-hover:text-brand-light transition font-heading">
                                    <a href="/blog/local-seo-guide">"The Ultimate Guide to Local SEO for Plumbers"</a>
                                </h3>
                                <p class="text-gray-400 text-sm mb-6 line-clamp-3 leading-relaxed">
                                    "Learn how to rank #1 in your local area and get the phone ringing with high-quality leads."
                                </p>
                                 <a href="/blog/local-seo-guide" class="text-white hover:text-brand-light text-xs font-bold uppercase tracking-widest flex items-center gap-2 group/link">
                                    "Read Analysis" <svg class="w-4 h-4 group-hover/link:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/></svg>
                                </a>
                            </div>
                        </div>

                         // Article 3
                         <div class="card-deep overflow-hidden group p-0 border-void-highlight hover:border-brand/50 transition-all duration-300">
                            <div class="h-56 bg-void relative overflow-hidden group-hover:opacity-90 transition">
                                 <div class="absolute inset-0 bg-cyber-grid opacity-30"></div>
                                 <div class="absolute inset-0 flex items-center justify-center">
                                    <div class="w-16 h-16 rounded-full bg-green-500/10 flex items-center justify-center text-green-500 group-hover:scale-110 transition-transform duration-500">
                                        <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
                                    </div>
                                 </div>
                            </div>
                            <div class="p-8 relative">
                                 <span class="inline-block px-2 py-1 rounded bg-green-500/10 text-green-400 text-[10px] font-mono font-bold uppercase tracking-wider mb-4">"Trust Protocols"</span>
                                <h3 class="text-xl font-bold text-white mb-4 leading-tight group-hover:text-brand-light transition font-heading">
                                    <a href="/blog/building-trust-online">"How to Build Instant Trust with Potential Clients"</a>
                                </h3>
                                <p class="text-gray-400 text-sm mb-6 line-clamp-3 leading-relaxed">
                                    "Your website is your first impression. Make it count with these 5 proven trust signals."
                                </p>
                                 <a href="/blog/building-trust-online" class="text-white hover:text-brand-light text-xs font-bold uppercase tracking-widest flex items-center gap-2 group/link">
                                    "Read Analysis" <svg class="w-4 h-4 group-hover/link:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/></svg>
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            
            <section class="bg-void border-t border-void-highlight py-24 px-4 overflow-hidden relative">
                 <div class="absolute inset-y-0 right-0 w-1/3 bg-gradient-to-l from-brand/5 to-transparent"></div>
                <div class="max-w-2xl mx-auto text-center relative z-10">
                    <h2 class="text-3xl font-bold text-white mb-6 font-heading">"STAY CONNECTED"</h2>
                    <p class="text-gray-400 mb-8 max-w-lg mx-auto">"Get the latest marketing intelligence for tradesmen delivered to your inbox."</p>
                    <div class="flex flex-col sm:flex-row gap-4 justify-center items-center">
                         <input type="email" placeholder="ENTER EMAIL FREQUENCY..." class="bg-void-surface border border-void-highlight text-white px-6 py-3 rounded text-sm w-full max-w-xs focus:ring-1 focus:ring-brand focus:border-brand placeholder-gray-600 font-mono" />
                         <Button label="SUBSCRIBE" href="#" variant=ButtonVariant::Primary />
                    </div>
                </div>
            </section>
        </div>
    }
}
