use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{PResult, Parser, ParserInput};

#[perfect_derive(Debug, Clone, Copy)]
pub struct Padded<T, TP: Parser<T>, P, PP: Parser<P>> {
    middle: TP,
    padding: PP,
    _m: PhantomData<T>,
    _p: PhantomData<P>,
}

impl<T, TP: Parser<T>, P, PP: Parser<P>> Padded<T, TP, P, PP> {
    pub fn new(middle: TP, padding: PP) -> Self {
        Self {
            middle,
            padding,
            _m: PhantomData,
            _p: PhantomData,
        }
    }
}

impl<T, TP: Parser<T>, P, PP: Parser<P>> Parser<T> for Padded<T, TP, P, PP> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<T> {
        let start = self.padding.parse(input)?;
        let middle = self.middle.parse(input)?;
        let end = self.padding.parse(input)?;

        Ok(start
            .combine(middle)
            .combine(end)
            .map(|((_, data), _)| data))
    }
}
