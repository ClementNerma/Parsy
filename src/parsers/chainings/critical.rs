use std::{borrow::Cow, marker::PhantomData};

use perfect_derive::perfect_derive;

use crate::{PResult, Parser, ParserInput};

#[perfect_derive(Debug, Clone, Copy)]
pub struct Critical<T, P: Parser<T>> {
    parser: P,
    message: Option<&'static str>,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>> Critical<T, P> {
    pub fn new(parser: P, message: Option<&'static str>) -> Self {
        Self {
            parser,
            message,
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<T> for Critical<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<T> {
        let is_empty = input.inner().is_empty();

        self.parser.parse(input).map_err(|err| {
            let message = match self.message {
                Some(message) => Cow::Borrowed(message),
                None => match err.atomic_error() {
                    Some(message) => Cow::Borrowed(message),
                    None => {
                        if is_empty {
                            Cow::Borrowed("unexpected end of input")
                        } else {
                            Cow::Owned(format!("{}", err.inner().expected()))
                        }
                    }
                },
            };

            err.criticalize(message)
        })
    }
}
