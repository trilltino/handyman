//! Blog index page.
//!
//! Lists all blog posts with SEO optimization.

use super::data::{get_all_posts, BlogPost};
use crate::components::seo::SeoHead;
use crate::components::ui::{Button, ButtonVariant};
use leptos::prelude::*;
use leptos_meta::Script;
use shared::PageMetadata;

fn blog_list_json_ld(posts: &[BlogPost]) -> String {
    let post_items: Vec<String> = posts
        .iter()
        .map(|p| {
            format!(
                r#"{{
                "@type": "BlogPosting",
                "headline": "{}",
                "description": "{}",
                "url": "{}",
                "datePublished": "{}",
                "author": {{
                    "@type": "Organization",
                    "name": "{}"
                }}
            }}"#,
                p.title,
                p.description,
                p.canonical_url(),
                p.published_date,
                p.author
            )
        })
        .collect();

    format!(
        r#"{{
        "@context": "https://schema.org",
        "@type": "Blog",
        "name": "XF Tradesmen Blog",
        "description": "Expert marketing guides and tips for UK tradesmen. Learn how to grow your business with digital tools, SEO strategies, and website best practices.",
        "url": "https://xftradesman.com/blog",
        "publisher": {{
            "@type": "Organization",
            "name": "XF Tradesmen",
            "url": "https://xftradesman.com"
        }},
        "blogPost": [{}]
    }}"#,
        post_items.join(",")
    )
}

#[component]
pub fn BlogIndex() -> impl IntoView {
    let posts = get_all_posts();
    let json_ld = blog_list_json_ld(&posts);

    view! {
        <SeoHead metadata=PageMetadata {
            title: "Marketing Tips for Tradesmen | XF Tradesmen Blog".to_string(),
            description: "Expert marketing guides, SEO strategies, and website tips for UK tradesmen. Learn how to grow your business and get more leads online.".to_string(),
            canonical_url: Some("https://xftradesman.com/blog".to_string()),
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

            // Posts Grid
            <section class="py-24 px-4 bg-void-surface relative">
                <div class="max-w-6xl mx-auto">
                    <div class="flex items-center gap-4 mb-12">
                         <div class="h-1 w-12 bg-gradient-to-r from-brand to-transparent rounded-full"></div>
                         <h2 class="text-2xl font-bold text-white tracking-widest uppercase font-mono">"Latest Transmissions"</h2>
                    </div>

                    <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                        {posts.into_iter().map(|post| {
                            let slug = post.slug.to_string();
                            let title = post.title.to_string();
                            let excerpt = post.description.to_string();
                            let category_str = post.category.as_str().to_string();
                            let category_bg = post.category.color_class().to_string();
                            let icon_color = post.category.icon_class().to_string();
                            let date = post.published_date.to_string();
                            let read_time = post.read_time.to_string();

                            view! {
                                <article class="card-deep overflow-hidden group p-0 border-void-highlight hover:border-brand/50 transition-all duration-300">
                                    <div class="h-56 bg-void relative overflow-hidden group-hover:opacity-90 transition">
                                         <div class="absolute inset-0 bg-cyber-grid opacity-30"></div>
                                         <div class="absolute inset-0 flex items-center justify-center">
                                            <div class={format!("w-16 h-16 rounded-full flex items-center justify-center group-hover:scale-110 transition-transform duration-500 {}", icon_color)}>
                                                <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h10a2 2 0 012 2v1m2 13a2 2 0 01-2-2V7m2 13a2 2 0 002-2V9a2 2 0 00-2-2h-2m-4-3H9M7 16h6M7 8h6v4H7V8z"/>
                                                </svg>
                                            </div>
                                         </div>
                                    </div>
                                    <div class="p-8 relative">
                                        <div class="flex justify-between items-center mb-4">
                                            <span class={format!("inline-block px-2 py-1 rounded text-[10px] font-mono font-bold uppercase tracking-wider {}", category_bg)}>
                                                {category_str}
                                            </span>
                                            <div class="flex items-center gap-2 text-gray-500 text-xs font-mono">
                                                <span>{date.clone()}</span>
                                                <span>"·"</span>
                                                <span>{read_time.clone()}</span>
                                            </div>
                                        </div>

                                        <h3 class="text-xl font-bold text-white mb-4 leading-tight group-hover:text-brand-light transition font-heading">
                                            <a href={format!("/blog/{}", slug.clone())}>{title}</a>
                                        </h3>
                                        <p class="text-gray-400 text-sm mb-6 line-clamp-3 leading-relaxed">
                                            {excerpt}
                                        </p>
                                        <a href={format!("/blog/{}", slug)} class="text-white hover:text-brand-light text-xs font-bold uppercase tracking-widest flex items-center gap-2 group/link">
                                            "Read Analysis"
                                            <svg class="w-4 h-4 group-hover/link:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/>
                                            </svg>
                                        </a>
                                    </div>
                                </article>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </section>

            // Newsletter CTA
            <section class="bg-void border-t border-void-highlight py-24 px-4 overflow-hidden relative">
                 <div class="absolute inset-y-0 right-0 w-1/3 bg-gradient-to-l from-brand/5 to-transparent"></div>
                <div class="max-w-2xl mx-auto text-center relative z-10">
                    <h2 class="text-3xl font-bold text-white mb-6 font-heading">"STAY CONNECTED"</h2>
                    <p class="text-gray-400 mb-8 max-w-lg mx-auto">"Get the latest marketing intelligence for tradesmen delivered to your inbox."</p>
                    <div class="flex flex-col sm:flex-row gap-4 justify-center items-center">
                         <input type="email" placeholder="ENTER EMAIL..." class="bg-void-surface border border-void-highlight text-white px-6 py-3 rounded text-sm w-full max-w-xs focus:ring-1 focus:ring-brand focus:border-brand placeholder-gray-600 font-mono" />
                         <Button label="SUBSCRIBE" href="#" variant=ButtonVariant::Primary />
                    </div>
                </div>
            </section>
        </div>
    }
}
