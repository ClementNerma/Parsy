use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserNonConstUtils, ParserResult};

/// See [`followed_by`](`crate::ParserConstUtils::followed_by`)
#[perfect_derive(Debug, Clone, Copy)]
pub struct FollowedBy<F, FP: Parser<F>, O, OP: Parser<O>> {
    parser: FP,
    following: OP,
    _p: PhantomData<(F, O)>,
}

impl<F, FP: Parser<F>, O, OP: Parser<O>> FollowedBy<F, FP, O, OP> {
    pub const fn new(parser: FP, following: OP) -> Self {
        Self {
            parser,
            following,
            _p: PhantomData,
        }
    }
}

impl<F, FP: Parser<F>, O, OP: Parser<O>> Parser<F> for FollowedBy<F, FP, O, OP> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<F> {
        let parsed = self.parser.parse(input)?;
        self.following.parse(input)?;
        Ok(parsed)
    }
}
