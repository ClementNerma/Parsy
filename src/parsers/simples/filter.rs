use perfect_derive::perfect_derive;

use crate::{ParserResult, Parser, ParserInput, ParsingError};

#[perfect_derive(Clone, Copy)]
pub struct Filter<F: Fn(char) -> bool> {
    func: F,
}

impl<F: Fn(char) -> bool> Filter<F> {
    pub fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F: Fn(char) -> bool> Parser<char> for Filter<F> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<char> {
        let start = input.at();

        let c = input
            .try_eat_char()
            .ok_or_else(|| ParsingError::custom(start.range(0), "No character left"))?;

        if (self.func)(c.data) {
            Ok(c)
        } else {
            Err(ParsingError::custom(
                start.range(1),
                "Character filter failed",
            ))
        }
    }
}
