use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Span, PResult, Parser, ParserInput};

#[perfect_derive(Debug, Clone, Copy)]
pub struct Spanned<T, P: Parser<T>> {
    parser: P,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>> Spanned<T, P> {
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<Span<T>> for Spanned<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<Span<T>> {
        let parsed = self.parser.parse(input)?;
        Ok(Span::ate(parsed.at, parsed))
    }
}
