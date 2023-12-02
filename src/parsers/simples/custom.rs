use perfect_derive::perfect_derive;

use crate::{PResult, Parser, ParserInput};

#[perfect_derive(Clone, Copy)]
pub struct Custom<F: Fn(&mut ParserInput) -> PResult<O>, O> {
    func: F,
}

impl<F: Fn(&mut ParserInput) -> PResult<O>, O> Custom<F, O> {
    pub fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F: Fn(&mut ParserInput) -> PResult<O>, O> Parser<O> for Custom<F, O> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<O> {
        (self.func)(input)
    }
}
