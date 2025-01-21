use std::collections::HashSet;

use crate::{PResult, Parser, ParserInput, ParsingError};

pub struct OneOfChars {
    set: HashSet<char>,
}

impl OneOfChars {
    pub fn new(set: HashSet<char>) -> Self {
        Self { set }
    }
}

impl Parser<char> for OneOfChars {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<char> {
        let start = input.at();

        let span = input
            .try_eat_char()
            .ok_or_else(|| ParsingError::custom(start.range(0), "expected a character to match"))?;

        if self.set.contains(&span.data) {
            Ok(span)
        } else {
            Err(ParsingError::custom(
                start.range(1),
                "character did not match against the provided set",
            ))
        }
    }
}
