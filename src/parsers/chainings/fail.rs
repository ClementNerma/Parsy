use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

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

// NOTE: This is required because of https://github.com/rust-lang/rust/issues/26925
impl<T, P: Parser<T> + Clone> Clone for Fail<T, P> {
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
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
            Ok(eaten) => Err(eaten.at.custom_err("Parser should not have matched")),
            Err(err) => Err(if err.is_critical() {
                err
            } else {
                start.range(0).just_break()
            }),
        }
    }
}
