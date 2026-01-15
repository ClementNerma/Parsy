use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserNonConstUtils, ParserResult, ParsingError};

/// See [`validate`](`crate::ParserConstUtils::validate`)
#[perfect_derive(Debug, Clone, Copy)]
pub struct Validate<T, P: Parser<T>, F: Fn(&T) -> bool> {
    parser: P,
    validator: F,
    _p: PhantomData<T>,
}

impl<T, P: Parser<T>, F: Fn(&T) -> bool> Validate<T, P, F> {
    pub const fn new(parser: P, validator: F) -> Self {
        Self {
            parser,
            validator,
            _p: PhantomData,
        }
    }
}

impl<T, P: Parser<T>, F: Fn(&T) -> bool> Parser<T> for Validate<T, P, F> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<T> {
        let start = input.at();
        let parsed = self.parser.parse(input)?;

        if (self.validator)(&parsed.data) {
            Ok(parsed)
        } else {
            Err(ParsingError::custom(
                start.range(parsed.at.len),
                "validator failed",
            ))
        }
    }
}
