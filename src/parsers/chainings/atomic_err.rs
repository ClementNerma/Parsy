use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

pub struct AtomicErr<T, P: Parser<T>> {
    parser: P,
    message: &'static str,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>> AtomicErr<T, P> {
    pub fn new(parser: P, message: &'static str) -> Self {
        Self {
            parser,
            message,
            _t: PhantomData,
        }
    }
}

// NOTE: This is required because of https://github.com/rust-lang/rust/issues/26925
impl<T, P: Parser<T> + Clone> Clone for AtomicErr<T, P> {
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            message: self.message,
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<T> for AtomicErr<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<T> {
        self.parser.parse(input).map_err(|err| {
            err.inner()
                .at()
                .custom_err(self.message)
                .with_atomic_error(self.message)
        })
    }
}
