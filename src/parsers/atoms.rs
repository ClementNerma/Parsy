use crate::{Parser, container::NoAllocContainer, filter};

pub fn whitespace() -> impl Parser<char> + Copy {
    filter(|c| c.is_whitespace())
}

pub fn digit(radix: u32) -> impl Parser<char> + Copy {
    filter(move |c| c.is_digit(radix))
}

pub fn digits(radix: u32) -> impl Parser<NoAllocContainer> + Copy {
    digit(radix).repeated().at_least(1)
}

pub fn alphabetic() -> impl Parser<char> + Copy {
    filter(|c| c.is_alphabetic())
}

pub fn alphanumeric() -> impl Parser<char> + Copy {
    filter(|c| c.is_alphanumeric())
}

pub fn any_char() -> impl Parser<char> + Copy {
    filter(|_| true)
}
