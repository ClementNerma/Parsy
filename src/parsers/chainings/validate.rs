use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

pub struct Validate<T, P: Parser<T>, F: Fn(&T) -> bool> {
    parser: P,
    validator: F,
    err_msg: Option<&'static str>,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>, F: Fn(&T) -> bool> Validate<T, P, F> {
    pub fn new(parser: P, validator: F) -> Self {
        Self {
            parser,
            validator,
            err_msg: None,
            _t: PhantomData,
        }
    }

    pub fn with_err_msg(mut self, msg: &'static str) -> Self {
        self.err_msg = Some(msg);
        self
    }
}

// NOTE: This is required because of https://github.com/rust-lang/rust/issues/26925
impl<T, P: Parser<T> + Clone, F: Fn(&T) -> bool + Clone> Clone for Validate<T, P, F> {
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            validator: self.validator.clone(),
            err_msg: self.err_msg,
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>, F: Fn(&T) -> bool> Parser<T> for Validate<T, P, F> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<T> {
        let start = input.at();
        let parsed = self.parser.parse(input)?;

        if (self.validator)(&parsed.data) {
            Ok(parsed)
        } else {
            // TODO: ranged error (from input start to parsed end)
            Err(start.range(0).custom_err(match &self.err_msg {
                Some(msg) => msg,
                None => "validator failed",
            }))
        }
    }
}
