use crate::{PResult, Parser, ParserInput};

#[derive(Clone)]
pub struct Custom<F: Fn(&mut ParserInput) -> PResult<O> + Clone, O> {
    func: F,
}

impl<'f, F: Fn(&mut ParserInput) -> PResult<O> + Clone, O> Custom<F, O> {
    pub fn new(func: F) -> Self {
        Self { func }
    }
}

impl<'f, F: Fn(&mut ParserInput) -> PResult<O> + Clone, O> Parser<O> for Custom<F, O> {
    fn parse_inner<'a>(&self, input: &mut ParserInput<'a>) -> PResult<O> {
        (self.func)(input)
    }
}
