use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{ParserResult, Parser, ParserInput, Span};

#[perfect_derive(Debug, Clone, Copy)]
pub struct Spanned<T, P: Parser<T>> {
    parser: P,
    _p: PhantomData<T>,
}

impl<T, P: Parser<T>> Spanned<T, P> {
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            _p: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<Span<T>> for Spanned<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<Span<T>> {
        let parsed = self.parser.parse(input)?;
        Ok(Span::ate(parsed.at, parsed))
    }
}
