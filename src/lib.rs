mod html_generation;
mod image_wrapper;
mod image_converter;

use image_converter::{ImageConverter, ImageToTextConverter};
use image_wrapper::ImageWrapper;
use html_generation::get_html_image_string;
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
pub fn convert_image(img_data: Vec<u8>, width: u32, height: u32) -> Option<String> {
    let img_wrapper = ImageWrapper::from_bytes(img_data, width, height)?;
    let mut converter = ImageToTextConverter::from_image_wrapper(img_wrapper);
    let text_image = converter.convert();
    
    let html_image = get_html_image_string(&text_image);
    Some(html_image)
}