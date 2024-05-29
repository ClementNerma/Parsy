use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{PResult, Parser, ParserInput, ParsingError};

#[perfect_derive(Debug, Clone, Copy)]
pub struct AtomicErr<T, P: Parser<T>> {
    parser: P,
    message: &'static str,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>> AtomicErr<T, P> {
    pub fn new(parser: P, message: &'static str) -> Self {
        Self {
            parser,
            message,
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<T> for AtomicErr<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<T> {
        self.parser.parse(input).map_err(|err| {
            ParsingError::custom(err.inner().at(), self.message).with_atomic_error(self.message)
        })
    }
}
