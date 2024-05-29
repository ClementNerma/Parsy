use crate::{PResult, Parser, ParserInput, ParsingError};

#[derive(Clone, Copy)]
pub struct Char {
    char: char,
}

impl Char {
    pub fn new(char: char) -> Self {
        Self { char }
    }
}

impl Parser<char> for Char {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<char> {
        let start = input.at();

        let eaten = input
            .try_eat_char()
            .ok_or_else(|| ParsingError::expected_char(start.range(0), self.char))?;

        if eaten.data == self.char {
            Ok(eaten)
        } else {
            Err(ParsingError::expected_char(start.range(1), self.char))
        }
    }
}
