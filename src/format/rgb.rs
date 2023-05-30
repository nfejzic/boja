use crate::Color;

pub(crate) struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl From<Color> for Rgb {
    fn from(color: Color) -> Self {
        Self {
            red: color.red,
            green: color.green,
            blue: color.blue,
        }
    }
}

impl From<Rgb> for Color {
    fn from(rgb: Rgb) -> Self {
        Self {
            red: rgb.red,
            green: rgb.green,
            blue: rgb.blue,
        }
    }
}

impl Rgb {
    pub fn to_rgb(&self) -> String {
        format!("{}, {}, {}", self.red, self.green, self.blue)
    }

    pub fn to_hex(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }
}
