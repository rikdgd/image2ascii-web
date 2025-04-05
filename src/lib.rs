mod html_generation;
mod image_wrapper;
mod image_converter;
mod character_mapping;

use image_converter::ImageToTextConverter;
use image_wrapper::ImageWrapper;
use html_generation::get_html_image_string;
use wasm_bindgen::prelude::*;
use character_mapping::UserOs;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn convert_image(img_data: Vec<u8>, user_os: &str) -> Option<String> {
    match ImageWrapper::from_bytes(img_data) {
        Ok(image_wrapper) => {
            let converter = ImageToTextConverter {
                image_wrapper,
                user_os: UserOs::from_str(user_os),
            };
            let text_image = converter.convert();

            let html_image = get_html_image_string(&text_image);
            Some(html_image)
        },
        Err(err) => {
            alert(&format!("Failed to convert image to text:\n{}", err));
            None
        },
    }
}
