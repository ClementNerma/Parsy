use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult};

#[perfect_derive(Debug, Clone, Copy)]
pub struct To<T, P: Parser<T>, U: Copy> {
    parser: P,
    data: U,
    _p: PhantomData<T>,
}

impl<T, P: Parser<T>, U: Copy> To<T, P, U> {
    pub fn new(parser: P, data: U) -> Self {
        Self {
            parser,
            data,
            _p: PhantomData,
        }
    }
}

impl<T, P: Parser<T>, U: Copy> Parser<U> for To<T, P, U> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<U> {
        self.parser.parse(input).map(|span| span.replace(self.data))
    }
}
