use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

pub struct Silenced<T, P: Parser<T>> {
    parser: P,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>> Silenced<T, P> {
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            _t: PhantomData,
        }
    }
}

// NOTE: This is required because of https://github.com/rust-lang/rust/issues/26925
impl<T, P: Parser<T> + Clone> Clone for Silenced<T, P> {
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<()> for Silenced<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<()> {
        self.parser.parse(input).map(|input| input.replace(()))
    }
}
