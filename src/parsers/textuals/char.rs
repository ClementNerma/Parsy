use crate::{PResult, Parser, ParserInput};

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

        input
            .try_eat_char()
            .filter(|eaten| eaten.data == self.char)
            .ok_or_else(|| start.expected_char(self.char))
    }
}
