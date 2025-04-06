use crate::image_wrapper::{ImageWrapper};
use crate::character_mapping::{UserOs, get_char_mapping};


pub struct ImageToTextConverter {
    pub image_wrapper: ImageWrapper,
    pub user_os: UserOs,
}

impl ImageToTextConverter {
    pub fn convert(mut self) -> String {
        self.convert_to_text(ImageScaleOptions::default())
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
            let pixel_char = pixel_to_char(pixel, UserOs::Unix);
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
    
    /// Converts image data to text that resembles the image in brightness values, where the rows are
    /// seperated by `\n`.
    /// 
    /// ## parameters:
    /// * `scale_options` - The image wrapper that holds the image that should be converted to text.
    /// 
    fn convert_to_text(&mut self, scale_options: ImageScaleOptions) -> String {
        if let ImageScaleOptions::HalfHeight = scale_options {
            self.image_wrapper.prepare_scale();
        }

        let pixels = self.image_wrapper.buffer.pixels();
        
        let mut text_image = String::new();
        let mut row_counter: u32 = 0;
        
        for pixel in pixels {
            let pixel_char = pixel_to_char(pixel, self.user_os);
            text_image.push(pixel_char);
            
            if row_counter == self.image_wrapper.width - 1 {
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

fn pixel_to_char(pixel: &image::Rgb<u8>, user_os: UserOs) -> char {
    let mut brightness = get_pixel_brightness(pixel);
    if brightness > u8::MAX as u32 {
        brightness = 255;
    }
    
    get_char_mapping(brightness as u8, user_os)
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
    use super::pixel_to_char;
    use crate::character_mapping::{UserOs, UNIX_CHAR_MAPPING};

    #[test]
    fn pixel_to_char_test() {
        let user_os = UserOs::Unix;
        let black_pixel = image::Rgb([0, 0, 0]);
        let white_pixel = image::Rgb([255, 255, 255]);

        let pixel_char_black = pixel_to_char(&black_pixel, user_os);
        let pixel_char_white = pixel_to_char(&white_pixel, user_os);

        assert_eq!(pixel_char_black, UNIX_CHAR_MAPPING[0]);
        assert_eq!(pixel_char_white, UNIX_CHAR_MAPPING[9]);
    }
}
