use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult};

#[perfect_derive(Debug, Clone, Copy)]
pub struct Map<F, FP: Parser<F>, O, OF: Fn(F) -> O + Clone> {
    parser: FP,
    mapper: OF,
    _p: PhantomData<(F, O)>,
}

impl<F, FP: Parser<F>, O, OF: Fn(F) -> O + Clone> Map<F, FP, O, OF> {
    pub fn new(from: FP, mapper: OF) -> Self {
        Self {
            parser: from,
            mapper,
            _p: PhantomData,
        }
    }
}

impl<F, FP: Parser<F>, O, OF: Fn(F) -> O + Clone> Parser<O> for Map<F, FP, O, OF> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<O> {
        Ok(self.parser.parse(input)?.map(&self.mapper))
    }
}
