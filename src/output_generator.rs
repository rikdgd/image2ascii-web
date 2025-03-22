use std::fs::OpenOptions;
use std::io::Write;

pub trait OutputGenerator {
    fn from_ascii_image(image: String) -> Self;
    fn generate_output(&self) -> std::io::Result<()>;
}

pub struct HtmlGenerator {
    image: String,
    output_path: String,
}
impl OutputGenerator for HtmlGenerator {
    fn from_ascii_image(image: String) -> Self {
        Self {
            image,
            output_path: String::from("./ascii-image.html"),
        }
    }

    fn generate_output(&self) -> std::io::Result<()> {
        let html_image_string = get_html_image_string(&self.image);
        let mut options = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.output_path)?;
        
        options.set_len(0)?;
        options.write_all(html_image_string.as_bytes())?;
        Ok(())
    }
}

impl HtmlGenerator {
    #[allow(unused)]
    pub fn set_output_path<S: Into<String>>(&mut self, path: S) {
        self.output_path = path.into();
    }
}

/// Converts a regular '\n' divided text image into a proper HTML string that can be inserted into the frontend.
/// ## paramters:
/// * `image` - The text image that should be used to generate the html content.
/// ## returns:
/// The function returns a HTML string that represents the text based image. The image is
/// separated in rows, each row as it own paragraph element.
fn get_html_image_string(image: &str) -> String {
    let row_divider = "</p>\n<p>";
    let mut html_image_string = String::from("<p>");

    for next_char in image.chars() {
        if next_char == '\n' {
            html_image_string.push_str(row_divider);
        } else {
            html_image_string.push(next_char);
        }
    }

    html_image_string.push_str("</p>");
    html_image_string
}