//! # Error Boundary Components
//!
//! Error boundary components for graceful error handling in the UI.
//!
//! ## Usage
//!
//! ```rust
//! use crate::components::error_boundary::ErrorBoundary;
//!
//! view! {
//!     <ErrorBoundary>
//!         <SomeComponent />
//!     </ErrorBoundary>
//! }
//! ```

use leptos::prelude::*;

/// Error boundary wrapper for graceful error handling.
///
/// Catches panics and errors in child components and displays
/// a user-friendly error message instead of crashing.
#[component]
pub fn ErrorBoundary(children: Children) -> impl IntoView {
    view! {
        <leptos::error::ErrorBoundary fallback=|errors| {
            view! {
                <div class="min-h-[200px] flex items-center justify-center p-8">
                    <div class="bg-red-50 border border-red-200 rounded-lg p-6 max-w-md text-center">
                        <svg class="w-12 h-12 text-red-500 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                        </svg>
                        <h3 class="text-lg font-semibold text-red-800 mb-2">"Something went wrong"</h3>
                        <p class="text-red-600 text-sm mb-4">
                            "We encountered an error loading this section. Please try refreshing the page."
                        </p>
                        <button
                            class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition"
                            on:click=move |_| {
                                // Reload the page
                                let _ = web_sys::window().map(|w| w.location().reload());
                            }
                        >
                            "Refresh Page"
                        </button>
                        <details class="mt-4 text-left">
                            <summary class="text-xs text-red-500 cursor-pointer">"Technical Details"</summary>
                            <pre class="mt-2 text-xs bg-red-100 p-2 rounded overflow-auto max-h-32">
                                {move || errors.get()
                                    .into_iter()
                                    .map(|(_, e)| format!("{:?}", e))
                                    .collect::<Vec<_>>()
                                    .join("\n")
                                }
                            </pre>
                        </details>
                    </div>
                </div>
            }
        }>
            {children()}
        </leptos::error::ErrorBoundary>
    }
}

/// Lightweight error fallback for small sections.
#[component]
pub fn ErrorFallbackSmall(#[prop(into)] message: String) -> impl IntoView {
    view! {
        <div class="p-4 bg-red-50 border border-red-200 rounded-lg text-red-700 text-sm">
            <span class="font-medium">"Error: "</span>
            {message}
        </div>
    }
}

/// Loading fallback with spinner.
#[component]
pub fn LoadingSpinner(#[prop(optional)] size: Option<&'static str>) -> impl IntoView {
    let size_class = size.unwrap_or("w-8 h-8");
    view! {
        <div class="flex items-center justify-center p-4">
            <svg class=format!("{} animate-spin text-blue-600", size_class) fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
        </div>
    }
}

/// Suspense-style loading placeholder.
#[component]
pub fn LoadingPlaceholder(#[prop(optional)] message: Option<&'static str>) -> impl IntoView {
    let msg = message.unwrap_or("Loading...");
    view! {
        <div class="flex flex-col items-center justify-center p-8 text-gray-500">
            <LoadingSpinner />
            <p class="mt-4 text-sm">{msg}</p>
        </div>
    }
}
