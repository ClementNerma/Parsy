use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult, Span};

#[perfect_derive(Clone, Copy)]
pub struct StaticRef<T, P: Parser<T> + 'static> {
    parser: &'static P,
    _p: PhantomData<T>,
}

impl<T, P: Parser<T>> StaticRef<T, P> {
    pub const fn new(parser: &'static P) -> Self {
        Self {
            parser,
            _p: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<T> for StaticRef<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<T> {
        let mut input_copy = *input;
        let parsed = self.parser.parse(&mut input_copy)?;
        Ok(Span::ate(input.range(0), parsed.data))
    }
}
