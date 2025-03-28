#![recursion_limit = "2048"] // <-- Add this line at the top

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(feature = "hydrate")]
use leptos::logging::log; // Import the log macro from leptos
#[cfg(feature = "hydrate")]
use log::Level; // Import Level from the log crate


// Import modules
pub mod app;
pub mod pages;
pub mod utils;


#[cfg(feature = "hydrate")]
#[wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    use leptos::mount::hydrate_body;
    // initializes logging using the `log` crate
    // Use console_log::init_with_level from the imported crate
    _ = console_log::init_with_level(Level::Debug);
    console_error_panic_hook::set_once();

    log!("hydrating app"); // Now uses leptos::logging::log

    hydrate_body(App);
}
