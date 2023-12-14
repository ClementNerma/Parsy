use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Eaten, PResult, Parser, ParserInput};

#[perfect_derive(Clone, Copy)]
pub struct Not<T, P: Parser<T>> {
    parser: P,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>> Not<T, P> {
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<()> for Not<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<()> {
        match self.parser.parse(input) {
            Ok(eaten) => Err(eaten.at.start.custom_err("Parser should not have matched")),
            Err(_) => Ok(Eaten::ate(input.range(0), ())),
        }
    }
}
