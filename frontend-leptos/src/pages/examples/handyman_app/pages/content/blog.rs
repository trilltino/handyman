//! Blog page for handyman site content marketing.
//!
//! Displays blog posts for SEO and authority building.

use leptos::prelude::*;
use leptos_meta::Title;

#[derive(Clone, Debug, PartialEq)]
struct BlogPost {
    slug: &'static str,
    title: &'static str,
    excerpt: &'static str,
    category: &'static str,
    date: &'static str,
    read_time: &'static str,
}

fn get_blog_posts() -> Vec<BlogPost> {
    vec![
        BlogPost {
            slug: "handyman-cost-coventry",
            title: "How Much Does a Handyman Cost in Coventry? (2024 Guide)",
            excerpt: "Complete pricing guide for handyman services in Coventry. Learn what to expect to pay for common jobs and how to get the best value.",
            category: "Pricing",
            date: "Dec 20, 2024",
            read_time: "5 min",
        },
        BlogPost {
            slug: "winter-home-maintenance",
            title: "10 Essential Home Maintenance Tasks Before Winter",
            excerpt: "Prepare your home for the cold months ahead. From checking seals to bleeding radiators, here's your complete winter prep checklist.",
            category: "Maintenance",
            date: "Dec 15, 2024",
            read_time: "7 min",
        },
        BlogPost {
            slug: "diy-vs-professional",
            title: "When to DIY vs Hire a Professional Handyman",
            excerpt: "Know when to tackle a job yourself and when it's time to call in the experts. Plus, common DIY mistakes to avoid.",
            category: "Tips",
            date: "Dec 10, 2024",
            read_time: "6 min",
        },
        BlogPost {
            slug: "furniture-assembly-tips",
            title: "IKEA Furniture Assembly: 7 Pro Tips for Success",
            excerpt: "Secrets from professional assemblers to make your flatpack furniture sturdy and stress-free.",
            category: "Tips",
            date: "Dec 5, 2024",
            read_time: "4 min",
        },
        BlogPost {
            slug: "finding-reliable-handyman",
            title: "How to Find a Reliable Handyman in Coventry",
            excerpt: "What to look for when hiring a handyman. Credentials, insurance, reviews, and red flags to watch out for.",
            category: "Advice",
            date: "Nov 28, 2024",
            read_time: "5 min",
        },
    ]
}

#[component]
pub fn HandymanBlog() -> impl IntoView {
    let posts = get_blog_posts();

    view! {
        <Title text="Blog | Coventry Handyman Tips & Advice"/>

        <div class="min-h-screen bg-slate-50">
            // Hero
            <section class="bg-gradient-to-br from-blue-900 to-blue-800 text-white py-16 px-6">
                <div class="max-w-4xl mx-auto text-center">
                    <h1 class="text-4xl md:text-5xl font-black mb-4">"Handyman Tips & Advice"</h1>
                    <p class="text-xl text-blue-200">
                        "Expert guides, DIY tips, and home maintenance advice from Coventry's trusted handymen."
                    </p>
                </div>
            </section>

            // Featured Post
            <section class="py-12 px-6">
                <div class="max-w-6xl mx-auto">
                    <div class="bg-white rounded-2xl shadow-lg overflow-hidden grid md:grid-cols-2">
                        <div class="bg-gradient-to-br from-yellow-400 to-yellow-500 p-8 flex items-center justify-center">
                            <div class="w-24 h-24 bg-white/20 rounded-full flex items-center justify-center">
                                <svg class="w-12 h-12 text-blue-900" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                </svg>
                            </div>
                        </div>
                        <div class="p-8">
                            <span class="inline-block px-3 py-1 bg-yellow-100 text-yellow-800 text-sm font-medium rounded-full mb-4">
                                "Featured"
                            </span>
                            <h2 class="text-2xl font-bold text-slate-900 mb-3">
                                "How Much Does a Handyman Cost in Coventry?"
                            </h2>
                            <p class="text-slate-600 mb-4">
                                "Complete pricing guide for 2024. Know exactly what to expect to pay for common handyman jobs in the Coventry area."
                            </p>
                            <a href="/handyman-coventry/blog/handyman-cost-coventry" class="inline-flex items-center gap-2 text-blue-600 font-bold hover:text-blue-700">
                                "Read Full Guide"
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                                </svg>
                            </a>
                        </div>
                    </div>
                </div>
            </section>

            // All Posts
            <section class="py-12 px-6 bg-white">
                <div class="max-w-6xl mx-auto">
                    <h2 class="text-2xl font-bold text-slate-900 mb-8">"Latest Articles"</h2>

                    <div class="grid md:grid-cols-3 gap-8">
                        {posts.iter().skip(1).map(|post| {
                            let post = post.clone();
                            view! {
                                <article class="bg-slate-50 rounded-xl overflow-hidden hover:shadow-lg transition group">
                                    <div class="h-40 bg-gradient-to-br from-blue-100 to-blue-200 flex items-center justify-center">
                                        <svg class="w-12 h-12 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                                        </svg>
                                    </div>
                                    <div class="p-6">
                                        <div class="flex items-center gap-3 text-sm text-slate-500 mb-3">
                                            <span class="px-2 py-1 bg-blue-100 text-blue-700 rounded text-xs font-medium">
                                                {post.category}
                                            </span>
                                            <span>{post.date}</span>
                                            <span>"·"</span>
                                            <span>{post.read_time}</span>
                                        </div>
                                        <h3 class="text-lg font-bold text-slate-900 mb-2 group-hover:text-blue-600 transition">
                                            {post.title}
                                        </h3>
                                        <p class="text-slate-600 text-sm mb-4">
                                            {post.excerpt}
                                        </p>
                                        <a
                                            href=format!("/handyman-coventry/blog/{}", post.slug)
                                            class="text-blue-600 font-medium text-sm hover:text-blue-700"
                                        >
                                            "Read more"
                                        </a>
                                    </div>
                                </article>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </section>

            // Newsletter CTA
            <section class="py-16 px-6">
                <div class="max-w-4xl mx-auto bg-gradient-to-r from-blue-900 to-blue-800 rounded-2xl p-8 text-white text-center">
                    <h2 class="text-2xl font-bold mb-4">"Get Home Tips in Your Inbox"</h2>
                    <p class="text-blue-200 mb-6">"Monthly maintenance reminders and DIY advice. No spam, just helpful tips."</p>
                    <div class="flex gap-3 max-w-md mx-auto">
                        <input
                            type="email"
                            placeholder="Your email"
                            class="flex-1 px-4 py-3 rounded-lg text-slate-900"
                        />
                        <button class="px-6 py-3 bg-yellow-500 text-blue-900 font-bold rounded-lg hover:bg-yellow-400 transition">
                            "Subscribe"
                        </button>
                    </div>
                </div>
            </section>
        </div>
    }
}
