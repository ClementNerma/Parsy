use crate::{chainings::DelimitedBy, Parser};

use super::Whitespaces;

pub type Padded<T, P> = DelimitedBy<(), Whitespaces, T, P, (), Whitespaces>;

impl<M, MP: Parser<M>> Padded<M, MP> {
    pub fn padded(middle: MP) -> Padded<M, MP> {
        Padded::new(Whitespaces::new(), middle, Whitespaces::new())
    }
}
