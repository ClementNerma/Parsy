use crate::{PResult, Parser, ParserInput};

pub struct Custom<F: Fn(&mut ParserInput) -> PResult<O>, O> {
    func: F,
}

impl<F: Fn(&mut ParserInput) -> PResult<O>, O> Custom<F, O> {
    pub fn new(func: F) -> Self {
        Self { func }
    }
}

// NOTE: This is required because of https://github.com/rust-lang/rust/issues/26925
impl<F: Fn(&mut ParserInput) -> PResult<O> + Clone, O> Clone for Custom<F, O> {
    fn clone(&self) -> Self {
        Self {
            func: self.func.clone(),
        }
    }
}

impl<F: Fn(&mut ParserInput) -> PResult<O>, O> Parser<O> for Custom<F, O> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<O> {
        (self.func)(input)
    }
}
