//! Card component for content containers.
//!
//! Provides styled containers for different content types.

#![allow(dead_code)]

use leptos::prelude::*;

/// Card style variant.
#[derive(Clone, Copy, PartialEq)]
pub enum CardVariant {
    /// Deep dark card with subtle border
    Deep,
    /// Glassmorphism style with backdrop blur
    Glass,
    /// Brand-accented with gradient border
    Brand,
}

impl CardVariant {
    /// Get CSS class for this variant.
    pub fn class(&self) -> &'static str {
        match self {
            CardVariant::Deep => "card-deep",
            CardVariant::Glass => "card-glass",
            CardVariant::Brand => "card-brand",
        }
    }
}

/// Content card component with title and shadow.
#[component]
pub fn Card(
    /// Card heading text
    #[prop(into)]
    title: String,
    /// Card content
    children: Children,
    /// Card style variant
    #[prop(default = CardVariant::Deep)]
    variant: CardVariant,
) -> impl IntoView {
    view! {
        <div class=variant.class()>
            <h3 class="text-xl font-bold text-white mb-4 border-b border-void-highlight pb-2">{title}</h3>
            <div>
                {children()}
            </div>
        </div>
    }
}

/// Simple card for FAQ items with question/answer format.
#[component]
pub fn FaqCard(
    /// Question text
    #[prop(into)]
    question: String,
    /// Answer text
    #[prop(into)]
    answer: String,
) -> impl IntoView {
    view! {
        <div class="border border-void-highlight rounded-xl p-6 bg-void hover:border-brand/30 transition-colors">
            <h3 class="text-lg font-bold text-white mb-2">{question}</h3>
            <p class="text-gray-400">{answer}</p>
        </div>
    }
}

/// Feature card with icon, title, and description.
#[component]
pub fn FeatureCard(
    /// Feature title
    #[prop(into)]
    title: String,
    /// Feature description
    #[prop(into)]
    description: String,
    /// Icon element (passed as children)
    children: Children,
) -> impl IntoView {
    view! {
        <div class="card-deep">
            <div class="w-12 h-12 bg-void-highlight rounded-lg flex items-center justify-center text-brand mb-6">
                {children()}
            </div>
            <h3 class="text-xl font-bold text-white mb-3">{title}</h3>
            <p class="text-gray-400">{description}</p>
        </div>
    }
}

/// Stat card for displaying metrics.
#[component]
pub fn StatCard(
    /// Stat value
    #[prop(into)]
    value: String,
    /// Stat label
    #[prop(into)]
    label: String,
    /// Optional border accent color class
    #[prop(optional, into)]
    accent: Option<String>,
) -> impl IntoView {
    let border_class = accent.unwrap_or_default();

    view! {
        <div class=format!("card-deep text-center py-8 {}", border_class)>
            <div class="text-3xl font-bold text-white mb-2 font-mono">{value}</div>
            <div class="text-xs text-gray-500 uppercase">{label}</div>
        </div>
    }
}
