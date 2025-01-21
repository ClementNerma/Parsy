use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult, ParsingError, Span};

#[perfect_derive(Clone, Copy)]
pub struct AndThen<T, P: Parser<T>, U, F: Fn(T) -> Result<U, ParsingError>> {
    parser: P,
    mapper: F,
    _p: PhantomData<(T, U)>,
}

impl<T, P: Parser<T>, U, F: Fn(T) -> Result<U, ParsingError>> AndThen<T, P, U, F> {
    pub fn new(parser: P, mapper: F) -> Self {
        Self {
            parser,
            mapper,
            _p: PhantomData,
        }
    }
}

impl<T, P: Parser<T>, U, F: Fn(T) -> Result<U, ParsingError>> Parser<U> for AndThen<T, P, U, F> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<U> {
        let Span { data, at } = self.parser.parse(input)?;

        (self.mapper)(data).map(|data| Span::ate(at, data))
    }
}
