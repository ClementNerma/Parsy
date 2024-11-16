use std::{borrow::Cow, marker::PhantomData};

use perfect_derive::perfect_derive;

use crate::{Span, PResult, Parser, ParserInput, ParsingError};

#[perfect_derive(Debug, Clone, Copy)]
pub struct AndThenOrStrErr<T, P: Parser<T>, U, F: Fn(T) -> Result<U, String>> {
    parser: P,
    mapper: F,
    _t: PhantomData<T>,
    _u: PhantomData<U>,
}

impl<T, P: Parser<T>, U, F: Fn(T) -> Result<U, String>> AndThenOrStrErr<T, P, U, F> {
    pub fn new(parser: P, mapper: F) -> Self {
        Self {
            parser,
            mapper,
            _t: PhantomData,
            _u: PhantomData,
        }
    }
}

impl<T, P: Parser<T>, U, F: Fn(T) -> Result<U, String>> Parser<U> for AndThenOrStrErr<T, P, U, F> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<U> {
        let Span { data, at } = self.parser.parse(input)?;

        (self.mapper)(data)
            .map(|data| Span::ate(at, data))
            .map_err(|err| {
                ParsingError::custom(at, "mapper returned an Err variant")
                    .criticalize(Cow::Owned(err))
            })
    }
}
