//! Implementation of the parser for various color formats.

use crate::error::CustomError;
use crate::Color;
use chumsky::{
    primitive::{choice, end, filter, just},
    text::whitespace,
    Parser,
};

pub fn parse_color(input: &str) -> Result<Color, Vec<CustomError>> {
    let parser = choice((parse_hex(), parse_rgb()));
    parser.parse(input)
}

fn parse_hex() -> impl Parser<char, Color, Error = CustomError> {
    let hex_digit =
        filter(|input: &char| input.is_ascii_hexdigit()).map_err(|err: CustomError| CustomError {
            msg: String::from("Unexpected input. Expected"),
            span: err.span,
            expected: vec![String::from("a hexadecimal digit")],
            found: err.found,
        });

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
    let prefix = filter(|input: &char| input.is_alphabetic())
        .repeated()
        .exactly(3)
        .map(|input| input.iter().collect::<String>())
        .then(just("("))
        .try_map(|(rgb, parenth), span| {
            if &rgb.to_lowercase() != "rgb" && parenth != "(" {
                Err(CustomError {
                    msg: String::from("Unexpected input. Expected"),
                    span,
                    expected: vec![String::from("rgb(")],
                    found: vec![rgb, parenth.to_string()],
                })
            } else {
                Ok(rgb)
            }
        });

    let digit = filter(|input: &char| input.is_ascii_digit());
    let single_val = digit
        .repeated()
        .at_least(1)
        .at_most(3)
        .map(|input| input.iter().collect::<String>());

    let parser = prefix
        .ignore_then(
            single_val
                .then_ignore(just(','))
                .then_ignore(whitespace())
                .repeated()
                .exactly(2),
        )
        .then(single_val)
        .then_ignore(just(")"))
        .map(|(mut rg, b)| {
            rg.push(b);
            rg
        })
        .then_ignore(end());

    parser
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
