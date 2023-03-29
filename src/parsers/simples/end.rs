use crate::{Eaten, PResult, Parser, ParserInput};

#[derive(Clone)]
pub struct End;

impl End {
    pub fn new() -> Self {
        Self
    }
}

impl Parser<()> for End {
    fn parse_inner<'a>(&self, input: &mut ParserInput<'a>) -> PResult<()> {
        if input.inner().is_empty() {
            Ok(Eaten::ate(input.range(0), ()))
        } else {
            Err(input.range(0).custom_err("Expected end of input"))
        }
    }
}
