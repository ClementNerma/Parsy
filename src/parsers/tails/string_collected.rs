use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserNonConstUtils, ParserResult, Span};

/// See [`crate::ParserConstUtils::collect_string`]
#[perfect_derive(Clone, Copy)]
pub struct StringCollected<T, P: Parser<T>> {
    parser: P,
    _p: PhantomData<T>,
}

impl<T, P: Parser<T>> StringCollected<T, P> {
    pub const fn new(parser: P) -> Self {
        Self {
            parser,
            _p: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<String> for StringCollected<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<String> {
        let parsed = self.parser.parse(input)?;

        Ok(Span::ate(parsed.at, input.extract(parsed.at).to_string()))
    }
}
