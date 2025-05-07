use crate::{Parser, ParserInput, ParserResult, ParsingError, Span};

#[derive(Clone, Copy)]
pub struct End;

impl End {
    pub fn new() -> Self {
        Self
    }
}

impl Default for End {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser<()> for End {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<()> {
        if input.inner().is_empty() {
            Ok(Span::ate(input.range(0), ()))
        } else {
            Err(ParsingError::custom(
                input.at().range(0),
                "Expected end of input",
            ))
        }
    }
}
