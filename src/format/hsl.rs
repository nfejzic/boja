use crate::Color;

use super::Rgb;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Hsl {
    hue: u16,
    saturation: u8,
    lightness: u8,
}

impl From<Rgb> for Hsl {
    fn from(rgb: Rgb) -> Self {
        let (red, green, blue) = rgb.into_tuple();

        let rp = red as f64 / 255.0;
        let gp = green as f64 / 255.0;
        let bp = blue as f64 / 255.0;

        let c_max = rp.max(gp).max(bp);
        let c_min = rp.min(gp).min(bp);
        let delta = c_max - c_min;

        let lightness = (c_max + c_min) / 2.0;

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

        let saturation = if delta == 0f64 {
            0.0
        } else {
            delta / (1.0 - (2.0 * lightness - 1.0).abs())
        };

        Self {
            hue: hue.round() as u16,
            saturation: (saturation * 100.0).round() as u8,
            lightness: (lightness * 100.0).round() as u8,
        }
    }
}

impl From<Hsl> for Color {
    fn from(hsl: Hsl) -> Self {
        Color::from(Rgb::from(hsl))
    }
}

impl From<(u16, u8, u8)> for Hsl {
    fn from((hue, saturation, lightness): (u16, u8, u8)) -> Self {
        Self {
            hue,
            saturation,
            lightness,
        }
    }
}

impl TryFrom<&[String]> for Hsl {
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

        let lightness = value
            .get(2)
            .ok_or(anyhow::anyhow!("Expected value for lightness"))?
            .parse::<u8>()?;

        if lightness > 100 {
            anyhow::bail!("Lightness must be in range 0..100");
        }

        Ok(Hsl {
            hue,
            saturation,
            lightness,
        })
    }
}

impl std::fmt::Display for Hsl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "hsl({}, {}, {})",
            self.hue, self.saturation, self.lightness
        ))
    }
}

impl Hsl {
    pub fn into_tuple(self) -> (u16, u8, u8) {
        (self.hue, self.saturation, self.lightness)
    }
}

#[cfg(test)]
mod tests {
    use crate::format::{hsl::Hsl, Rgb};

    #[test]
    fn black_from_rgb() {
        let black = Rgb::from((0, 0, 0));

        let hsl = Hsl::from(black);
        assert_eq!(
            hsl,
            Hsl {
                hue: 0,
                saturation: 0,
                lightness: 0
            }
        );
    }

    #[test]
    fn white_from_rgb() {
        let black = Rgb::from((255, 255, 255));

        let hsl = Hsl::from(black);
        assert_eq!(
            hsl,
            Hsl {
                hue: 0,
                saturation: 0,
                lightness: 100
            }
        );
    }

    #[test]
    fn red_from_rgb() {
        let black = Rgb::from((255, 0, 0));

        let hsl = Hsl::from(black);
        assert_eq!(
            hsl,
            Hsl {
                hue: 0,
                saturation: 100,
                lightness: 50
            }
        );
    }

    #[test]
    fn olive_from_rgb() {
        let black = Rgb::from((128, 128, 0));

        let hsl = Hsl::from(black);
        assert_eq!(
            hsl,
            Hsl {
                hue: 60,
                saturation: 100,
                lightness: 25
            }
        );
    }
}
