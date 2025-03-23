use std::error::Error;
use image::{ImageBuffer, ImageReader};
use image::imageops::FilterType;

#[allow(unused)]
pub trait Image {
    type Output;

    fn new(width: u32, height: u32) -> Self;
    fn from_path(path: &str) -> Result<Self::Output, Box<dyn Error>>;
    fn dimensions(&self) -> (u32, u32);
}

pub struct ImageWrapper {
    pub buffer: ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    pub width: u32,
    pub height: u32,
}

impl Image for ImageWrapper {
    type Output = Self;

    fn new(width: u32, height: u32) -> Self {
        let blank_image = ImageBuffer::from_pixel(width, height, image::Rgb([0, 0, 0]));

        ImageWrapper {
            buffer: blank_image,
            width,
            height,
        }
    }

    fn from_path(path: &str) -> Result<Self, Box<dyn Error>> {
        let reader = ImageReader::open(path)?;
        let image = reader.decode()?;
        let rgb_image = image.to_rgb8();

        Ok(
            ImageWrapper {
                buffer: rgb_image,
                width: image.width(),
                height: image.height(),
            }
        )
    }

    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl ImageWrapper {
    /// Scales the image to half its height since most monospace fonts are approximately twice as high as they are wide.
    /// This should be used for a good-looking result, even though some data is lost.
    pub fn prepare_scale(&mut self) {
        let new_height = self.height / 2;
        let new_buffer = image::imageops::resize(
            &self.buffer,
            self.width,
            new_height,
            FilterType::Gaussian
        );

        self.buffer = new_buffer;
    }

    /// Scales the image by the given factor, lower than 1.0 will scale down, higher than 1.0 will scale up.
    #[allow(unused)]
    pub fn scale(&mut self, scale_factor_x: f32, scale_factor_y: f32) {
        let new_width = (self.width as f32 * scale_factor_x) as u32;
        let new_height = (self.height as f32 * scale_factor_y) as u32;

        let new_buffer = image::imageops::resize(
            &self.buffer,
            new_width,
            new_height,
            FilterType::Gaussian
        );

        self.buffer = new_buffer;
    }
    
    pub fn from_bytes(data: Vec<u8>, width: u32, height: u32) -> Option<Self> {
        let buffer: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, data)?;
        
        Some(Self {
            buffer,
            width,
            height,
        })
    }
}
