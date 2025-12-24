//! XFTradesmen Mobile App
//!
//! Tauri wrapper that loads the deployed XFTradesmen website
//! in a native webview for iOS and Android.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
