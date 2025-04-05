use std::ffi::OsString;

pub const UNIX_CHAR_MAPPING: [char; 10] = [
    ' ',
    '.',
    ':',
    '*',
    '+',
    'x',
    'H',
    '#',
    '@',
    'W',
];

pub const WINDOWS_CHAR_MAPPING: [char; 10] = [
    ' ',
    '.',
    ':',
    '*',
    'x',
    '+',
    '#',
    'H',
    'W',
    '@',
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UserOs {
    Windows,
    Unix,
}
impl UserOs {
    pub fn from_str(os_string: &str) -> Self {
        match os_string {
            "windows" => UserOs::Windows,
            _ => UserOs::Unix,
        }
    }
}

/// Fetches the correct character based on a pixel's brightness value.
/// 
/// ## parameters:
/// * `brightness` - The brightness value of the pixel that should be converted.
/// * `os` - The user's operating system to use the char mapping for.
/// 
/// ## returns:
/// The `char` that has approximately the correct brightness value to replace the pixel.
/// The character that has been selected might differ based on the user's operating system.
pub fn get_char_mapping(brightness: u8, os: UserOs) -> char {
    let mapping = match os {
        UserOs::Unix => UNIX_CHAR_MAPPING,
        UserOs::Windows => WINDOWS_CHAR_MAPPING,
    };
    
    match brightness {
        0..=25 => mapping[0],
        26..=51 => mapping[1],
        52..=77 => mapping[2],
        78..=103 => mapping[3],
        104..=129 => mapping[4],
        130..=155 => mapping[5],
        156..=181 => mapping[6],
        182..=207 => mapping[7],
        208..=233 => mapping[8],
        _ => mapping[9],
    }
}
