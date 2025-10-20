//! # Home Page
//!
//! ## Purpose
//! Landing page for the handyman marketplace.
//! First page visitors see with hero section and call-to-action.
//!
//! ## Content
//! - Hero section with welcome message
//! - Brief description of services
//! - "Book a Service" call-to-action button
//! - Future: Features, testimonials, how it works, pricing
//!
//! ## Relation to Entire Program
//! - **Route**: `/` (root path)
//! - **Public**: No authentication required
//! - **Goal**: Convert visitors to bookings

use yew::prelude::*;  // Yew framework
use stylist::css;     // CSS-in-Rust styling

#[function_component]
pub fn Home() -> Html {
    let css = css!(r#"
        .home-container { min-height: 80vh; display: flex; align-items: center; justify-content: center; }
        .hero { text-align: center; padding: 60px 20px; }
        .hero h1 { font-size: 3rem; color: #2c3e50; margin-bottom: 20px; }
        .hero p { font-size: 1.5rem; color: #7f8c8d; margin-bottom: 30px; }
        .cta-button { display: inline-block; padding: 15px 40px; background: #3498db; color: white; text-decoration: none; border-radius: 5px; font-size: 1.2rem; transition: background 0.3s; }
        .cta-button:hover { background: #2980b9; }
    "#);

    html! {
        <div class={css}>
            <div class="home-container">
                <div class="hero">
                    <h1>{ "Welcome to Handyman Services" }</h1>
                    <p>{ "Professional home repair and maintenance services at your doorstep" }</p>
                    <a href="/booking" class="cta-button">{ "Book a Service" }</a>
                </div>
            </div>
        </div>
    }
}
