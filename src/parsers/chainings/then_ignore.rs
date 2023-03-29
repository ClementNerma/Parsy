use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

#[derive(Clone)]
pub struct ThenIgnore<F, FP: Parser<F>, O, OP: Parser<O>> {
    from: FP,
    to: OP,
    _f: PhantomData<F>,
    _o: PhantomData<O>,
}

impl<F, FP: Parser<F>, O, OP: Parser<O>> ThenIgnore<F, FP, O, OP> {
    pub fn new(from: FP, to: OP) -> Self {
        Self {
            from,
            to,
            _f: PhantomData,
            _o: PhantomData,
        }
    }
}

impl<F, FP: Parser<F>, O, OP: Parser<O>> Parser<F> for ThenIgnore<F, FP, O, OP> {
    fn parse_inner<'a>(&self, input: &mut ParserInput<'a>) -> PResult<F> {
        let from = self.from.parse(input)?;
        let to = self.to.parse(input)?;
        Ok(from.combine(to).map(|(from, _)| from))
    }
}
