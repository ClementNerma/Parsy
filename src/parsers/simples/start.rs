use crate::{Eaten, PResult, Parser, ParserInput};

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
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<()> {
        if input.offset() == 0 {
            Ok(Eaten::ate(input.range(0), ()))
        } else {
            Err(input.at().custom_err("Expected start of input", 0))
        }
    }
}
