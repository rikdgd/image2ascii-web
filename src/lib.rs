mod output_generator;
mod image_wrapper;
mod image_converter;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn convert_image(img_data: &str) -> String {
    String::from(img_data)
}