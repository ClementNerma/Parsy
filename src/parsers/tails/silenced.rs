use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult};

#[perfect_derive(Debug, Clone, Copy)]
pub struct Silenced<T, P: Parser<T>> {
    parser: P,
    _p: PhantomData<T>,
}

impl<T, P: Parser<T>> Silenced<T, P> {
    pub const fn new(parser: P) -> Self {
        Self {
            parser,
            _p: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<()> for Silenced<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<()> {
        self.parser.parse(input).map(|input| input.replace(()))
    }
}
