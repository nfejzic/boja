mod command;
pub mod error;
mod format;
mod parser;

use format::{Hsl, Rgb};
use itertools::Itertools;
use std::error::Error;

pub use command::*;
pub use parser::parse_color;

/// Internal color representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn convert(&self, fmt: Format) -> String {
        match fmt {
            Format::Hex => self.convert_to_hex(),
            Format::Rgb => self.convert_to_rgb(),
            Format::Hsl => self.convert_to_hsl(),
            Format::Hsv => self.convert_to_hsv(),
            Format::Cmyk => self.convert_to_cmyk(),
        }
    }

    fn convert_to_hex(self) -> String {
        Rgb::from(self).to_hex()
    }

    fn convert_to_rgb(self) -> String {
        Rgb::from(self).to_rgb()
    }

    fn convert_to_hsl(self) -> String {
        Hsl::from(Rgb::from(self)).to_string()
    }

    fn convert_to_hsv(self) -> String {
        todo!()
    }

    fn convert_to_cmyk(self) -> String {
        todo!()
    }
}

impl TryFrom<&[char]> for Color {
    type Error = Box<dyn Error>;

    fn try_from(value: &[char]) -> Result<Self, Self::Error> {
        let value: Vec<char> = if value.len() == 3 {
            value.iter().flat_map(|ch| [*ch, *ch]).collect()
        } else {
            value.to_vec()
        };

        match value.len() {
            6 => {
                let red = u8::from_str_radix(&value[0..2].iter().collect::<String>(), 16)?;
                let green = u8::from_str_radix(&value[2..4].iter().collect::<String>(), 16)?;
                let blue = u8::from_str_radix(&value[4..6].iter().collect::<String>(), 16)?;

                Ok(Self { red, green, blue })
            }
            _ => todo!(),
        }
    }
}

impl TryFrom<&[Vec<char>]> for Color {
    type Error = anyhow::Error;

    fn try_from(value: &[Vec<char>]) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            anyhow::bail!("Bad input provided: {:?}", value)
        }

        todo!()
    }
}

impl TryFrom<&[&str]> for Color {
    type Error = anyhow::Error;

    fn try_from(value: &[&str]) -> Result<Self, Self::Error> {
        let (red, green, blue) = value
            .iter()
            .tuple_windows()
            .next()
            .ok_or(anyhow::anyhow!("Bad input provided {:?}", value))?;

        Ok(Color {
            red: red.parse()?,
            green: green.parse()?,
            blue: blue.parse()?,
        })
    }
}

impl TryFrom<&[String]> for Color {
    type Error = anyhow::Error;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        let (red, green, blue) = value
            .iter()
            .map(String::as_str)
            .tuple_windows()
            .next()
            .ok_or(anyhow::anyhow!("Bad input: {:?}", value))?;

        Color::try_from(&[red, green, blue][..])
    }
}
