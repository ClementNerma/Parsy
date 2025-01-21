use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{ParserResult, Parser, ParserInput, Span};

#[perfect_derive(Debug, Clone, Copy)]
pub struct OrNot<T, P: Parser<T>> {
    parser: P,
    _p: PhantomData<T>,
}

impl<T, P: Parser<T>> OrNot<T, P> {
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            _p: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<Option<T>> for OrNot<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<Option<T>> {
        let start = input.at();

        match self.parser.parse(input) {
            Ok(span) => Ok(span.map(Some)),
            Err(err) if err.is_critical() => Err(err),
            Err(_) => Ok(Span::ate(start.range(0), None)),
        }
    }
}
