use std::{borrow::Cow, marker::PhantomData};

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult, ParsingError};

#[perfect_derive(Debug, Clone, Copy)]
pub struct ValidateOrCritical<T, P: Parser<T>, F: Fn(&T) -> Result<(), Cow<'static, str>>> {
    parser: P,
    validator: F,
    _p: PhantomData<T>,
}

impl<T, P: Parser<T>, F: Fn(&T) -> Result<(), Cow<'static, str>>> ValidateOrCritical<T, P, F> {
    pub const fn new(parser: P, validator: F) -> Self {
        Self {
            parser,
            validator,
            _p: PhantomData,
        }
    }
}

impl<T, P: Parser<T>, F: Fn(&T) -> Result<(), Cow<'static, str>>> Parser<T>
    for ValidateOrCritical<T, P, F>
{
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<T> {
        let start = input.at();
        let parsed = self.parser.parse(input)?;

        match (self.validator)(&parsed.data) {
            Ok(()) => Ok(parsed),

            Err(msg) => Err(
                ParsingError::custom(start.range(parsed.at.len), "Validator failed")
                    .criticalize(msg),
            ),
        }
    }
}
