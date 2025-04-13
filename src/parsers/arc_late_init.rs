use std::sync::{Arc, RwLock};

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult};

#[perfect_derive(Debug, Clone, Copy)]
pub struct ArcLateInit<T> {
    parser_ref: Arc<RwLock<Option<Box<dyn Parser<T> + Send + Sync>>>>,
}

impl<T> ArcLateInit<T> {
    pub fn new() -> Self {
        Self {
            parser_ref: Arc::new(RwLock::new(None)),
        }
    }

    pub fn define(&self, parser: impl Parser<T> + Send + Sync + 'static) {
        let mut borrowed = self.parser_ref.write().unwrap();

        assert!(borrowed.is_none(), "This late parser was already set");

        borrowed.replace(Box::new(parser));
    }
}

impl<T> Default for ArcLateInit<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Parser<T> for ArcLateInit<T> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<T> {
        self.parser_ref
            .read()
            .unwrap()
            .as_ref()
            .expect("Weak parser reference was not created yet :(")
            .parse(input)
    }
}
