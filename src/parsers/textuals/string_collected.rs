use std::marker::PhantomData;

use crate::{Eaten, PResult, Parser, ParserInput};

#[derive(Clone)]
pub struct StringCollected<T, P: Parser<T>> {
    parser: P,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>> StringCollected<T, P> {
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<String> for StringCollected<T, P> {
    fn parse_inner<'a>(&self, input: &mut ParserInput<'a>) -> PResult<String> {
        let parsed = self.parser.parse(input)?;

        Ok(Eaten::ate(parsed.at, input.extract(parsed.at).to_string()))
    }
}
