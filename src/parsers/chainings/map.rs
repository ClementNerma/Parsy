use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

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

// NOTE: This is required because of https://github.com/rust-lang/rust/issues/26925
impl<F, FP: Parser<F> + Clone, O, OF: Fn(F) -> O + Clone> Clone for Map<F, FP, O, OF> {
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            mapper: self.mapper.clone(),
            _f: PhantomData,
            _o: PhantomData,
        }
    }
}

impl<F, FP: Parser<F>, O, OF: Fn(F) -> O + Clone> Parser<O> for Map<F, FP, O, OF> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<O> {
        Ok(self.parser.parse(input)?.map(&self.mapper))
    }
}
