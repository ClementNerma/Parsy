use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserNonConstUtils, ParserResult};

/// See [`padded_by`](`crate::ParserConstUtils::padded_by`)
#[perfect_derive(Debug, Clone, Copy)]
pub struct PaddedBy<T, TP: Parser<T>, P, PP: Parser<P>> {
    middle: TP,
    padding: PP,
    _p: PhantomData<(T, P)>,
}

impl<T, TP: Parser<T>, P, PP: Parser<P>> PaddedBy<T, TP, P, PP> {
    pub const fn new(middle: TP, padding: PP) -> Self {
        Self {
            middle,
            padding,
            _p: PhantomData,
        }
    }
}

impl<T, TP: Parser<T>, P, PP: Parser<P>> Parser<T> for PaddedBy<T, TP, P, PP> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<T> {
        let start = self.padding.parse(input)?;
        let middle = self.middle.parse(input)?;
        let end = self.padding.parse(input)?;

        Ok(start
            .combine(middle)
            .combine(end)
            .map(|((_, data), _)| data))
    }
}
