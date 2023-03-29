use crate::{Eaten, PResult, Parser, ParserInput};

#[derive(Clone, Copy)]
pub struct Whitespaces {
    multiple: bool,
    no_newline: bool,
}

impl Whitespaces {
    pub fn new() -> Self {
        Self {
            multiple: false,
            no_newline: false,
        }
    }

    pub fn at_least_one(mut self) -> Self {
        self.multiple = true;
        self
    }

    pub fn no_newline(mut self) -> Self {
        self.no_newline = true;
        self
    }
}

impl Parser<()> for Whitespaces {
    fn parse_inner<'a>(&self, input: &mut ParserInput<'a>) -> PResult<()> {
        let trimmed = if self.no_newline {
            input
                .inner()
                .trim_start_matches(|c: char| c.is_whitespace() && c != '\n' && c != '\r')
        } else {
            input.inner().trim_start()
        };

        let trimmed = input.inner().as_bytes().len() - trimmed.as_bytes().len();

        if self.multiple && trimmed == 0 {
            Err(input
                .range(0)
                .custom_err("Expected at least one whitespace"))
        } else {
            Ok(Eaten::ate(input.range(trimmed), ()))
        }
    }
}
