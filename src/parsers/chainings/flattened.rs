use std::marker::PhantomData;

use crate::{container::Container, PResult, Parser, ParserInput};

#[derive(Clone)]
pub struct Flattened<
    T,
    S: IntoIterator<Item = T>,
    I: IntoIterator<Item = S>,
    P: Parser<I>,
    C: Container<T>,
> {
    parser: P,
    _i: PhantomData<I>,
    _t: PhantomData<T>,
    _c: PhantomData<C>,
}

impl<T, S: IntoIterator<Item = T>, I: IntoIterator<Item = S>, P: Parser<I>, C: Container<T>>
    Flattened<T, S, I, P, C>
{
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            _i: PhantomData,
            _t: PhantomData,
            _c: PhantomData,
        }
    }
}

impl<T, S: IntoIterator<Item = T>, I: IntoIterator<Item = S>, P: Parser<I>, C: Container<T>>
    Parser<C> for Flattened<T, S, I, P, C>
{
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<C> {
        let parsed = self.parser.parse(input)?;
        Ok(parsed.map(|data| C::from_iter(data.into_iter().flatten())))
    }
}
