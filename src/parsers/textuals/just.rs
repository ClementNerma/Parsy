use crate::{Parser, ParserInput, ParserResult, ParsingError};

/// See [`crate::helpers::just`]
#[derive(Clone, Copy)]
pub struct Just {
    str: &'static str,
}

impl Just {
    pub const fn new(str: &'static str) -> Self {
        Self { str }
    }
}

impl Parser<&'static str> for Just {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<&'static str> {
        let start = input.at();

        let span = input
            // Try to eat the string
            .try_eat(self.str.len())
            // Otherwise, generate an error
            .ok_or_else(|| ParsingError::expected_str(start.range(0), self.str))?;

        // Ensure it was correctly parsed
        if span.data == self.str {
            // If so, replace success data with the stored string to get a 'static lifetime
            Ok(span.forge_here(self.str))
        } else {
            // Otherwise, generate an error
            Err(ParsingError::expected_str(
                start.range(span.at.len),
                self.str,
            ))
        }
    }
}
