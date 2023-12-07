use std::{borrow::Cow, marker::PhantomData};

use perfect_derive::perfect_derive;

use crate::{PResult, Parser, ParserInput};

#[perfect_derive(Debug, Clone, Copy)]
pub struct Critical<T, P: Parser<T>> {
    parser: P,
    message: Option<&'static str>,
    unexpected_eof_msg: bool,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>> Critical<T, P> {
    pub fn new(parser: P, message: Option<&'static str>) -> Self {
        Self {
            parser,
            message,
            unexpected_eof_msg: true,
            _t: PhantomData,
        }
    }

    pub fn unexpected_eof_msg(mut self, enable: bool) -> Self {
        self.unexpected_eof_msg = enable;
        self
    }
}

impl<T, P: Parser<T>> Parser<T> for Critical<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<T> {
        let is_empty = input.inner().is_empty();

        self.parser.parse(input).map_err(|err| {
            let message = if is_empty && self.unexpected_eof_msg {
                Cow::Borrowed("unexpected end of input")
            } else {
                match self.message {
                    Some(message) => Cow::Borrowed(message),
                    None => match err.atomic_error() {
                        Some(message) => Cow::Borrowed(message),
                        None => Cow::Owned(format!("{}", err.inner().expected())),
                    },
                }
            };

            err.criticalize(message)
        })
    }
}
