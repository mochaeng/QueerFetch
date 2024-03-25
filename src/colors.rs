#[derive(Debug)]
pub struct RGB(pub u8, pub u8, pub u8);

fn color_line(rgb: &RGB) -> String {
    format!("\x1b[48;2;{};{};{}m", rgb.0, rgb.1, rgb.2)
}

pub fn get_color_line(width: u8, rgb: &RGB) -> String {
    let mut line = String::new();
    for _ in 0..width {
        line.push_str(&format!(" {}", color_line(rgb)));
    }
    line
}

pub fn get_color_text(rgb: &RGB) -> String {
    format!("\x1b[38;2;{};{};{}m", rgb.0, rgb.1, rgb.2)
}

pub mod trans {
    use super::RGB;

    pub const LIGHT_BLUE: RGB = RGB(91, 206, 250);
    pub const PINK: RGB = RGB(245, 169, 184);
    pub const WHITE: RGB = RGB(255, 255, 255);
}

pub mod bi {
    use super::RGB;

    pub const PINK: RGB = RGB(214, 2, 112);
    pub const PURPLE: RGB = RGB(155, 79, 150);
    pub const BLUE: RGB = RGB(0, 56, 168);
}
