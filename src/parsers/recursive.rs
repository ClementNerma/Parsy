use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use perfect_derive::perfect_derive;

use crate::{PResult, Parser, ParserInput};

#[perfect_derive(Debug, Clone, Copy)]
pub struct Recursive<T> {
    parser: RecursiveRef<T>,
    _t: PhantomData<T>,
}

impl<T> Recursive<T> {
    pub fn declarative<P: Parser<T> + 'static>(decl: impl FnOnce(RecursiveRef<T>) -> P) -> Self {
        let mut rf = RecursiveRef::new();

        let parser = decl(rf.clone());

        rf.finish(Box::new(parser));

        Self {
            parser: rf,
            _t: PhantomData,
        }
    }

    pub fn declarative_with_value<P: Parser<T> + 'static, R>(
        decl: impl FnOnce(RecursiveRef<T>) -> (P, R),
    ) -> (Self, R) {
        let mut rf = RecursiveRef::new();

        let (parser, ret) = decl(rf.clone());

        rf.finish(Box::new(parser));

        (
            Self {
                parser: rf,
                _t: PhantomData,
            },
            ret,
        )
    }
}

impl<T> Parser<T> for Recursive<T> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<T> {
        self.parser.parse(input)
    }
}

#[perfect_derive(Debug, Clone, Copy)]
pub struct RecursiveRef<T> {
    parser_ref: Rc<RefCell<Option<Box<dyn Parser<T>>>>>,
}

impl<T> RecursiveRef<T> {
    fn new() -> Self {
        Self {
            parser_ref: Rc::new(RefCell::new(None)),
        }
    }

    fn finish(&mut self, inner: Box<dyn Parser<T>>) {
        assert!(
            self.parser_ref.borrow().is_none(),
            "Cannot replace a weak parser reference's inner value twice"
        );

        *self.parser_ref.borrow_mut() = Some(inner);
    }
}

impl<T> Parser<T> for RecursiveRef<T> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<T> {
        self.parser_ref
            .borrow()
            .as_ref()
            .expect("Weak parser reference was not created yet :(")
            .parse(input)
    }
}
