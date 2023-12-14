use crate::{PResult, Parser, ParserInput};

#[derive(Clone, Copy)]
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
            .try_eat(self.str.len())
            // Ensure it was correctly eaten
            .filter(|eaten| eaten.data == self.str)
            // If so, replace success data with the stored string to get a 'static lifetime
            .map(|eaten| eaten.replace(self.str))
            // Otherwise, generate an error
            .ok_or_else(|| start.expected_str(self.str))
    }
}
