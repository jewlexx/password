pub mod app;
pub mod components;

#[macro_use]
extern crate log;

use app::App;

// Use `std::alloc` as the global allocator.
#[global_allocator]
static ALLOC: std::alloc::System = std::alloc::System;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
