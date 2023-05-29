use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Command {
    #[arg(value_enum, index = 1)]
    fmt: Format,

    #[arg(
        index = 2,
        help = "Input color in one of the supported representations."
    )]
    input: Vec<String>,
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
enum Format {
    #[value(help = "Hexadecimal representation of RGB color. Example for white: #ffffff")]
    Hex,

    #[value(help = "RGB representation of color. Example for white: 255,255,255")]
    Rgb,

    #[value(help = "HSL representation of color. Example for white 0,0,100")]
    Hsl,

    #[value(help = "HSV representation of color. Example for white: 0,0,100")]
    Hsv,

    #[value(help = "CMYK representation of color. Example for white: 0,0,0,0")]
    Cmyk,
}

fn main() {
    let cfg = Command::parse();

    dbg!(cfg);
}
