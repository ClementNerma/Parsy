use crate::{Eaten, PResult, Parser, ParserInput};

#[derive(Clone)]
pub struct Start;

impl Start {
    pub fn new() -> Self {
        Self
    }
}

impl Parser<()> for Start {
    fn parse_inner<'a>(&self, input: &mut ParserInput<'a>) -> PResult<()> {
        if input.offset() == 0 {
            Ok(Eaten::ate(input.range(0), ()))
        } else {
            Err(input.range(0).custom_err("Expected start of input"))
        }
    }
}
