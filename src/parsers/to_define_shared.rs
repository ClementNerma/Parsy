use std::sync::{Arc, OnceLock};

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult};

#[perfect_derive(Debug, Clone, Copy)]
pub struct ToDefineShared<T> {
    parser_ref: Arc<OnceLock<Box<dyn Parser<T>>>>,
}

impl<T> ToDefineShared<T> {
    pub fn new() -> Self {
        Self {
            parser_ref: Arc::new(OnceLock::new()),
        }
    }

    pub fn define(&self, parser: impl Parser<T> + 'static) {
        if self.parser_ref.set(Box::new(parser)).is_err() {
            panic!("The .define() method was already called on this parser")
        }
    }
}

impl<T> Default for ToDefineShared<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Parser<T> for ToDefineShared<T> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<T> {
        self.parser_ref
            .get()
            .expect("The .define() method was not called yet on this parser")
            .parse(input)
    }
}
