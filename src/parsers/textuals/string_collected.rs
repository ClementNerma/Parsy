use std::marker::PhantomData;

use crate::{Eaten, PResult, Parser, ParserInput};

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

// NOTE: This is required because of https://github.com/rust-lang/rust/issues/26925
impl<T, P: Parser<T> + Clone> Clone for StringCollected<T, P> {
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<String> for StringCollected<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<String> {
        let parsed = self.parser.parse(input)?;

        Ok(Eaten::ate(parsed.at, input.extract(parsed.at).to_string()))
    }
}
