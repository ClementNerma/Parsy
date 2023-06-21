use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

#[derive(Clone)]
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

impl<T, P: Parser<T>, U: Clone> Parser<U> for To<T, P, U> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<U> {
        self.parser
            .parse(input)
            .map(|eaten| eaten.replace(self.data.clone()))
    }
}
