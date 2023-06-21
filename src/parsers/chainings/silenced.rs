use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

#[derive(Clone)]
pub struct Silenced<T, P: Parser<T>> {
    parser: P,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>> Silenced<T, P> {
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<()> for Silenced<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<()> {
        self.parser.parse(input).map(|input| input.replace(()))
    }
}
