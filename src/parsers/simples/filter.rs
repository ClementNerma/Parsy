use crate::{PResult, Parser, ParserInput};

#[derive(Clone, Copy)]
pub struct Filter<F: Fn(char) -> bool> {
    func: F,
}

impl<F: Fn(char) -> bool> Filter<F> {
    pub fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F: Fn(char) -> bool> Parser<char> for Filter<F> {
    fn parse_inner<'a>(&self, input: &mut ParserInput<'a>) -> PResult<char> {
        let start = input.at();

        let c = input
            .try_eat_char()
            .ok_or_else(|| start.range(0).custom_err("No character left"))?;

        if (self.func)(c.data) {
            Ok(c)
        } else {
            Err(start.range(1).custom_err("Character filter failed"))
        }
    }
}
