use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{PResult, Parser, ParserInput, Span};

#[perfect_derive(Clone, Copy)]
pub struct MapStr<F, FP: Parser<F>, O, OF: Fn(&str) -> O + Clone> {
    parser: FP,
    mapper: OF,
    _p: PhantomData<(F, O)>,
}

impl<F, FP: Parser<F>, O, OF: Fn(&str) -> O + Clone> MapStr<F, FP, O, OF> {
    pub fn new(parser: FP, mapper: OF) -> Self {
        Self {
            parser,
            mapper,
            _p: PhantomData,
        }
    }
}

impl<F, FP: Parser<F>, O, OF: Fn(&str) -> O + Clone> Parser<O> for MapStr<F, FP, O, OF> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<O> {
        let parsed = self.parser.parse(input)?;
        let extract = input.extract(parsed.at);

        Ok(Span::ate(parsed.at, (self.mapper)(extract)))
    }
}
