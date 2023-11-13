use std::marker::PhantomData;

use crate::{Eaten, PResult, Parser, ParserInput};

#[derive(Clone)]
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

        Ok(Eaten::ate(
            parsed.at,
            (input.extract(parsed.at).to_string(), parsed.data),
        ))
    }
}
