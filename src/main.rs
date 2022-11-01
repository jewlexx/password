pub mod app;
pub mod components;

#[macro_use]
extern crate log;

use app::App;
use wasm_bindgen::prelude::wasm_bindgen;

// Use `std::alloc` as the global allocator.
#[global_allocator]
static ALLOC: std::alloc::System = std::alloc::System;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = "registerSW")]
    fn register_sw();
}

fn main() {
    // Ensure we only register the service worker in release builds
    #[cfg(not(debug_assertions))]
    {
        register_sw();
    }

    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
