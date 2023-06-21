use crate::{Eaten, PResult, Parser, ParserInput};

#[derive(Clone)]
pub struct Start;

impl Start {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }
}

impl Parser<()> for Start {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<()> {
        if input.offset() == 0 {
            Ok(Eaten::ate(input.range(0), ()))
        } else {
            Err(input.range(0).custom_err("Expected start of input"))
        }
    }
}
