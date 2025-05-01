use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult, container::Container};

#[perfect_derive(Debug, Clone, Copy)]
pub struct Flattened<
    T,
    S: IntoIterator<Item = T>,
    I: IntoIterator<Item = S>,
    P: Parser<I>,
    C: Container<T>,
> {
    parser: P,
    _p: PhantomData<(I, T, C)>,
}

impl<T, S: IntoIterator<Item = T>, I: IntoIterator<Item = S>, P: Parser<I>, C: Container<T>>
    Flattened<T, S, I, P, C>
{
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            _p: PhantomData,
        }
    }
}

impl<T, S: IntoIterator<Item = T>, I: IntoIterator<Item = S>, P: Parser<I>, C: Container<T>>
    Parser<C> for Flattened<T, S, I, P, C>
{
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<C> {
        let parsed = self.parser.parse(input)?;
        Ok(parsed.map(|data| C::from_iter(data.into_iter().flatten())))
    }
}
