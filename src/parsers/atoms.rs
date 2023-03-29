use crate::{container::NoAllocContainer, filter, Parser};

pub fn whitespace() -> impl Parser<char> + Clone {
    filter(|c| c.is_whitespace())
}

pub fn digit(radix: u32) -> impl Parser<char> + Clone {
    filter(move |c| c.is_digit(radix))
}

pub fn digits(radix: u32) -> impl Parser<NoAllocContainer> + Clone {
    digit(radix).repeated().at_least(1)
}

pub fn digits_vec<O>(radix: u32) -> impl Parser<Vec<char>> + Clone {
    digit(radix).repeated_vec().at_least(1)
}

pub fn alphabetic() -> impl Parser<char> + Clone {
    filter(|c| c.is_alphabetic())
}

pub fn alphanumeric() -> impl Parser<char> + Clone {
    filter(|c| c.is_alphanumeric())
}

pub fn any_char() -> impl Parser<char> + Clone {
    filter(|_| true)
}
