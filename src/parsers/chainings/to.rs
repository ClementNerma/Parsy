use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

pub struct To<T, P: Parser<T>, U: Clone> {
    parser: P,
    data: U,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>, U: Clone> To<T, P, U> {
    pub fn new(parser: P, data: U) -> Self {
        Self {
            parser,
            data,
            _t: PhantomData,
        }
    }
}

// NOTE: This is required because of https://github.com/rust-lang/rust/issues/26925
impl<T, P: Parser<T> + Clone, U: Clone> Clone for To<T, P, U> {
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            data: self.data.clone(),
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>, U: Clone> Parser<U> for To<T, P, U> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<U> {
        self.parser
            .parse(input)
            .map(|eaten| eaten.replace(self.data.clone()))
    }
}
