use crate::{Parser, ParserInput, ParserResult, ParsingError};

/// See [`whitespace`](`crate::parsers::helpers::whitespace`)
#[derive(Clone, Copy)]
pub struct Whitespace {
    no_newline: bool,
}

impl Whitespace {
    pub const fn new() -> Self {
        Self { no_newline: false }
    }

    pub const fn no_newline(mut self) -> Self {
        self.no_newline = true;
        self
    }
}

impl Default for Whitespace {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser<()> for Whitespace {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<()> {
        let start = input.at();

        let c = input
            .try_eat_char()
            .ok_or_else(|| ParsingError::custom(start.range(0), "No character left"))?;

        if c.data.is_whitespace() {
            Ok(c.forge_here(()))
        } else {
            Err(ParsingError::custom(
                start.range(c.data.len_utf8()),
                "Character filter failed",
            ))
        }
    }
}
