use std::{borrow::Cow, marker::PhantomData};

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult, ParsingError, Span};

#[perfect_derive(Clone, Copy)]
pub struct AndThen<T, P: Parser<T>, U, F: Fn(T) -> Result<U, ParsingError>> {
    parser: P,
    mapper: F,
    custom_err_msg: Option<&'static str>,
    _p: PhantomData<(T, U)>,
}

impl<T, P: Parser<T>, U, F: Fn(T) -> Result<U, ParsingError>> AndThen<T, P, U, F> {
    pub const fn new(parser: P, mapper: F) -> Self {
        Self {
            parser,
            mapper,
            custom_err_msg: None,
            _p: PhantomData,
        }
    }

    pub const fn with_custom_err(mut self, err: &'static str) -> Self {
        self.custom_err_msg = Some(err);
        self
    }
}

impl<T, P: Parser<T>, U, F: Fn(T) -> Result<U, ParsingError>> Parser<U> for AndThen<T, P, U, F> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<U> {
        let Span { data, at } = self.parser.parse(input)?;

        (self.mapper)(data)
            .map(|data| Span::ate(at, data))
            .map_err(|err| match self.custom_err_msg {
                None => err,
                Some(msg) => ParsingError::custom(at, "an error was returned")
                    .criticalize(Cow::Borrowed(msg)),
            })
    }
}
