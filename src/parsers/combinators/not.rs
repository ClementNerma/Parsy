use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult, ParsingError, Span};

#[perfect_derive(Clone, Copy)]
pub struct Not<T, P: Parser<T>> {
    parser: P,
    _p: PhantomData<T>,
}

impl<T, P: Parser<T>> Not<T, P> {
    pub const fn new(parser: P) -> Self {
        Self {
            parser,
            _p: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<()> for Not<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<()> {
        match self.parser.parse(input) {
            Ok(span) => Err(ParsingError::custom(
                span.at,
                "Parser should not have matched",
            )),

            Err(_) => Ok(Span::ate(input.range(0), ())),
        }
    }
}
