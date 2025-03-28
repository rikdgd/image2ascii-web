use crate::image_wrapper::{ImageWrapper};


const CHAR_MAPPING: [char; 10] = [
    ' ',
    '.',
    ':',
    '*',
    '+',
    'x',
    'H',
    '#',
    '@',
    'W'
];


pub trait ImageConverter {
    type ConvertsTo;
    fn convert(&mut self) -> Self::ConvertsTo;
}

pub struct ImageToTextConverter {
    pub image_wrapper: ImageWrapper,
}

impl ImageConverter for ImageToTextConverter {
    type ConvertsTo = String;

    fn convert(&mut self) -> Self::ConvertsTo {
        Self::convert_to_text(&mut self.image_wrapper, ImageScaleOptions::default())
    }
}

impl ImageToTextConverter {
    pub fn from_image_wrapper(image_wrapper: ImageWrapper) -> Self {
        Self { image_wrapper }
    }
    
    #[allow(unused)]
    #[deprecated(note = "Converting the image to a 2D matrix is unnecessary and makes the process take longer.")]
    fn convert_to_2d_char_matrix(image_wrapper: &mut ImageWrapper, scale_options: ImageScaleOptions) -> Vec<Vec<char>> {

        if let ImageScaleOptions::HalfHeight = scale_options {
            image_wrapper.prepare_scale();
        }

        let pixels = image_wrapper.buffer.pixels();

        let mut text_image: Vec<Vec<char>> = Vec::new();
        let mut text_image_row: Vec<char> = Vec::new();
        let mut row_counter: u32 = 0;

        for pixel in pixels {
            let pixel_char = pixel_to_char(pixel);
            text_image_row.push(pixel_char);

            if row_counter == image_wrapper.width - 1 {
                text_image.push(text_image_row);
                text_image_row = Vec::new();
                row_counter = 0;
            } else {
                row_counter += 1;
            }
        }

        text_image
    }
    
    fn convert_to_text(image_wrapper: &mut ImageWrapper, scale_options: ImageScaleOptions) -> String {
        if let ImageScaleOptions::HalfHeight = scale_options {
            image_wrapper.prepare_scale();
        }

        let pixels = image_wrapper.buffer.pixels();
        
        let mut text_image = String::new();
        let mut row_counter: u32 = 0;
        
        for pixel in pixels {
            let pixel_char = pixel_to_char(pixel);
            text_image.push(pixel_char);
            
            if row_counter == image_wrapper.width - 1 {
                row_counter = 0;
                text_image.push('\n');
            } else {
                row_counter += 1;
            }
        }
        
        text_image
    }
}

#[derive(Default)]
#[allow(unused)]
pub enum ImageScaleOptions {
    None,
    #[default]
    HalfHeight,
}

fn pixel_to_char(pixel: &image::Rgb<u8>) -> char {
    match get_pixel_brightness(pixel) {
        0..=25 => CHAR_MAPPING[0],
        26..=51 => CHAR_MAPPING[1],
        52..=77 => CHAR_MAPPING[2],
        78..=103 => CHAR_MAPPING[3],
        104..=129 => CHAR_MAPPING[4],
        130..=155 => CHAR_MAPPING[5],
        156..=181 => CHAR_MAPPING[6],
        182..=207 => CHAR_MAPPING[7],
        208..=233 => CHAR_MAPPING[8],
        _ => CHAR_MAPPING[9],
    }
}


fn get_pixel_brightness(pixel: &image::Rgb<u8>) -> u32 {
    let (red, green, blue) = (pixel[0], pixel[1], pixel[2]);
    let brightness =
        0.2126 * red as f32 +
            0.7152 * green as f32 +
            0.0722 * blue as f32;

    brightness.round() as u32
}



#[cfg(test)]
mod tests {
    use super::{
        pixel_to_char,
        CHAR_MAPPING,
    };

    #[test]
    fn pixel_to_char_test() {
        let pixel_1 = image::Rgb([0, 0, 0]);
        let pixel_6 = image::Rgb([255, 255, 255]);

        let pixel_char_1 = pixel_to_char(&pixel_1);
        let pixel_char_6 = pixel_to_char(&pixel_6);

        assert_eq!(pixel_char_1, CHAR_MAPPING[0]);  // -> ' '
        assert_eq!(pixel_char_6, CHAR_MAPPING[9]);  // -> 'W'
    }
}
