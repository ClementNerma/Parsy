use crate::{Parser, ParserInput, ParserResult, ParsingError};

/// See [`crate::helpers::char`]
#[derive(Clone, Copy)]
pub struct Char {
    char: char,
}

impl Char {
    pub const fn new(char: char) -> Self {
        Self { char }
    }
}

impl Parser<char> for Char {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<char> {
        let start = input.at();

        let span = input
            .try_eat_char()
            .ok_or_else(|| ParsingError::expected_char(start.range(0), self.char))?;

        if span.data == self.char {
            Ok(span)
        } else {
            Err(ParsingError::expected_char(
                start.range(span.data.len_utf8()),
                self.char,
            ))
        }
    }
}
