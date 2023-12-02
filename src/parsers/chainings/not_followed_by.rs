use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{PResult, Parser, ParserInput};

#[perfect_derive(Debug, Clone, Copy)]
pub struct NotFollowedBy<F, FP: Parser<F>, O, OP: Parser<O>> {
    parser: FP,
    following: OP,
    _f: PhantomData<F>,
    _o: PhantomData<O>,
}

impl<F, FP: Parser<F>, O, OP: Parser<O>> NotFollowedBy<F, FP, O, OP> {
    pub fn new(parser: FP, following: OP) -> Self {
        Self {
            parser,
            following,
            _f: PhantomData,
            _o: PhantomData,
        }
    }
}

impl<F, FP: Parser<F>, O, OP: Parser<O>> Parser<F> for NotFollowedBy<F, FP, O, OP> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<F> {
        let parsed = self.parser.parse(input)?;

        match self.following.parse(input) {
            Ok(eaten) => Err(eaten.at.custom_err("Parser should not have matched")),
            Err(err) if err.is_critical() => Err(err),
            Err(_) => Ok(parsed),
        }
    }
}
