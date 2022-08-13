pub mod files;
pub mod headers;
pub mod settings;
pub mod map;

use wasm_bindgen::prelude::*;

/// Report any panics to `console.error` to avoid losing their panic messages.
/// 
/// This only has to be called once, e.g. on intialization
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
