//! Badge component for status indicators.
//!
//! Displays small label badges with custom styling and color variants.

use leptos::prelude::*;

/// Badge color variant for different contexts.
#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum BadgeVariant {
    /// Red/brand color - default style
    Brand,
    /// Green - for success/online status
    Success,
    /// Yellow - for warnings/popular tags
    Warning,
    /// Gray/mono - for tech labels
    Info,
}

impl BadgeVariant {
    /// Get CSS classes for dot, text, and background.
    pub fn classes(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            BadgeVariant::Brand => (
                "bg-brand-glow animate-pulse",
                "text-brand-light",
                "bg-brand/10 border-brand/20",
            ),
            BadgeVariant::Success => (
                "bg-green-500 animate-pulse",
                "text-green-400",
                "bg-green-500/10 border-green-500/20",
            ),
            BadgeVariant::Warning => (
                "bg-yellow-500 animate-pulse",
                "text-yellow-500",
                "bg-yellow-500/10 border-yellow-500/20",
            ),
            BadgeVariant::Info => (
                "bg-gray-400",
                "text-gray-300",
                "bg-void-surface border-void-highlight",
            ),
        }
    }
}

/// Inline badge/label component with color variants.
#[component]
pub fn Badge(
    /// Badge text
    #[prop(into)]
    label: String,
    /// Color variant
    #[prop(default = BadgeVariant::Brand)]
    variant: BadgeVariant,
) -> impl IntoView {
    let (dot_class, text_class, bg_class) = variant.classes();

    view! {
        <div class=format!(
            "inline-flex items-center gap-2 px-3 py-1 rounded-full border text-xs font-mono font-bold tracking-widest uppercase {}",
            bg_class
        )>
            <span class=format!("w-1.5 h-1.5 rounded-full {}", dot_class)></span>
            <span class=text_class>{label}</span>
        </div>
    }
}
