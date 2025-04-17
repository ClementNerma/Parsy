use std::{cell::RefCell, rc::Rc};

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult};

#[perfect_derive(Debug, Clone, Copy)]
pub struct ToDefine<T> {
    parser_ref: Rc<RefCell<Option<Box<dyn Parser<T>>>>>,
}

impl<T> ToDefine<T> {
    pub fn new() -> Self {
        Self {
            parser_ref: Rc::new(RefCell::new(None)),
        }
    }

    pub fn define(&self, parser: impl Parser<T> + 'static) {
        let mut borrowed = self.parser_ref.borrow_mut();

        let prev = borrowed.replace(Box::new(parser));

        assert!(
            prev.is_none(),
            "The .define() method was already called on this parser"
        );
    }
}

impl<T> Default for ToDefine<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Parser<T> for ToDefine<T> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<T> {
        self.parser_ref
            .borrow()
            .as_ref()
            .expect("The .define() method was not called yet on this parser")
            .parse(input)
    }
}
