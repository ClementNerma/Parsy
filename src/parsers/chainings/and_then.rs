use std::marker::PhantomData;

use crate::{Eaten, PResult, Parser, ParserInput, ParsingError};

pub struct AndThen<T, P: Parser<T>, U, F: Fn(T) -> Result<U, ParsingError>> {
    parser: P,
    mapper: F,
    _t: PhantomData<T>,
    _u: PhantomData<U>,
}

impl<T, P: Parser<T>, U, F: Fn(T) -> Result<U, ParsingError>> AndThen<T, P, U, F> {
    pub fn new(parser: P, mapper: F) -> Self {
        Self {
            parser,
            mapper,
            _t: PhantomData,
            _u: PhantomData,
        }
    }
}

// NOTE: This is required because of https://github.com/rust-lang/rust/issues/26925
impl<T, P: Parser<T> + Clone, U, F: Fn(T) -> Result<U, ParsingError> + Clone> Clone
    for AndThen<T, P, U, F>
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            mapper: self.mapper.clone(),
            _t: PhantomData,
            _u: PhantomData,
        }
    }
}

impl<T, P: Parser<T>, U, F: Fn(T) -> Result<U, ParsingError>> Parser<U> for AndThen<T, P, U, F> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<U> {
        let Eaten { data, at } = self.parser.parse(input)?;

        (self.mapper)(data).map(|data| Eaten::ate(at, data))
    }
}
