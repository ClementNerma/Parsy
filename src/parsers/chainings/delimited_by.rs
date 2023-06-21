use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

pub struct DelimitedBy<L, LP: Parser<L>, M, MP: Parser<M>, R, RP: Parser<R>> {
    left: LP,
    middle: MP,
    right: RP,
    _l: PhantomData<L>,
    _m: PhantomData<M>,
    _r: PhantomData<R>,
}

impl<L, LP: Parser<L>, M, MP: Parser<M>, R, RP: Parser<R>> DelimitedBy<L, LP, M, MP, R, RP> {
    pub fn new(left: LP, middle: MP, right: RP) -> Self {
        Self {
            left,
            middle,
            right,
            _l: PhantomData,
            _m: PhantomData,
            _r: PhantomData,
        }
    }
}

// NOTE: This is required because of https://github.com/rust-lang/rust/issues/26925
impl<L, LP: Parser<L> + Clone, M, MP: Parser<M> + Clone, R, RP: Parser<R> + Clone> Clone
    for DelimitedBy<L, LP, M, MP, R, RP>
{
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            middle: self.middle.clone(),
            right: self.right.clone(),
            _l: self._l,
            _m: self._m,
            _r: self._r,
        }
    }
}

impl<L, LP: Parser<L>, M, MP: Parser<M>, R, RP: Parser<R>> Parser<M>
    for DelimitedBy<L, LP, M, MP, R, RP>
{
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<M> {
        let start = self.left.parse(input).unwrap();
        let middle = self.middle.parse(input)?;
        let end = self.right.parse(input).unwrap();

        Ok(start
            .combine(middle)
            .combine(end)
            .map(|((_, data), _)| data))
    }
}
