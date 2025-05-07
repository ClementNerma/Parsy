use std::{any::Any, marker::PhantomData};

use perfect_derive::perfect_derive;

use crate::{Parser, ParserInput, ParserResult, ParsingError, Span};

#[perfect_derive(Clone, Copy)]
pub struct GetContext<C> {
    _c: PhantomData<C>,
    not_critical: bool,
}

impl<C> GetContext<C> {
    pub const fn new() -> Self {
        Self {
            _c: PhantomData,
            not_critical: false,
        }
    }

    pub const fn not_critical(mut self) -> Self {
        self.not_critical = true;
        self
    }
}

impl<C> Default for GetContext<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Any> Parser<Box<C>> for GetContext<C> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<Box<C>> {
        let logic = || -> Result<_, _> {
            let ctx = input.get_ctx().ok_or(
                "Internal error: Expected a context in the parser input, but context is missing",
            )?;

            <Box<dyn Any>>::downcast::<C>(ctx)
                .map_err(|_| "Internal error: Context found in the parser input does not have the expected type")
        };

        logic()
            .map(|ctx| Span {
                at: input.range(0),
                data: ctx,
            })
            .map_err(|msg| {
                let mut err = ParsingError::custom(input.range(0), msg);

                if !self.not_critical {
                    err = err.criticalize(msg);
                }

                err
            })
    }
}
