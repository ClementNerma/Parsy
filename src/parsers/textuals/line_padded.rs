use crate::{Parser, chainings::DelimitedBy};

use super::Whitespaces;

pub type LinePadded<T, P> = DelimitedBy<(), Whitespaces, T, P, (), Whitespaces>;

impl<M, MP: Parser<M>> LinePadded<M, MP> {
    pub const fn line_padded(middle: MP) -> LinePadded<M, MP> {
        LinePadded::new(
            Whitespaces::new().no_newline(),
            middle,
            Whitespaces::new().no_newline(),
        )
    }
}
