use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{PResult, Parser, ParserInput};

#[perfect_derive(Debug, Clone, Copy)]
pub struct ValidateWithMessage<T, P: Parser<T>, F: Fn(&T) -> Option<&'static str>> {
    parser: P,
    validator: F,
    err_msg: Option<&'static str>,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>, F: Fn(&T) -> Option<&'static str>> ValidateWithMessage<T, P, F> {
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

impl<T, P: Parser<T>, F: Fn(&T) -> Option<&'static str>> Parser<T>
    for ValidateWithMessage<T, P, F>
{
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<T> {
        let start = input.at();
        let parsed = self.parser.parse(input)?;

        match (self.validator)(&parsed.data) {
            Some(err) => Err(start.custom_err(err, parsed.at.len)),
            None => Ok(parsed),
        }
    }
}
