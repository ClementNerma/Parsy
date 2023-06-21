use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

#[derive(Clone)]
pub struct Then<F, FP: Parser<F>, O, OP: Parser<O>> {
    from: FP,
    to: OP,
    _f: PhantomData<F>,
    _o: PhantomData<O>,
}

impl<F, FP: Parser<F>, O, OP: Parser<O>> Then<F, FP, O, OP> {
    pub fn new(from: FP, to: OP) -> Self {
        Self {
            from,
            to,
            _f: PhantomData,
            _o: PhantomData,
        }
    }
}

impl<F, FP: Parser<F>, O, OP: Parser<O>> Parser<(F, O)> for Then<F, FP, O, OP> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<(F, O)> {
        let from = self.from.parse(input)?;
        let to = self.to.parse(input)?;
        Ok(from.combine(to))
    }
}
