//! Implementation of the parser for various color formats.

mod utils;

use crate::Color;
use crate::{error::CustomError, format::Hsl};
use chumsky::{
    primitive::{choice, end, just},
    Parser,
};

use self::utils::{digit, n_digits, numbers_separated_by, prefix};

pub fn parse_color(input: &str) -> Result<Color, Vec<CustomError>> {
    let parser = choice((parse_hex(), parse_rgb(), parse_hsl()));
    parser.parse(input)
}

fn parse_hex() -> impl Parser<char, Color, Error = CustomError> {
    let hex_digit = digit(16);
    let three_hex = hex_digit.repeated().exactly(3).then_ignore(end());
    let six_hex = hex_digit.repeated().exactly(6).then_ignore(end());

    choice((six_hex, three_hex))
        .map(|colors| Color::try_from(&colors[..]))
        .unwrapped()
        .map_err(|err| CustomError {
            msg: String::from("Unexpected input. Expected"),
            span: err.span.clone(),
            expected: vec![String::from("Hex digit")],
            found: err.found,
        })
}

fn parse_rgb() -> impl Parser<char, Color, Error = CustomError> {
    prefix("rgb")
        .ignore_then(numbers_separated_by(n_digits(3, 10), 3, ','))
        .then_ignore(just(')'))
        .then_ignore(end())
        .map(|rgb| Color::try_from(&rgb[..]))
        .try_map(|res: Result<Color, _>, span| match res {
            Ok(color) => Ok(color),
            Err(err) => Err(CustomError {
                msg: String::from("Invalid RGB value. Expected"),
                span,
                expected: vec![String::from("Value in range 0-255")],
                found: vec![err.to_string()],
            }),
        })
}

fn parse_hsl() -> impl Parser<char, Color, Error = CustomError> {
    prefix("hsl")
        .ignore_then(numbers_separated_by(n_digits(3, 10), 3, ','))
        .then_ignore(just(')'))
        .then_ignore(end())
        .try_map(|hsl, span| {
            Hsl::try_from(&hsl[..]).map_err(|err| CustomError {
                msg: String::from("Invalid HSL value. Expected"),
                span,
                expected: vec![String::from(
                    "Values: 0-360 for hue, 0-100 for saturation and lightness",
                )],
                found: vec![err.to_string()],
            })
        })
        .map(Color::from)
}
