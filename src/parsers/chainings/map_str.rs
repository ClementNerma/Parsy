use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserNonConstUtils, ParserResult, Span};

/// See [`crate::ParserConstUtils::map_str`]
#[perfect_derive(Clone, Copy)]
pub struct MapStr<F, FP: Parser<F>, O, OF: Fn(&str) -> O> {
    parser: FP,
    mapper: OF,
    _p: PhantomData<(F, O)>,
}

impl<F, FP: Parser<F>, O, OF: Fn(&str) -> O> MapStr<F, FP, O, OF> {
    pub const fn new(parser: FP, mapper: OF) -> Self {
        Self {
            parser,
            mapper,
            _p: PhantomData,
        }
    }
}

impl<F, FP: Parser<F>, O, OF: Fn(&str) -> O> Parser<O> for MapStr<F, FP, O, OF> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<O> {
        let parsed = self.parser.parse(input)?;
        let extract = input.extract(parsed.at);

        Ok(Span::ate(parsed.at, (self.mapper)(extract)))
    }
}
