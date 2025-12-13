//! Section component for page layout.
//!
//! Provides consistent section spacing and styling.

use leptos::prelude::*;

/// Full-width content section with responsive layout.
#[component]
pub fn Section(
    /// Section heading text
    #[prop(into)]
    title: String,
    /// Section content
    #[prop(into)]
    children: Children,
    /// Use darker background variant?
    #[prop(default = false)]
    darker: bool,
) -> impl IntoView {
    let bg_class = if darker { "bg-void" } else { "bg-void-surface" };

    view! {
        <section class=format!("py-16 px-4 {}", bg_class)>
            <div class="max-w-6xl mx-auto">
                <div class="flex items-center gap-4 mb-10">
                    <div class="h-1 w-10 bg-brand rounded-full"></div>
                    <h2 class="text-3xl font-bold text-white tracking-tight">{title}</h2>
                </div>
                {children()}
            </div>
        </section>
    }
}
