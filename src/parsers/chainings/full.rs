use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

pub struct Full<T, P: Parser<T>> {
    parser: P,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>> Full<T, P> {
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            _t: PhantomData,
        }
    }
}

// NOTE: This is required because of https://github.com/rust-lang/rust/issues/26925
impl<T, P: Parser<T> + Clone> Clone for Full<T, P> {
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<T> for Full<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<T> {
        if input.offset() > 0 {
            return Err(input.range(0).custom_err("Expected start of input"));
        }

        let data = self.parser.parse(input)?;

        if data.at.len != input.original().len() {
            return Err(data
                .at
                .start
                .add(data.at.len)
                .range(0)
                .custom_err("Unexpected end of input"));
        }

        Ok(data)
    }
}
