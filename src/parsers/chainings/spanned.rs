use std::marker::PhantomData;

use crate::{Eaten, PResult, Parser, ParserInput};

#[derive(Clone)]
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
