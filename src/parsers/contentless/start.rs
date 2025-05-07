use crate::{Parser, ParserInput, ParserResult, ParsingError, Span};

#[derive(Clone, Copy)]
pub struct Start;

impl Start {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Start {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser<()> for Start {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<()> {
        if input.offset() == 0 {
            Ok(Span::ate(input.range(0), ()))
        } else {
            Err(ParsingError::custom(
                input.at().range(0),
                "Expected start of input",
            ))
        }
    }
}
