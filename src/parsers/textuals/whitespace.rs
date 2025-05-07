use crate::{Parser, ParserInput, ParserResult, ParsingError};

#[derive(Clone, Copy)]
pub struct Whitespace {
    no_newline: bool,
}

impl Whitespace {
    pub fn new() -> Self {
        Self { no_newline: false }
    }

    pub fn no_newline(mut self) -> Self {
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
            Ok(c.replace(()))
        } else {
            Err(ParsingError::custom(
                start.range(1),
                "Character filter failed",
            ))
        }
    }
}
