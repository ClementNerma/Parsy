use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult};

#[perfect_derive(Debug, Clone, Copy)]
pub struct IgnoreThen<F, FP: Parser<F>, O, OP: Parser<O>> {
    from: FP,
    to: OP,
    _p: PhantomData<(F, O)>,
}

impl<F, FP: Parser<F>, O, OP: Parser<O>> IgnoreThen<F, FP, O, OP> {
    pub const fn new(from: FP, to: OP) -> Self {
        Self {
            from,
            to,
            _p: PhantomData,
        }
    }
}

impl<F, FP: Parser<F>, O, OP: Parser<O>> Parser<O> for IgnoreThen<F, FP, O, OP> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<O> {
        let from = self.from.parse(input)?;
        let to = self.to.parse(input)?;
        Ok(from.combine(to).map(|(_, to)| to))
    }
}
