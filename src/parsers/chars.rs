use super::{helpers::filter, textuals::Filter};

/// Match any character
pub const fn any_char() -> Filter {
    filter(|_| true)
}

/// Match any alphabetic character
pub const fn alphabetic() -> Filter {
    filter(|c| c.is_alphabetic())
}

/// Match any alphabetic or digit character
pub const fn alphanumeric() -> Filter {
    filter(|c| c.is_alphanumeric())
}
