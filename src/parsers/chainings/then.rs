use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserNonConstUtils, ParserResult};

/// See [`crate::ParserConstUtils::then`]
#[perfect_derive(Debug, Clone, Copy)]
pub struct Then<F, FP: Parser<F>, O, OP: Parser<O>> {
    from: FP,
    to: OP,
    _p: PhantomData<(F, O)>,
}

impl<F, FP: Parser<F>, O, OP: Parser<O>> Then<F, FP, O, OP> {
    pub const fn new(from: FP, to: OP) -> Self {
        Self {
            from,
            to,
            _p: PhantomData,
        }
    }
}

impl<F, FP: Parser<F>, O, OP: Parser<O>> Parser<(F, O)> for Then<F, FP, O, OP> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<(F, O)> {
        let from = self.from.parse(input)?;
        let to = self.to.parse(input)?;
        Ok(from.combine(to))
    }
}
