use crate::{Parser, ParserInput, ParserResult, ParsingError, Span};

/// See [`whitespaces`](`crate::parsers::helpers::whitespaces`)
#[derive(Clone, Copy)]
pub struct Whitespaces {
    at_least_one: bool,
    no_newline: bool,
}

impl Whitespaces {
    pub const fn new() -> Self {
        Self {
            at_least_one: false,
            no_newline: false,
        }
    }

    /// Don't accept newline symbols
    pub const fn no_newline(mut self) -> Self {
        self.no_newline = true;
        self
    }

    /// Require at least one whitespace for the parser to succeed
    pub const fn at_least_one(mut self) -> Self {
        self.at_least_one = true;
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
