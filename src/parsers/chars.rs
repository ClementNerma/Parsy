use super::{helpers::filter, textuals::Filter};

pub const fn any_char() -> Filter {
    filter(|_| true)
}

pub const fn alphabetic() -> Filter {
    filter(|c| c.is_alphabetic())
}

pub const fn alphanumeric() -> Filter {
    filter(|c| c.is_alphanumeric())
}
