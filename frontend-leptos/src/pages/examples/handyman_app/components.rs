//! Design System for Handyman App - "Deep Visual Trust"

use leptos::prelude::*;

#[component]
pub fn SectionTitle(
    subtitle: &'static str,
    title: &'static str,
    #[prop(optional)] align: &'static str, // "left", "center"
    #[prop(optional)] dark: bool,
) -> impl IntoView {
    let align_class = if align == "left" {
        "text-left"
    } else {
        "text-center mx-auto"
    };
    let text_color = if dark { "text-white" } else { "text-blue-900" };
    let subtitle_color = if dark {
        "text-yellow-400"
    } else {
        "text-blue-700"
    };
    let bar_color = if dark { "bg-yellow-400" } else { "bg-blue-600" };

    view! {
        <div class={format!("mb-12 max-w-3xl {}", align_class)}>
            <span class={format!("inline-block uppercase tracking-widest text-xs font-bold mb-3 {}", subtitle_color)}>
                {subtitle}
            </span>
            <h2 class={format!("text-3xl md:text-5xl font-black mb-6 leading-tight drop-shadow-sm {}", text_color)}>
                {title}
            </h2>
            <div class={format!("h-1.5 w-24 rounded-full {}", bar_color)}></div>
        </div>
    }
}

#[component]
pub fn GlassCard(children: Children, #[prop(optional)] class: &'static str) -> impl IntoView {
    view! {
        <div class={format!("relative overflow-hidden bg-white/90 backdrop-blur-md border border-white/50 shadow-xl shadow-blue-900/5 rounded-2xl p-8 hover:shadow-2xl hover:shadow-blue-900/10 hover:-translate-y-1 transition-all duration-300 group {}", class)}>
             {children()}
        </div>
    }
}

#[component]
pub fn CtaButton(
    text: &'static str,
    href: &'static str,
    #[prop(optional)] variant: &'static str, // "primary", "outline", "glass"
) -> impl IntoView {
    let base_class = "relative inline-flex items-center justify-center px-8 py-4 text-lg font-bold uppercase tracking-wide transition-all duration-300 transform rounded-lg overflow-hidden group";

    let variant_class = match variant {
        "outline" => "bg-transparent border-2 border-current text-white hover:bg-white hover:text-blue-900",
        "glass" => "bg-white/10 backdrop-blur border border-white/30 text-white hover:bg-white/20 shadow-lg",
        _ => "bg-gradient-to-br from-yellow-400 to-yellow-500 text-blue-900 shadow-lg shadow-yellow-500/30 hover:shadow-yellow-500/50 hover:scale-[1.02]", // Primary
    };

    view! {
        <a href={href} class={format!("{} {}", base_class, variant_class)}>
            <span class="relative z-10 flex items-center gap-2">
                {text}
                <svg class="w-5 h-5 transition-transform group-hover:translate-x-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/></svg>
            </span>
            // Shine effect for primary button
            {if variant == "" || variant == "primary" {
                view! { <div class="absolute inset-0 -translate-x-full group-hover:animate-[shimmer_1.5s_infinite] bg-gradient-to-r from-transparent via-white/30 to-transparent z-0"></div> }.into_any()
            } else {
                ().into_any()
            }}
        </a>
    }
}

#[component]
pub fn TrustBadge(icon_path: &'static str, text: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-center gap-3 bg-blue-50/50 px-4 py-2 rounded-full border border-blue-100 shadow-sm">
            <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d=icon_path/></svg>
            <span class="text-sm font-bold text-blue-900">{text}</span>
        </div>
    }
}
