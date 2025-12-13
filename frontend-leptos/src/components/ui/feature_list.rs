//! Feature list component.
//!
//! Displays a list of features with check icons.

use leptos::prelude::*;

/// Bulleted feature list with checkmark icons.
#[component]
pub fn FeatureList(
    /// List of feature strings
    #[prop(into)]
    features: Vec<String>,
) -> impl IntoView {
    view! {
        <ul class="space-y-4">
            {features.into_iter().map(|feature| {
                view! {
                    <li class="flex items-start gap-3">
                        <div class="mt-1 flex-shrink-0 w-5 h-5 rounded-full bg-brand-deep/50 flex items-center justify-center border border-brand-glow/50">
                             <svg class="w-3 h-3 text-brand-light" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7"/>
                            </svg>
                        </div>
                        <span class="text-gray-300">{feature}</span>
                    </li>
                }
            }).collect::<Vec<_>>()}
        </ul>
    }
}
