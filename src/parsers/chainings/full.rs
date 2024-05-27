use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{PResult, Parser, ParserInput};

#[perfect_derive(Debug, Clone, Copy)]
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

impl<T, P: Parser<T>> Parser<T> for Full<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<T> {
        if input.offset() > 0 {
            return Err(input.at().custom_err("Expected start of input", 0));
        }

        let data = self.parser.parse(input)?;

        if data.at.len != input.original().len() {
            return Err(data
                .at
                .start
                .add(data.at.len)
                .custom_err("Unexpected symbol", 0));
        }

        Ok(data)
    }
}
