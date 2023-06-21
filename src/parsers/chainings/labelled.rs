use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

#[derive(Clone)]
pub struct Labelled<T, P: Parser<T>> {
    parser: P,
    label: &'static str,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>> Labelled<T, P> {
    pub fn new(parser: P, label: &'static str) -> Self {
        Self {
            parser,
            label,
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<T> for Labelled<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<T> {
        self.parser
            .parse(input)
            .map_err(|err| err.labellize(self.label))
    }
}
