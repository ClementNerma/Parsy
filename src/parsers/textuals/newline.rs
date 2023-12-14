use crate::{Eaten, PResult, Parser, ParserInput};

#[derive(Clone, Copy)]
pub struct Newline;

impl Newline {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Newline {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser<()> for Newline {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<()> {
        let input_str = input.inner();

        let trimmed = if input_str.starts_with("\r\n") {
            2
        } else if input_str.starts_with('\r') || input_str.starts_with('\n') {
            1
        } else {
            return Err(input.at().custom_err("Expected at least one newline"));
        };

        Ok(Eaten::ate(input.range(trimmed), ()))
    }
}
