use crate::{Eaten, PResult, Parser, ParserInput};

#[derive(Clone, Copy)]
pub struct Empty;

impl Empty {
    pub fn new() -> Self {
        Self
    }
}

impl Parser<()> for Empty {
    fn parse_inner<'a>(&self, input: &mut ParserInput<'a>) -> PResult<()> {
        Ok(Eaten::ate(input.range(0), ()))
    }
}
