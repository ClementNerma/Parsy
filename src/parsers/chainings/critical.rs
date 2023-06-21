use std::{borrow::Cow, marker::PhantomData};

use crate::{CriticalErrorMsgContent, CriticalErrorNature, PResult, Parser, ParserInput};

#[derive(Clone)]
pub struct Critical<T, P: Parser<T>> {
    parser: P,
    message: Option<&'static str>,
    criticallize_eoi: bool,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>> Critical<T, P> {
    pub fn new(parser: P, message: Option<&'static str>, criticallize_eoi: bool) -> Self {
        Self {
            parser,
            message,
            criticallize_eoi,
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<T> for Critical<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<T> {
        self.parser.parse(input).map_err(|err| {
            let msg = match self.message {
                Some(msg) => CriticalErrorMsgContent::Custom(Cow::Borrowed(msg)),
                None => CriticalErrorMsgContent::Inherit,
            };

            err.criticalize(if self.criticallize_eoi && input.inner().is_empty() {
                CriticalErrorNature::UnexpectedEndOfInput(msg)
            } else {
                CriticalErrorNature::Direct(msg)
            })
        })
    }
}
