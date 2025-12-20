//! Section component for page layout.
//!
//! Provides consistent section spacing and styling with optional subtitle.

#![allow(dead_code)]

use leptos::prelude::*;

/// Full-width content section with responsive layout.
#[component]
pub fn Section(
    /// Section heading text
    #[prop(into)]
    title: String,
    /// Section content
    children: Children,
    /// Use darker background variant?
    #[prop(default = false)]
    darker: bool,
    /// Optional subtitle/tagline above title
    #[prop(optional, into)]
    subtitle: Option<String>,
    /// Optional section ID for anchor links
    #[prop(optional, into)]
    id: Option<String>,
    /// Show decorative accent bar?
    #[prop(default = true)]
    show_accent: bool,
) -> impl IntoView {
    let bg_class = if darker { "bg-void" } else { "bg-void-surface" };

    view! {
        <section
            class=format!("py-16 px-4 {} relative", bg_class)
            id=id
        >
            <div class="max-w-6xl mx-auto">
                <div class="mb-10">
                    {subtitle.map(|s| view! {
                        <span class="text-brand font-mono text-sm tracking-widest uppercase mb-2 block">{s}</span>
                    })}
                    <div class="flex items-center gap-4">
                        {show_accent.then(|| view! {
                            <div class="h-1 w-10 bg-brand rounded-full"></div>
                        })}
                        <h2 class="text-3xl font-bold text-white tracking-tight">{title}</h2>
                    </div>
                </div>
                {children()}
            </div>
        </section>
    }
}

/// Hero section variant with full-height and centered content.
#[component]
pub fn HeroSection(
    /// Hero content
    children: Children,
    /// Minimum height class (e.g., "min-h-screen", "min-h-[90vh]")
    #[prop(default = "min-h-screen")]
    min_height: &'static str,
) -> impl IntoView {
    view! {
        <section class=format!(
            "relative bg-void {} flex items-center pt-20 pb-32 px-4 overflow-hidden",
            min_height
        )>
            // Background Effects
            <div class="absolute inset-0 bg-cyber-grid bg-[length:40px_40px] opacity-20"></div>
            <div class="absolute top-[-20%] left-1/2 -translate-x-1/2 w-[800px] h-[800px] bg-brand/10 blur-[120px] rounded-full pointer-events-none animate-pulse-slow"></div>
            <div class="absolute bottom-[-10%] right-[-10%] w-[600px] h-[600px] bg-void-highlight/30 blur-[100px] rounded-full pointer-events-none"></div>

            <div class="relative max-w-7xl mx-auto z-10">
                {children()}
            </div>
        </section>
    }
}

/// CTA (Call to Action) section with brand background.
#[component]
pub fn CtaSection(
    /// CTA content
    children: Children,
) -> impl IntoView {
    view! {
        <section class="relative py-32 px-4 text-center overflow-hidden">
            <div class="absolute inset-0 bg-brand-deep"></div>
            <div class="absolute inset-0 bg-[url('https://www.transparenttextures.com/patterns/carbon-fibre.png')] opacity-10 mix-blend-overlay"></div>
            <div class="absolute inset-0 bg-gradient-to-t from-void via-transparent to-void opacity-80"></div>

            <div class="relative max-w-4xl mx-auto z-10">
                {children()}
            </div>
        </section>
    }
}
