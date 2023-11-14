use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

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

// NOTE: This is required because of https://github.com/rust-lang/rust/issues/26925
impl<F, FP: Parser<F> + Clone, O, OP: Parser<O> + Clone> Clone for ThenIgnore<F, FP, O, OP> {
    fn clone(&self) -> Self {
        Self {
            from: self.from.clone(),
            to: self.to.clone(),
            _f: PhantomData,
            _o: PhantomData,
        }
    }
}

impl<F, FP: Parser<F>, O, OP: Parser<O>> Parser<F> for ThenIgnore<F, FP, O, OP> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<F> {
        let from = self.from.parse(input)?;
        let to = self.to.parse(input)?;
        Ok(from.combine(to).map(|(from, _)| from))
    }
}
