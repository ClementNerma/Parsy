use std::{borrow::Cow, marker::PhantomData};

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult, ParsingError};

#[perfect_derive(Debug, Clone, Copy)]
pub struct ValidateOrCriticalMsg<T, P: Parser<T>, F: Fn(&T) -> bool> {
    parser: P,
    validator: F,
    message: &'static str,
    _p: PhantomData<T>,
}

impl<T, P: Parser<T>, F: Fn(&T) -> bool> ValidateOrCriticalMsg<T, P, F> {
    pub const fn new(parser: P, validator: F, message: &'static str) -> Self {
        Self {
            parser,
            validator,
            message,
            _p: PhantomData,
        }
    }
}

impl<T, P: Parser<T>, F: Fn(&T) -> bool> Parser<T> for ValidateOrCriticalMsg<T, P, F> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<T> {
        let start = input.at();
        let parsed = self.parser.parse(input)?;

        if (self.validator)(&parsed.data) {
            Ok(parsed)
        } else {
            Err(
                ParsingError::custom(start.range(parsed.at.len), "Validator failed")
                    .criticalize(Cow::Borrowed(self.message)),
            )
        }
    }
}
