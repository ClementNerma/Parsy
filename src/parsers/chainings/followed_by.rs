use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

#[derive(Clone)]
pub struct FollowedBy<F, FP: Parser<F>, O, OP: Parser<O>> {
    parser: FP,
    following: OP,
    _f: PhantomData<F>,
    _o: PhantomData<O>,
}

impl<F, FP: Parser<F>, O, OP: Parser<O>> FollowedBy<F, FP, O, OP> {
    pub fn new(parser: FP, following: OP) -> Self {
        Self {
            parser,
            following,
            _f: PhantomData,
            _o: PhantomData,
        }
    }
}

impl<F, FP: Parser<F>, O, OP: Parser<O>> Parser<F> for FollowedBy<F, FP, O, OP> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<F> {
        let parsed = self.parser.parse(input)?;
        self.following.parse(input)?;
        Ok(parsed)
    }
}
