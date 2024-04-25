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

        let eaten = input
            // Try to eat the string
            .try_eat(self.str.len())
            // Otherwise, generate an error
            .ok_or_else(|| start.expected_str(self.str, 0))?;

        // Ensure it was correctly eaten
        if eaten.data == self.str {
            // If so, replace success data with the stored string to get a 'static lifetime
            Ok(eaten.replace(self.str))
        } else {
            // Otherwise, generate an error
            Err(start.expected_str(self.str, eaten.at.len))
        }
    }
}
