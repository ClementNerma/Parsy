use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Eaten, PResult, Parser, ParserInput};

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

impl<T, P: Parser<T>> Parser<Eaten<T>> for Spanned<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<Eaten<T>> {
        let parsed = self.parser.parse(input)?;
        Ok(Eaten::ate(parsed.at, parsed))
    }
}
