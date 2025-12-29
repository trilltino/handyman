//! Blog article page.
//!
//! Individual blog post view with dynamic routing and full SEO.

use super::data::{get_post_by_slug, get_related_posts};
use crate::components::seo::SeoHead;
use leptos::prelude::*;
use leptos_meta::Script;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use shared::PageMetadata;

#[derive(PartialEq, Clone, Params)]
pub struct BlogParams {
    pub slug: Option<String>,
}

#[component]
pub fn BlogArticle() -> impl IntoView {
    let params = use_params::<BlogParams>();

    let slug = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|p| p.slug.clone())
            .unwrap_or_default()
    };

    view! {
        {move || {
            let current_slug = slug();
            if let Some(post) = get_post_by_slug(&current_slug) {
                let title = format!("{} | XF Tradesmen", post.title);
                let description = post.description.to_string();
                let canonical = post.canonical_url();
                let category_str = post.category.as_str().to_string();
                let category_class = post.category.color_class().to_string();
                let published = post.published_date.to_string();
                let read_time = post.read_time.to_string();
                let author = post.author.to_string();
                let content = post.content.to_string();
                let headline = post.title.to_string();
                let keywords = post.keywords.join(", ");

                let json_ld = format!(
                    r#"{{
                    "@context": "https://schema.org",
                    "@type": "BlogPosting",
                    "headline": "{}",
                    "description": "{}",
                    "url": "{}",
                    "datePublished": "{}",
                    "author": {{
                        "@type": "Organization",
                        "name": "{}"
                    }},
                    "publisher": {{
                        "@type": "Organization",
                        "name": "XF Tradesmen",
                        "url": "https://xftradesman.com"
                    }},
                    "keywords": "{}"
                }}"#,
                    headline,
                    description,
                    canonical,
                    published,
                    author,
                    keywords
                );

                let related = get_related_posts(&current_slug, 2);

                view! {
                    <SeoHead metadata=PageMetadata {
                        title: title.clone(),
                        description: description.clone(),
                        canonical_url: Some(canonical),
                        og_image: None,
                    }/>
                    <Script type_="application/ld+json">{json_ld}</Script>

                    <article class="bg-void min-h-screen text-white pt-32 pb-20 relative overflow-hidden">
                        <div class="absolute inset-0 bg-cyber-grid bg-[length:30px_30px] opacity-10 pointer-events-none"></div>

                        <div class="max-w-3xl mx-auto px-4 relative z-10">
                            // Breadcrumb & Header
                            <div class="mb-12 animate-fade-in">
                                <nav class="mb-8" aria-label="Breadcrumb">
                                    <ol class="flex items-center gap-2 text-xs font-mono text-gray-500">
                                        <li>
                                            <a href="/" class="hover:text-brand transition-colors">"Home"</a>
                                        </li>
                                        <li>
                                            <svg class="w-3 h-3 mx-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                                            </svg>
                                        </li>
                                        <li>
                                            <a href="/blog" class="hover:text-brand transition-colors">"Blog"</a>
                                        </li>
                                        <li>
                                            <svg class="w-3 h-3 mx-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                                            </svg>
                                        </li>
                                        <li class="text-gray-400 truncate max-w-[200px]">{headline.clone()}</li>
                                    </ol>
                                </nav>

                                <div class="flex items-center gap-4 mb-6">
                                    <span class={format!("px-2 py-1 rounded text-[10px] font-mono uppercase tracking-widest {}", category_class)}>
                                        {category_str}
                                    </span>
                                    <span class="text-gray-500 text-xs font-mono uppercase">{published.clone()}</span>
                                </div>

                                <h1 class="text-4xl md:text-6xl font-heading font-black mt-4 mb-8 leading-[1.1]">
                                    {headline}
                                </h1>

                                <div class="flex items-center gap-4 text-xs text-brand-light border-b border-void-highlight pb-8 font-mono">
                                    <span class="flex items-center gap-2">
                                        <span class="w-1 h-1 bg-brand rounded-full"></span>
                                        {read_time} " READ"
                                    </span>
                                    <span class="flex items-center gap-2">
                                        <span class="w-1 h-1 bg-brand rounded-full"></span>
                                        "BY " {author}
                                    </span>
                                </div>
                            </div>

                            // Article Content
                            <div
                                class="prose prose-invert prose-lg max-w-none prose-headings:font-heading prose-headings:font-bold prose-a:text-brand prose-a:no-underline hover:prose-a:text-brand-light prose-blockquote:border-l-brand prose-blockquote:bg-void-surface prose-blockquote:py-2 prose-blockquote:px-6 prose-blockquote:not-italic prose-blockquote:rounded-r-lg prose-li:marker:text-brand prose-strong:text-white"
                                inner_html=content
                            />

                            // Related Posts
                            <div class="mt-20 pt-12 border-t border-void-highlight">
                                <h2 class="text-xl font-bold text-white mb-8 font-heading tracking-wider uppercase">"Related Intelligence"</h2>
                                <div class="grid md:grid-cols-2 gap-6">
                                    {related.into_iter().map(|related_post| {
                                        let r_slug = related_post.slug.to_string();
                                        let r_title = related_post.title.to_string();
                                        let r_category = related_post.category.as_str().to_string();
                                        let r_category_class = related_post.category.color_class().to_string();
                                        let r_read_time = related_post.read_time.to_string();

                                        view! {
                                            <a href={format!("/blog/{}", r_slug)} class="card-deep p-6 group hover:border-brand/50 transition-all">
                                                <span class={format!("inline-block px-2 py-1 rounded text-[10px] font-mono font-bold uppercase tracking-wider mb-3 {}", r_category_class)}>
                                                    {r_category}
                                                </span>
                                                <h3 class="text-lg font-bold text-white group-hover:text-brand-light transition font-heading leading-tight">
                                                    {r_title}
                                                </h3>
                                                <span class="text-gray-500 text-xs font-mono mt-2 block">{r_read_time} " read"</span>
                                            </a>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>

                            // CTA
                            <div class="mt-20 card-deep border-brand/20 p-10 text-center relative overflow-hidden group">
                                <div class="absolute inset-0 bg-brand/5 group-hover:bg-brand/10 transition duration-500"></div>
                                <div class="relative z-10">
                                    <h3 class="text-2xl font-bold mb-4 font-heading text-white">"Deploy Your Digital Presence"</h3>
                                    <p class="text-gray-400 mb-8 max-w-lg mx-auto">"Join 500+ tradesmen who are dominating their local markets."</p>
                                    <a href="/pricing" class="btn-primary inline-flex">
                                        "Start Free Trial"
                                    </a>
                                </div>
                            </div>
                        </div>
                    </article>
                }.into_any()
            } else {
                // 404 state
                view! {
                    <SeoHead metadata=PageMetadata {
                        title: "Article Not Found | XF Tradesmen".to_string(),
                        description: "The requested article could not be found.".to_string(),
                        canonical_url: Some("https://xftradesman.com/blog".to_string()),
                        og_image: None,
                    }/>

                    <article class="bg-void min-h-screen text-white pt-32 pb-20 relative overflow-hidden">
                        <div class="absolute inset-0 bg-cyber-grid bg-[length:30px_30px] opacity-10 pointer-events-none"></div>
                        <div class="max-w-3xl mx-auto px-4 relative z-10 text-center">
                            <h1 class="text-4xl font-heading font-black mb-4">"Article Not Found"</h1>
                            <p class="text-gray-400 mb-8">"The article you're looking for doesn't exist."</p>
                            <a href="/blog" class="btn-primary inline-flex">
                                "Back to Blog"
                            </a>
                        </div>
                    </article>
                }.into_any()
            }
        }}
    }
}
