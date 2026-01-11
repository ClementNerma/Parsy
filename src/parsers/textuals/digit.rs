use crate::{Parser, ParserInput, ParserResult, ParsingError};

#[derive(Clone, Copy)]
pub struct Digit {
    radix: u32,
}

impl Digit {
    pub const fn new(radix: u32) -> Self {
        Self { radix }
    }
}

impl Parser<()> for Digit {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<()> {
        let start = input.at();

        let c = input
            .try_eat_char()
            .ok_or_else(|| ParsingError::custom(start.range(0), "No character left"))?;

        if c.data.is_digit(self.radix) {
            Ok(c.replace(()))
        } else {
            Err(ParsingError::custom(
                start.range(c.data.len_utf8()),
                "Character filter failed",
            ))
        }
    }
}
