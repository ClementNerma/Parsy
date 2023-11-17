use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

pub struct EnsureType<T, P: Parser<T>> {
    inner: P,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>> EnsureType<T, P> {
    pub fn new(inner: P) -> Self {
        Self {
            inner,
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<T> for EnsureType<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<T> {
        self.inner.parse(input)
    }
}
