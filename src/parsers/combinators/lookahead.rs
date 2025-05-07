use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult, Span};

#[perfect_derive(Clone, Copy)]
pub struct Lookahead<T, P: Parser<T>> {
    parser: P,
    _p: PhantomData<T>,
}

impl<T, P: Parser<T>> Lookahead<T, P> {
    pub const fn new(parser: P) -> Self {
        Self {
            parser,
            _p: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<T> for Lookahead<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<T> {
        let mut input_copy = *input;
        let parsed = self.parser.parse(&mut input_copy)?;
        Ok(Span::ate(input.range(0), parsed.data))
    }
}
