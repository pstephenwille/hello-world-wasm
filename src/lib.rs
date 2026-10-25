mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

// #[wasm_bindgen(start)]
// pub fn init() {
//     utils::set_panic_hook();
// }

// Automatically runs when the Wasm module loads, clearing the dead-code warning
#[wasm_bindgen(start)]
pub fn init() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn greet() {
    console::log_1(&"Hello, hello-world-wasm!".into());
}
