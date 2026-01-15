use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserNonConstUtils, ParserResult, ParsingError};

#[perfect_derive(Debug, Clone, Copy)]
pub struct Full<T, P: Parser<T>> {
    parser: P,
    _p: PhantomData<T>,
}

impl<T, P: Parser<T>> Full<T, P> {
    pub const fn new(parser: P) -> Self {
        Self {
            parser,
            _p: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<T> for Full<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<T> {
        if input.offset() > 0 {
            return Err(ParsingError::custom(
                input.at().range(0),
                "Expected start of input",
            ));
        }

        let data = self.parser.parse(input)?;

        if data.at.len < input.original().len() {
            let next_char = input.original()[data.at.len..].chars().next().unwrap();

            return Err(ParsingError::custom(
                data.at.start.add(data.at.len).range(next_char.len_utf8()),
                "Unexpected symbol",
            ));
        }

        assert!(data.at.len == input.original().len());

        Ok(data)
    }
}
