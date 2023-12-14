use crate::{Eaten, PResult, Parser, ParserInput};

#[derive(Clone, Copy)]
pub struct Empty;

impl Empty {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Empty {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser<()> for Empty {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<()> {
        Ok(Eaten::ate(input.range(0), ()))
    }
}
