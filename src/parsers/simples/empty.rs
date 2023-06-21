use crate::{Eaten, PResult, Parser, ParserInput};

#[derive(Clone, Copy)]
pub struct Empty;

impl Empty {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }
}

impl Parser<()> for Empty {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<()> {
        Ok(Eaten::ate(input.range(0), ()))
    }
}
