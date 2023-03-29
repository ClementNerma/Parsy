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

impl<T, P: Parser<T>> Parser<T> for Full<T, P> {
    fn parse_inner<'a>(&self, input: &mut ParserInput<'a>) -> PResult<T> {
        let data = self.parser.parse(input)?;

        if data.at.len != input.original().len() {
            return Err(data.at.custom_err(format!(
                "Input was not consumed entirely (consumed {} char(s) on {})",
                data.at.len,
                input.original().len()
            )));
        }

        Ok(data)
    }
}
