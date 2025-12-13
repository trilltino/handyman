//! Card component for content containers.
//!
//! Provides a styled container with title and children.

use leptos::prelude::*;

/// Content card component with title and shadow.
#[component]
pub fn Card(
    /// Card heading text
    #[prop(into)]
    title: String,
    /// Card content
    #[prop(into)]
    children: Children,
) -> impl IntoView {
    view! {
        <div class="card-deep">
            <h3 class="text-xl font-bold text-white mb-4 border-b border-void-highlight pb-2">{title}</h3>
            <div>
                {children()}
            </div>
        </div>
    }
}
