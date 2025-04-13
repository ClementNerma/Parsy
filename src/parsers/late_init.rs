use std::{cell::RefCell, rc::Rc};

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult};

#[perfect_derive(Debug, Clone, Copy)]
pub struct LateInit<T> {
    parser_ref: Rc<RefCell<Option<Box<dyn Parser<T>>>>>,
}

impl<T> LateInit<T> {
    pub fn new() -> Self {
        Self {
            parser_ref: Rc::new(RefCell::new(None)),
        }
    }

    pub fn define(&self, parser: impl Parser<T> + 'static) {
        let mut borrowed = self.parser_ref.borrow_mut();

        assert!(borrowed.is_none(), "This late parser was already set");

        borrowed.replace(Box::new(parser));
    }
}

impl<T> Default for LateInit<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Parser<T> for LateInit<T> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<T> {
        self.parser_ref
            .borrow()
            .as_ref()
            .expect("Weak parser reference was not created yet :(")
            .parse(input)
    }
}
