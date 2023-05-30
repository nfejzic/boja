use crate::Color;

use super::Rgb;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Hsv {
    hue: u16,
    saturation: u8,
    value: u8,
}

impl From<Rgb> for Hsv {
    fn from(rgb: Rgb) -> Self {
        let (red, green, blue) = rgb.into_tuple();

        let rp = red as f64 / 255.0;
        let gp = green as f64 / 255.0;
        let bp = blue as f64 / 255.0;

        let c_max = rp.max(gp).max(bp);
        let c_min = rp.min(gp).min(bp);
        let delta = c_max - c_min;

        let value = c_max;

        let hue = if delta == 0f64 {
            0.0
        } else {
            match c_max {
                max if max == rp => ((gp - bp) / delta) % 6.0 * 60.0,
                max if max == gp => (2.0 + (bp - rp) / delta) * 60.0,
                max if max == bp => (4.0 + (rp - gp) / delta) * 60.0,
                _ => unreachable!("c_max must be equal to red, green or blue"),
            }
        };

        let hue = if hue < 0.0 { 360.0 - hue.abs() } else { hue };
        let saturation = if delta == 0f64 { 0.0 } else { delta / c_max };

        Self {
            hue: hue.round() as u16,
            saturation: (saturation * 100.0).round() as u8,
            value: (value * 100.0).round() as u8,
        }
    }
}

impl From<Hsv> for Color {
    fn from(hsv: Hsv) -> Self {
        Color::from(Rgb::from(hsv))
    }
}

impl From<(u16, u8, u8)> for Hsv {
    fn from((hue, saturation, value): (u16, u8, u8)) -> Self {
        Self {
            hue,
            saturation,
            value,
        }
    }
}

impl TryFrom<&[String]> for Hsv {
    type Error = anyhow::Error;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        let hue = value
            .get(0)
            .ok_or(anyhow::anyhow!("Expected value for hue"))?
            .parse::<u16>()?
            % 360;

        let saturation = value
            .get(1)
            .ok_or(anyhow::anyhow!("Expected value for saturation"))?
            .parse::<u8>()?;

        if saturation > 100 {
            anyhow::bail!("Saturation must be in range 0..100");
        }

        let value = value
            .get(2)
            .ok_or(anyhow::anyhow!("Expected value for value"))?
            .parse::<u8>()?;

        if value > 100 {
            anyhow::bail!("Value must be in range 0..100");
        }

        Ok(Hsv {
            hue,
            saturation,
            value,
        })
    }
}

impl std::fmt::Display for Hsv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "hsv({}, {}, {})",
            self.hue, self.saturation, self.value
        ))
    }
}

impl Hsv {
    pub fn into_tuple(self) -> (u16, u8, u8) {
        (self.hue, self.saturation, self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::format::{hsv::Hsv, Rgb};

    #[test]
    fn black_from_rgb() {
        let black = Rgb::from((0, 0, 0));

        let hsv = Hsv::from(black);
        assert_eq!(
            hsv,
            Hsv {
                hue: 0,
                saturation: 0,
                value: 0
            }
        );
    }

    #[test]
    fn white_from_rgb() {
        let black = Rgb::from((255, 255, 255));

        let hsv = Hsv::from(black);
        assert_eq!(
            hsv,
            Hsv {
                hue: 0,
                saturation: 0,
                value: 100
            }
        );
    }

    #[test]
    fn red_from_rgb() {
        let black = Rgb::from((255, 0, 0));

        let hsv = Hsv::from(black);
        assert_eq!(
            hsv,
            Hsv {
                hue: 0,
                saturation: 100,
                value: 100
            }
        );
    }

    #[test]
    fn olive_from_rgb() {
        let black = Rgb::from((128, 128, 0));

        let hsv = Hsv::from(black);
        assert_eq!(
            hsv,
            Hsv {
                hue: 60,
                saturation: 100,
                value: 50
            }
        );
    }
}
