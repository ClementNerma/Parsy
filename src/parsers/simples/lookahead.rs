use std::marker::PhantomData;

use crate::{Eaten, PResult, Parser, ParserInput};

pub struct Lookahead<T, P: Parser<T>> {
    parser: P,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>> Lookahead<T, P> {
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            _t: PhantomData,
        }
    }
}

// NOTE: This is required because of https://github.com/rust-lang/rust/issues/26925
impl<T, P: Parser<T> + Clone> Clone for Lookahead<T, P> {
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<T> for Lookahead<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<T> {
        let parsed = self.parser.parse(&mut input.clone())?;
        Ok(Eaten::ate(input.range(0), parsed.data))
    }
}
