use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult};

#[perfect_derive(Clone, Copy)]
pub struct Custom<F: Fn(&mut ParserInput) -> ParserResult<O>, O> {
    func: F,
}

impl<F: Fn(&mut ParserInput) -> ParserResult<O>, O> Custom<F, O> {
    pub const fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F: Fn(&mut ParserInput) -> ParserResult<O>, O> Parser<O> for Custom<F, O> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<O> {
        (self.func)(input)
    }
}
