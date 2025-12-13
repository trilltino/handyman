//! Badge component for status indicators.
//!
//! Displays small label badges with custom styling.

use leptos::prelude::*;

/// Inline badge/label component.
#[component]
pub fn Badge(
    /// Badge text
    #[prop(into)]
    label: String,
) -> impl IntoView {
    view! {
        <div class="badge-red">
            <span class="text-brand-glow">"●"</span>
            {label}
        </div>
    }
}
