use std::{borrow::Cow, marker::PhantomData};

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult, ParsingError, Span};

#[perfect_derive(Debug, Clone, Copy)]
pub struct AndThenOrCritical<T, P: Parser<T>, U, F: Fn(T) -> Result<U, Cow<'static, str>>> {
    parser: P,
    mapper: F,
    _p: PhantomData<(T, U)>,
}

impl<T, P: Parser<T>, U, F: Fn(T) -> Result<U, Cow<'static, str>>> AndThenOrCritical<T, P, U, F> {
    pub fn new(parser: P, mapper: F) -> Self {
        Self {
            parser,
            mapper,
            _p: PhantomData,
        }
    }
}

impl<T, P: Parser<T>, U, F: Fn(T) -> Result<U, Cow<'static, str>>> Parser<U>
    for AndThenOrCritical<T, P, U, F>
{
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<U> {
        let Span { data, at } = self.parser.parse(input)?;

        (self.mapper)(data)
            .map(|data| Span::ate(at, data))
            .map_err(|err| ParsingError::custom(at, "an error was returned").criticalize(err))
    }
}
