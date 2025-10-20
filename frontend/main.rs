//! # Frontend Entry Point
//!
//! ## Purpose
//! WASM entry point for the Yew frontend application.
//! Initializes logging, error handling, and renders the root App component.
//!
//! ## How It Works
//! 1. Initialize WASM logger for browser console logging
//! 2. Set panic hook for better error messages in browser
//! 3. Create Yew renderer and mount App to DOM
//!
//! ## Relation to Entire Program
//! - **Entry Point**: First code executed when WASM module loads
//! - **Renders**: App component (root of component tree)
//! - **Built by**: Trunk (compiles to WASM and injects into index.html)

use frontend::App;  // Root application component

fn main() {
    // Initialize WASM logger (writes to browser console)
    wasm_logger::init(wasm_logger::Config::default());
    // Set panic hook for better error messages in browser console
    console_error_panic_hook::set_once();
    // Create Yew renderer and mount App component to DOM
    yew::Renderer::<App>::new().render();
}
