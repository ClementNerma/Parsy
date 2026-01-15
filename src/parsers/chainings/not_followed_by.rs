use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserNonConstUtils, ParserResult, ParsingError};

/// See [`not_followed_by`](`crate::ParserConstUtils::not_followed_by`)
#[perfect_derive(Debug, Clone, Copy)]
pub struct NotFollowedBy<F, FP: Parser<F>, O, OP: Parser<O>> {
    parser: FP,
    following: OP,
    _p: PhantomData<(F, O)>,
}

impl<F, FP: Parser<F>, O, OP: Parser<O>> NotFollowedBy<F, FP, O, OP> {
    pub const fn new(parser: FP, following: OP) -> Self {
        Self {
            parser,
            following,
            _p: PhantomData,
        }
    }
}

impl<F, FP: Parser<F>, O, OP: Parser<O>> Parser<F> for NotFollowedBy<F, FP, O, OP> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<F> {
        let parsed = self.parser.parse(input)?;

        match self.following.parse(input) {
            Ok(span) => Err(ParsingError::custom(
                span.at,
                "Parser should not have matched",
            )),

            Err(err) if err.is_critical() => Err(err),

            Err(_) => Ok(parsed),
        }
    }
}
