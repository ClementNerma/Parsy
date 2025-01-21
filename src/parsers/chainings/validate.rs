use std::{borrow::Cow, marker::PhantomData};

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult, ParsingError};

#[perfect_derive(Debug, Clone, Copy)]
pub struct Validate<T, P: Parser<T>, F: Fn(&T) -> bool> {
    parser: P,
    validator: F,
    err_msg: Option<&'static str>,
    critical: bool,
    _p: PhantomData<T>,
}

impl<T, P: Parser<T>, F: Fn(&T) -> bool> Validate<T, P, F> {
    pub fn new(parser: P, validator: F) -> Self {
        Self {
            parser,
            validator,
            err_msg: None,
            critical: false,
            _p: PhantomData,
        }
    }

    pub fn with_custom_msg(mut self, msg: &'static str) -> Self {
        self.err_msg = Some(msg);
        self
    }

    pub fn as_critical(mut self) -> Self {
        self.critical = true;
        self
    }
}

impl<T, P: Parser<T>, F: Fn(&T) -> bool> Parser<T> for Validate<T, P, F> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<T> {
        let start = input.at();
        let parsed = self.parser.parse(input)?;

        if (self.validator)(&parsed.data) {
            Ok(parsed)
        } else {
            let msg = match &self.err_msg {
                Some(msg) => msg,
                None => "validator failed",
            };

            let mut err = ParsingError::custom(start.range(parsed.at.len), msg);

            if self.critical {
                err = err.criticalize(Cow::Borrowed(msg));
            }

            Err(err)
        }
    }
}
