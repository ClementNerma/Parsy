use std::marker::PhantomData;

use crate::{Eaten, PResult, Parser, ParserInput};

#[derive(Clone)]
pub struct OrNot<T, P: Parser<T>> {
    parser: P,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>> OrNot<T, P> {
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<Option<T>> for OrNot<T, P> {
    fn parse_inner<'a>(&self, input: &mut ParserInput<'a>) -> PResult<Option<T>> {
        let start = input.at();

        match self.parser.parse(input) {
            Ok(eaten) => Ok(eaten.map(Some)),
            Err(err) if err.is_critical() => Err(err),
            Err(_) => Ok(Eaten::ate(start.range(0), None)),
        }
    }
}
