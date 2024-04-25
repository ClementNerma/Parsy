use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{PResult, Parser, ParserInput};

#[perfect_derive(Debug, Clone, Copy)]
pub struct Fail<T, P: Parser<T>> {
    parser: P,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>> Fail<T, P> {
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            _t: PhantomData,
        }
    }
}

// TODO: Once issue <https://github.com/rust-lang/rust/issues/35121> is solved,
//       change the '()' type to '!'
impl<T, P: Parser<T>> Parser<()> for Fail<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<()> {
        let start = input.at();

        match self.parser.parse(input) {
            Ok(eaten) => Err(eaten
                .at
                .start
                .custom_err("Parser should not have matched", eaten.at.len)),
            Err(err) => Err(if err.is_critical() {
                err
            } else {
                start.just_break()
            }),
        }
    }
}
