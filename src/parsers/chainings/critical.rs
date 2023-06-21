use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

#[derive(Clone)]
pub struct Critical<T, P: Parser<T>> {
    parser: P,
    message: &'static str,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>> Critical<T, P> {
    pub fn new(parser: P, message: &'static str) -> Self {
        Self {
            parser,
            message,
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T>> Parser<T> for Critical<T, P> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<T> {
        self.parser.parse(input).map_err(|err| {
            // let msg = match self.message {
            //     Some(msg) => CriticalErrorMsgContent::Custom(msg),
            //     None => CriticalErrorMsgContent::Inherit,
            // };

            // err.criticalize(if self.criticallize_eoi && input.inner().is_empty() {
            //     CriticalErrorNature::UnexpectedEndOfInput(msg)
            // } else {
            //     CriticalErrorNature::Direct(msg)
            // })

            err.criticalize(self.message)
        })
    }
}
