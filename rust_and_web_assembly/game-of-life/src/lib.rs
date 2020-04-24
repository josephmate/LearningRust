mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Renderer {
    count: u32
}

#[wasm_bindgen]
impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            count: 0,
        }
    }
    
    pub fn render(&mut self) -> u32 {
        self.count = self.count + 1;
        self.count
    }
}