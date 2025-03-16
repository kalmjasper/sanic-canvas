use wasm_bindgen::prelude::*;

// This is the entry point for our WebAssembly module
#[wasm_bindgen]
pub fn init() {
    // Set up panic hook for better error messages in debug mode
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
}
