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
            .try_eat(self.str.as_bytes().len())
            .filter(|eaten| eaten.data.starts_with(self.str))
            .map(|eaten| eaten.replace(self.str))
            .ok_or_else(|| start.range(0).expected_str(self.str))
    }
}
