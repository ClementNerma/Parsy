use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{PResult, Parser, ParserInput};

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
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<O> {
        Ok(self.parser.parse(input)?.map(&self.mapper))
    }
}
