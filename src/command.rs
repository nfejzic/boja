//! Command line arguments parsing.

use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Command {
    #[arg(value_enum, index = 1)]
    fmt: Format,

    #[arg(
        index = 2,
        help = "Input color in one of the supported representations."
    )]
    input: Vec<String>,
}

impl Command {
    pub fn init() -> Self {
        Command::parse()
    }

    pub fn input(&self) -> String {
        self.input.concat()
    }

    pub fn fmt(&self) -> Format {
        self.fmt
    }
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum Format {
    #[value(help = "Hexadecimal representation of RGB color. Example for white: ffffff")]
    Hex,

    #[value(help = "RGB representation of color. Example for white: rgb(255, 255, 255)")]
    Rgb,

    #[value(help = "HSL representation of color. Example for white hsl(0, 0, 100)")]
    Hsl,

    #[value(help = "HSV representation of color. Example for white: hsv(0, 0, 100)")]
    Hsv,

    #[value(help = "CMYK representation of color. Example for white: cmyk(0, 0, 0, 0)")]
    Cmyk,
}
