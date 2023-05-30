use std::ops::Range;

use itertools::Itertools;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct CustomError {
    pub msg: String,
    pub span: Range<usize>,
    pub expected: Vec<String>,
    pub found: Vec<String>,
}

impl chumsky::Error<char> for CustomError {
    type Span = Range<usize>;

    type Label = String;

    fn expected_input_found<Iter: IntoIterator<Item = Option<char>>>(
        span: Self::Span,
        expected: Iter,
        found: Option<char>,
    ) -> Self {
        Self {
            msg: String::from("Unexpected input. Expected"),
            span,
            expected: expected
                .into_iter()
                .filter_map(|opt| opt.map(|ch| ch.to_string()))
                .collect(),
            found: found.map(|ch| ch.to_string()).into_iter().collect(),
        }
    }

    fn with_label(self, label: Self::Label) -> Self {
        Self { msg: label, ..self }
    }

    fn merge(self, other: Self) -> Self {
        Self {
            msg: other.msg,
            span: self.span.start..other.span.end,
            expected: self
                .expected
                .into_iter()
                .chain(other.expected.into_iter())
                .unique()
                .collect(),
            found: self
                .found
                .into_iter()
                .chain(other.found.into_iter())
                .unique()
                .collect(),
        }
    }
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expected = self.expected.join(", ");
        let found = self.found.concat();

        f.write_fmt(format_args!(
            "{}: {:?} but found {:?}",
            self.msg, expected, found
        ))
    }
}
