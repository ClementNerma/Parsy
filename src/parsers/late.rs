use std::{cell::RefCell, rc::Rc};

use crate::{PResult, Parser, ParserInput};

pub struct Late<T> {
    parser_ref: Rc<RefCell<Option<Box<dyn Parser<T>>>>>,
}

impl<T> Late<T> {
    pub fn new() -> Self {
        Self {
            parser_ref: Rc::new(RefCell::new(None)),
        }
    }

    pub fn finish(&self, parser: impl Parser<T> + 'static) {
        let mut borrowed = self.parser_ref.borrow_mut();

        assert!(borrowed.is_none(), "This late parser was already set");

        borrowed.replace(Box::new(parser));
    }
}

// NOTE: This is required because of https://github.com/rust-lang/rust/issues/26925
impl<T> Clone for Late<T> {
    fn clone(&self) -> Self {
        Self {
            parser_ref: self.parser_ref.clone(),
        }
    }
}

impl<T> Parser<T> for Late<T> {
    fn parse_inner<'a>(&self, input: &mut ParserInput<'a>) -> PResult<T> {
        self.parser_ref
            .borrow()
            .as_ref()
            .expect("Weak parser reference was not created yet :(")
            .parse(input)
    }
}
