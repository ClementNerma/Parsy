use std::sync::LazyLock;

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult};

#[perfect_derive(Debug, Clone, Copy)]
pub struct LazilyDefined<T> {
    parser_ref: LazyLock<Box<dyn Parser<T> + Send + Sync>>,
}

impl<T> LazilyDefined<T> {
    pub fn new(lazy_define: fn() -> Box<dyn Parser<T> + Send + Sync>) -> Self {
        Self {
            parser_ref: LazyLock::new(lazy_define),
        }
    }

    pub fn init(&self) {
        LazyLock::force(&self.parser_ref);
    }
}

impl<T> Parser<T> for LazilyDefined<T> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<T> {
        self.parser_ref.parse(input)
    }
}
