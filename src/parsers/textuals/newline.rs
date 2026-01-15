use crate::{Parser, ParserInput, ParserResult, ParsingError, Span};

/// See [`crate::helpers::newline`]
#[derive(Clone, Copy)]
pub struct Newline;

impl Newline {
    pub const fn new() -> Self {
        Self
    }
}

impl Default for Newline {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser<()> for Newline {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<()> {
        let input_str = input.inner();

        let trimmed = if input_str.starts_with("\r\n") {
            2
        } else if input_str.starts_with('\r') || input_str.starts_with('\n') {
            1
        } else {
            return Err(ParsingError::custom(
                input.at().range(0),
                "Expected at least one newline",
            ));
        };

        Ok(Span::ate(input.range(trimmed), ()))
    }
}
