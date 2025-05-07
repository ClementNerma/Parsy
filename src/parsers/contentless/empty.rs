use crate::{Parser, ParserInput, ParserResult, Span};

#[derive(Clone, Copy)]
pub struct Empty;

impl Empty {
    pub const fn new() -> Self {
        Self
    }
}

impl Default for Empty {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser<()> for Empty {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<()> {
        Ok(Span::ate(input.range(0), ()))
    }
}
