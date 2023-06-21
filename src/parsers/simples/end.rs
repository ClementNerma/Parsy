use crate::{Eaten, PResult, Parser, ParserInput};

#[derive(Clone)]
pub struct End;

impl End {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }
}

impl Parser<()> for End {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<()> {
        if input.inner().is_empty() {
            Ok(Eaten::ate(input.range(0), ()))
        } else {
            Err(input.range(0).custom_err("Expected end of input"))
        }
    }
}
