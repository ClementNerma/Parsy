use perfect_derive::perfect_derive;

use crate::{PResult, Parser, ParserInput};

#[perfect_derive(Clone, Copy)]
pub struct Filter<F: Fn(char) -> bool> {
    func: F,
}

impl<F: Fn(char) -> bool> Filter<F> {
    pub fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F: Fn(char) -> bool> Parser<char> for Filter<F> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<char> {
        let start = input.at();

        let c = input
            .try_eat_char()
            .ok_or_else(|| start.range(0).custom_err("No character left"))?;

        if (self.func)(c.data) {
            Ok(c)
        } else {
            Err(start.range(c.data.len_utf8()).custom_err("Character filter failed"))
        }
    }
}
