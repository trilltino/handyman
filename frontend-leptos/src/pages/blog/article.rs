//! Blog article page.
//!
//! Individual blog post view with dynamic routing.

use crate::components::seo::SeoHead;
use leptos::prelude::*;
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

    // In a real app, we would fetch data based on the slug.
    // For now, we'll just show a generic template or switch based on slug mock.

    let slug = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|p| p.slug.clone())
            .unwrap_or_default()
    };

    let title = move || match slug().as_str() {
        "why-tradesmen-need-websites" => {
            "Why Tradesmen with Websites Earn 40% More - XF Tradesmen".to_string()
        }
        "local-seo-guide" => "Ultimate Guide to Local SEO for Plumbers - XF Tradesmen".to_string(),
        "building-trust-online" => {
            "How to Build Trust with Clients Online - XF Tradesmen".to_string()
        }
        _ => "Blog Article - XF Tradesmen".to_string(),
    };

    view! {
        <SeoHead metadata=PageMetadata {
            title: title(),
            description: "Read our latest expert guide for tradesmen. Learn how to grow your business with digital tools.".to_string(),
            canonical_url: Some(format!("https://xftradesmen.com/blog/{}", slug())),
            og_image: None,
        }/>

        <article class="bg-void min-h-screen text-white pt-32 pb-20 relative overflow-hidden">
             <div class="absolute inset-0 bg-cyber-grid bg-[length:30px_30px] opacity-10 pointer-events-none"></div>

            <div class="max-w-3xl mx-auto px-4 relative z-10">
                <div class="mb-12 animate-fade-in">
                     <a href="/blog" class="text-gray-500 hover:text-brand text-xs font-mono font-bold uppercase tracking-wider flex items-center gap-2 mb-8 transition-colors">
                        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/></svg>
                        "Back to Knowledge Base"
                    </a>

                    <div class="flex items-center gap-4 mb-6">
                        <span class="px-2 py-1 rounded bg-void-highlight text-gray-400 text-[10px] font-mono uppercase tracking-widest">"Intelligence"</span>
                        <span class="text-gray-500 text-xs font-mono uppercase">"August 15, 2024"</span>
                    </div>

                    <h1 class="text-4xl md:text-6xl font-heading font-black mt-4 mb-8 leading-[1.1]">
                        {move || match slug().as_str() {
                            "why-tradesmen-need-websites" => "Why Tradesmen with Websites Earn 40% More",
                            "local-seo-guide" => "The Ultimate Guide to Local SEO for Plumbers",
                            "building-trust-online" => "How to Build Instant Trust with Potential Clients",
                            _ => "Article Title Placeholder"
                        }}
                    </h1>

                    <div class="flex items-center gap-4 text-xs text-brand-light border-b border-void-highlight pb-8 font-mono">
                         <span class="flex items-center gap-2">
                            <span class="w-1 h-1 bg-brand rounded-full"></span>
                            "5 MIN READ"
                         </span>
                         <span class="flex items-center gap-2">
                             <span class="w-1 h-1 bg-brand rounded-full"></span>
                             "BY XF Tradesmen"
                         </span>
                    </div>
                </div>

                <div class="prose prose-invert prose-lg max-w-none prose-headings:font-heading prose-headings:font-bold prose-a:text-brand prose-a:no-underline hover:prose-a:text-brand-light prose-blockquote:border-l-brand prose-blockquote:bg-void-surface prose-blockquote:py-2 prose-blockquote:px-6 prose-blockquote:not-italic prose-blockquote:rounded-r-lg">
                    <p class="lead text-xl text-gray-300 mb-8 font-light leading-relaxed">
                        "Introduction placeholder. This is where the engaging hook would go, explaining why this topic is critical for tradesmen right now."
                    </p>

                    <h2>"Why This Matters"</h2>
                    <p>
                        "Content placeholder. Detailed explanation of the problem and solution."
                    </p>

                    // Specific content injection based on slug could go here
                    {move || match slug().as_str() {
                         "why-tradesmen-need-websites" => view! {
                            <div>
                                <p>"Statistics show that 70% of homeowners search online before hiring a tradesperson. Without a website, you are invisible to this massive market."</p>
                                <h3>"The specific benefits:"</h3>
                                <ul>
                                    <li>"Credibility and Professionalism"</li>
                                    <li>"24/7 Lead Generation"</li>
                                    <li>"Showcasing Your Best Work"</li>
                                </ul>
                            </div>
                         }.into_any(),
                         _ => view! {
                            <p>"Specific content for this article is being drafted..."</p>
                        }.into_any()
                    }}

                    <h2>"Next Steps"</h2>
                    <p>
                        "Ready to take action? Start by building your free website with XF Tradesmen today."
                    </p>
                </div>

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
    }
}
