//! Implementation of the parser for various color formats.

use chumsky::{
    prelude::Simple,
    primitive::{choice, end, just, one_of},
    text::{int, whitespace},
    Parser,
};

use crate::Color;

pub fn parse_color(input: &str) -> Result<Color, Vec<Simple<char>>> {
    let parser = choice((parse_hex(), parse_rgb()));
    parser.parse(input)
}

fn parse_hex() -> impl Parser<char, Color, Error = Simple<char>> {
    let hex_digit = one_of::<_, _, Simple<char>>("0123456789abcdefABCDEF");

    let three_hex = hex_digit.clone().repeated().exactly(3).then_ignore(end());
    let six_hex = hex_digit.repeated().exactly(6).then_ignore(end());

    choice((six_hex, three_hex))
        .map(|colors| Color::try_from(&colors[..]))
        .unwrapped()
}

fn parse_rgb() -> impl Parser<char, Color, Error = Simple<char>> {
    let parser = int(10)
        .then_ignore(just(','))
        .then_ignore(whitespace())
        .repeated()
        .exactly(2)
        .chain(int(10))
        .then(end());

    parser
        .map(|(input, _): (Vec<String>, _)| Color::try_from(&input[..]))
        .unwrapped()
}
