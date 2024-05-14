use std::fmt::format;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern  {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn say(message: &str) {
    log(&format!("{}", message));
}

#[wasm_bindgen]
pub fn generate() -> usize {
    1000
}