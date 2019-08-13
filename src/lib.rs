mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

#[wasm_bindgen]
pub fn get_total(number: u32) -> u32 {
    let mut total = 0;
    for i in 0..number {
        total += i;
    }
    return total;
}

#[wasm_bindgen]
pub fn get_str(input: &str) -> String {
    let mut buf = String::with_capacity(input.len());

    for c in input.chars() {
        if c != ' ' {
            buf.push(c);
        }
    }

    return buf;
}
