use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

pub struct Critical<T, P: Parser<T>> {
    parser: P,
    message: &'static str,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>> Critical<T, P> {
    pub fn new(parser: P, message: &'static str) -> Self {
        Self {
            parser,
            message,
            _t: PhantomData,
        }
    }
}

// NOTE: This is required because of https://github.com/rust-lang/rust/issues/26925
impl<T, P: Parser<T> + Clone> Clone for Critical<T, P> {
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            message: self.message.clone(),
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<T> for Critical<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<T> {
        self.parser
            .parse(input)
            .map_err(|err| err.criticalize(self.message))
    }
}
