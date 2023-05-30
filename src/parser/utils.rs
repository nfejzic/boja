use chumsky::{
    primitive::{filter, just},
    text::whitespace,
    Parser,
};

use crate::error::CustomError;

pub fn prefix(prefix: &str) -> impl Parser<char, String, Error = CustomError> + '_ {
    filter(|input: &char| input.is_alphabetic())
        .repeated()
        .exactly(3)
        .map(|input| input.iter().collect::<String>())
        .then(just("("))
        .try_map(move |(rgb, parenth), span| {
            if rgb.to_lowercase() != prefix || parenth != "(" {
                Err(CustomError {
                    msg: String::from("Unexpected input. Expected"),
                    span,
                    expected: vec![String::from("rgb(")],
                    found: vec![rgb, parenth.to_string()],
                })
            } else {
                Ok(rgb)
            }
        })
}

pub fn digit(radix: u32) -> impl Parser<char, char, Error = CustomError> + Copy {
    filter(move |input: &char| input.is_digit(radix))
}

pub fn n_digits(n: usize, radix: u32) -> impl Parser<char, String, Error = CustomError> + Copy {
    digit(radix)
        .repeated()
        .at_least(1)
        .at_most(n)
        .map(|input| input.iter().collect::<String>())
}

pub fn numbers_separated_by<P>(
    digits_parser: P,
    n: usize,
    separator: char,
) -> impl Parser<char, Vec<String>, Error = CustomError> + Copy
where
    P: Parser<char, String, Error = CustomError>,
    P: Copy,
{
    n_digits(3, 10)
        .then_ignore(just(separator))
        .then_ignore(whitespace())
        .repeated()
        .exactly(n.saturating_sub(1))
        .then(digits_parser)
        .map(|(mut rg, b)| {
            rg.push(b);
            rg
        })
}
