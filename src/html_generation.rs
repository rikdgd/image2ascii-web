/// Converts a regular '\n' divided text image into a proper HTML string that can be inserted into the frontend.
/// ## parameters:
/// * `image` - The text image that should be used to generate the html content.
/// ## returns:
/// The function returns an HTML string that represents the text based image. The image is
/// separated in rows, each row as it own paragraph element.
pub fn get_html_image_string(image: &str) -> String {
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
