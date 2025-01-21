use crate::{Parser, ParserInput, ParserResult, ParsingError, Span};

#[derive(Clone, Copy)]
pub struct Whitespaces {
    at_least_one: bool,
    no_newline: bool,
}

impl Whitespaces {
    pub fn new() -> Self {
        Self {
            at_least_one: false,
            no_newline: false,
        }
    }

    pub fn at_least_one(mut self) -> Self {
        self.at_least_one = true;
        self
    }

    pub fn no_newline(mut self) -> Self {
        self.no_newline = true;
        self
    }
}

impl Default for Whitespaces {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser<()> for Whitespaces {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<()> {
        let input_str = input.inner();

        let trimmed = if self.no_newline {
            input_str.trim_start_matches(|c: char| c.is_whitespace() && c != '\n' && c != '\r')
        } else {
            input_str.trim_start()
        };

        let trimmed = input_str.len() - trimmed.len();

        if self.at_least_one && trimmed == 0 {
            Err(ParsingError::custom(
                input.at().range(0),
                "Expected at least one whitespace",
            ))
        } else {
            Ok(Span::ate(input.range(trimmed), ()))
        }
    }
}
