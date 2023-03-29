use std::marker::PhantomData;

use crate::{Eaten, PResult, Parser, ParserInput};

#[derive(Clone)]
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

impl<T, P: Parser<T>> Parser<T> for Lookahead<T, P> {
    fn parse_inner<'a>(&self, input: &mut ParserInput<'a>) -> PResult<T> {
        let parsed = self.parser.parse(&mut input.clone())?;
        Ok(Eaten::ate(input.range(0), parsed.data))
    }
}
