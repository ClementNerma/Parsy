use std::sync::{Arc, RwLock};

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserNonConstUtils, ParserResult};

/// See [`crate::helpers::to_define_shared`]
#[perfect_derive(Debug, Clone, Copy)]
pub struct ToDefineShared<T> {
    parser_ref: Arc<RwLock<Option<Box<dyn Parser<T> + Send + Sync>>>>,
}

impl<T> ToDefineShared<T> {
    pub fn new() -> Self {
        Self {
            parser_ref: Arc::new(RwLock::new(None)),
        }
    }

    pub fn define(&self, parser: impl Parser<T> + Send + Sync + 'static) {
        let mut borrowed = self.parser_ref.write().unwrap();

        let prev = borrowed.replace(Box::new(parser));

        assert!(
            prev.is_none(),
            "The .define() method was already called on this parser"
        );
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
            .read()
            .unwrap()
            .as_ref()
            .expect("The .define() method was not called yet on this parser")
            .parse(input)
    }
}
