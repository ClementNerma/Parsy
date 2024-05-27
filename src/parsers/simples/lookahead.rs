use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Eaten, PResult, Parser, ParserInput};

#[perfect_derive(Clone, Copy)]
pub struct Lookahead<T, P: Parser<T>> {
    parser: P,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>> Lookahead<T, P> {
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<T> for Lookahead<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<T> {
        let mut input_copy = *input;
        let parsed = self.parser.parse(&mut input_copy)?;
        Ok(Eaten::ate(input.range(0), parsed.data))
    }
}
