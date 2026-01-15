use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult, ParsingError};

/// See [`crate::helpers::filter`]
#[perfect_derive(Clone, Copy)]
pub struct Filter {
    func: fn(char) -> bool,
}

impl Filter {
    pub const fn new(func: fn(char) -> bool) -> Self {
        Self { func }
    }
}

impl Parser<char> for Filter {
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
