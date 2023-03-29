use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

#[derive(Clone)]
pub struct Map<F, FP: Parser<F>, O, OF: Fn(F) -> O + Clone> {
    parser: FP,
    mapper: OF,
    _f: PhantomData<F>,
    _o: PhantomData<O>,
}

impl<F, FP: Parser<F>, O, OF: Fn(F) -> O + Clone> Map<F, FP, O, OF> {
    pub fn new(from: FP, mapper: OF) -> Self {
        Self {
            parser: from,
            mapper,
            _f: PhantomData,
            _o: PhantomData,
        }
    }
}

impl<F, FP: Parser<F>, O, OF: Fn(F) -> O + Clone> Parser<O> for Map<F, FP, O, OF> {
    fn parse_inner<'a>(&self, input: &mut ParserInput<'a>) -> PResult<O> {
        Ok(self.parser.parse(input)?.map(&self.mapper))
    }
}
