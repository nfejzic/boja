use super::hsl::Hsl;
use crate::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

impl From<(u8, u8, u8)> for Rgb {
    fn from((red, green, blue): (u8, u8, u8)) -> Self {
        Self { red, green, blue }
    }
}

impl From<Hsl> for Rgb {
    fn from(value: Hsl) -> Self {
        let (h, s, l) = value.into_tuple();

        debug_assert!((0..360).contains(&h));

        let s = s as f64 / 100f64;
        let l = l as f64 / 100f64;

        let c: f64 = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h as f64 / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r1, g1, b1) = match h % 360 {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            300..=359 => (c, 0.0, x),
            360.. => unreachable!("hsl hue value must be between 0 and 360"),
        };

        let red = ((r1 + m) * 255.0).round() as i64;
        let green = ((g1 + m) * 255.0).round() as i64;
        let blue = ((b1 + m) * 255.0).round() as i64;

        debug_assert!(u8::try_from(red).is_ok());
        debug_assert!(u8::try_from(green).is_ok());
        debug_assert!(u8::try_from(blue).is_ok());

        Self {
            red: red as u8,
            green: green as u8,
            blue: blue as u8,
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
    pub fn to_rgb(self) -> String {
        format!("rgb({}, {}, {})", self.red, self.green, self.blue)
    }

    pub fn to_hex(self) -> String {
        format!("{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }

    pub fn into_tuple(self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }
}

#[cfg(test)]
mod tests {
    use crate::format::hsl::Hsl;

    use super::Rgb;

    #[test]
    fn black_from_hsl() {
        let hsl = Hsl::from((0, 0, 0));

        let rgb = Rgb::from(hsl);

        assert_eq!(
            rgb,
            Rgb {
                red: 0,
                green: 0,
                blue: 0
            }
        )
    }

    #[test]
    fn white_from_hsl() {
        let hsl = Hsl::from((0, 0, 100));

        let rgb = Rgb::from(hsl);

        assert_eq!(
            rgb,
            Rgb {
                red: 255,
                green: 255,
                blue: 255
            }
        )
    }

    #[test]
    fn red_from_hsl() {
        let hsl = Hsl::from((0, 100, 50));

        let rgb = Rgb::from(hsl);

        assert_eq!(
            rgb,
            Rgb {
                red: 255,
                green: 0,
                blue: 0
            }
        )
    }

    #[test]
    fn olive_from_hsl() {
        let hsl = Hsl::from((60, 100, 25));

        let rgb = Rgb::from(hsl);

        assert_eq!(
            rgb,
            Rgb {
                red: 128,
                green: 128,
                blue: 0
            }
        )
    }
}
