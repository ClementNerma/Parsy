use crate::{PResult, Parser, ParserInput};

#[derive(Clone)]
pub struct Just {
    str: &'static str,
}

impl Just {
    pub fn new(str: &'static str) -> Self {
        Self { str }
    }
}

impl Parser<&'static str> for Just {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<&'static str> {
        let start = input.at();

        input
            // Try to eat the string
            .try_eat(self.str.len(), self.str)
            // Ensure it was correctly eaten
            .filter(|eaten| eaten.data == self.str)
            // Otherwise, generate an error
            .ok_or_else(|| start.range(0).expected_str(self.str))
    }
}
