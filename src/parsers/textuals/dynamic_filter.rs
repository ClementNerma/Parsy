use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult, ParsingError};

/// See [`crate::helpers::dynamic_filter`]
#[perfect_derive(Clone, Copy)]
pub struct DynamicFilter<F: Fn(char) -> bool> {
    func: F,
}

impl<F: Fn(char) -> bool> DynamicFilter<F> {
    pub const fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F: Fn(char) -> bool> Parser<char> for DynamicFilter<F> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<char> {
        let start = input.at();

        let c = input
            .try_eat_char()
            .ok_or_else(|| ParsingError::custom(start.range(0), "No character left"))?;

        if (self.func)(c.data) {
            Ok(c)
        } else {
            Err(ParsingError::custom(
                start.range(c.data.len_utf8()),
                "Character filter failed",
            ))
        }
    }
}
