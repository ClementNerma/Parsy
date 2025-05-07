use super::{helpers::filter, textuals::Filter};

pub fn any_char() -> Filter {
    filter(|_| true)
}

pub fn alphabetic() -> Filter {
    filter(|c| c.is_alphabetic())
}

pub fn alphanumeric() -> Filter {
    filter(|c| c.is_alphanumeric())
}
