//! Mobile sticky booking bar component.
//!
//! Shows at bottom of screen on mobile devices only.

use leptos::prelude::*;

/// Sticky bottom bar for mobile devices with call and book buttons
#[component]
pub fn MobileStickyBar() -> impl IntoView {
    view! {
        // This bar only shows on mobile (< md breakpoint)
        <div class="fixed bottom-0 left-0 right-0 z-50 md:hidden bg-blue-900 border-t border-blue-800 p-3 flex gap-3">
            <a
                href="tel:+447833263486"
                class="flex-1 flex items-center justify-center gap-2 py-3 bg-white text-blue-900 font-bold rounded-lg"
            >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"/>
                </svg>
                "Call Now"
            </a>
            <a
                href="/handyman-coventry/booking"
                class="flex-1 flex items-center justify-center gap-2 py-3 bg-yellow-500 text-blue-900 font-bold rounded-lg"
            >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                </svg>
                "Book Now"
            </a>
        </div>
    }
}
