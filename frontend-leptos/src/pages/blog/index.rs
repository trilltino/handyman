//! Blog index page.
//!
//! Lists all blog posts.

use crate::components::seo::SeoHead;
use crate::components::ui::{Button, ButtonVariant};
use leptos::prelude::*;
use shared::PageMetadata;

#[derive(Clone)]
struct BlogPost {
    title: &'static str,
    slug: &'static str,
    category: &'static str,
    date: &'static str,
    excerpt: &'static str,
    image_color: &'static str, // Simple placeholder for now, would be URL in real app
}

#[component]
pub fn BlogIndex() -> impl IntoView {
    let posts = vec![
        BlogPost {
            title: "Why Tradesmen with Websites Earn 40% More",
            slug: "why-tradesmen-need-websites",
            category: "Revenue Data",
            date: "2025-10-15",
            excerpt: "New 2024 statistics show the massive gap in earnings between tradesmen with a professional online presence and those without.",
            image_color: "brand",
        },
        BlogPost {
            title: "The Ultimate Guide to Local SEO for Plumbers",
            slug: "local-seo-guide",
            category: "Local SEO",
            date: "2025-10-22",
            excerpt: "Learn how to rank #1 in your local area and get the phone ringing with high-quality leads.",
            image_color: "blue-500",
        },
        BlogPost {
            title: "How to Build Instant Trust with Potential Clients",
            slug: "building-trust-online",
            category: "Trust Protocols",
            date: "2025-11-01",
            excerpt: "Your website is your first impression. Make it count with these 5 proven trust signals.",
            image_color: "green-500",
        },
    ];

    let json_ld = r#"{
      "@context": "https://schema.org",
      "@type": "Blog",
      "name": "XF Tradesmen Intelligence",
      "description": "Tactical guides for tradesmen to dominate their local market.",
      "url": "https://xftradesmen.com/blog"
    }"#;

    view! {
        <SeoHead metadata=PageMetadata {
            title: "XF Tradesmen Blog - Tips for Tradesmen | XF Tradesmen".to_string(),
            description: "Expert advice for plumbers, electricians, and handymen on growing their business online. Marketing tips, SEO guides, and website best practices.".to_string(),
            canonical_url: Some("https://xftradesmen.com/blog".to_string()),
            og_image: None,
        }/>
        <Script type_="application/ld+json">{json_ld}</Script>

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
                        {posts.into_iter().map(|post| {
                            let category_bg = match post.image_color {
                                "blue-500" => "bg-blue-500/10 text-blue-400",
                                "green-500" => "bg-green-500/10 text-green-400",
                                _ => "bg-brand/10 text-brand",
                            };

                            let icon_color = match post.image_color {
                                "blue-500" => "text-blue-500 bg-blue-500/10",
                                "green-500" => "text-green-500 bg-green-500/10",
                                _ => "text-brand bg-brand/10",
                            };

                            view! {
                                <div class="card-deep overflow-hidden group p-0 border-void-highlight hover:border-brand/50 transition-all duration-300">
                                    <div class="h-56 bg-void relative overflow-hidden group-hover:opacity-90 transition">
                                         <div class="absolute inset-0 bg-cyber-grid opacity-30"></div>
                                         <div class="absolute inset-0 flex items-center justify-center">
                                            <div class={format!("w-16 h-16 rounded-full flex items-center justify-center group-hover:scale-110 transition-transform duration-500 {}", icon_color)}>
                                                <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h10a2 2 0 012 2v1m2 13a2 2 0 01-2-2V7m2 13a2 2 0 002-2V9a2 2 0 00-2-2h-2m-4-3H9M7 16h6M7 8h6v4H7V8z"/></svg>
                                            </div>
                                         </div>
                                    </div>
                                    <div class="p-8 relative">
                                        <div class="flex justify-between items-center mb-4">
                                            <span class={format!("inline-block px-2 py-1 rounded text-[10px] font-mono font-bold uppercase tracking-wider {}", category_bg)}>
                                                {post.category}
                                            </span>
                                            <span class="text-gray-500 text-xs font-mono">{post.date}</span>
                                        </div>

                                        <h3 class="text-xl font-bold text-white mb-4 leading-tight group-hover:text-brand-light transition font-heading">
                                            <a href=format!("/blog/{}", post.slug) class="inset-0">{post.title}</a>
                                        </h3>
                                        <p class="text-gray-400 text-sm mb-6 line-clamp-3 leading-relaxed">
                                            {post.excerpt}
                                        </p>
                                        <a href=format!("/blog/{}", post.slug) class="text-white hover:text-brand-light text-xs font-bold uppercase tracking-widest flex items-center gap-2 group/link">
                                            "Read Analysis" <svg class="w-4 h-4 group-hover/link:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/></svg>
                                        </a>
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
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
