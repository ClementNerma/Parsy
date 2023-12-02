use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{PResult, Parser, ParserInput};

#[perfect_derive(Debug, Clone, Copy)]
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
