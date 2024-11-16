use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Span, PResult, Parser, ParserInput};

#[perfect_derive(Clone, Copy)]
pub struct StringCollectedWithData<T, P: Parser<T>> {
    parser: P,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>> StringCollectedWithData<T, P> {
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<(String, T)> for StringCollectedWithData<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<(String, T)> {
        let parsed = self.parser.parse(input)?;

        Ok(Span::ate(
            parsed.at,
            (input.extract(parsed.at).to_string(), parsed.data),
        ))
    }
}
