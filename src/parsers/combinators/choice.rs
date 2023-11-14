use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

pub struct Choice<T: IntoChoice<O>, O> {
    parsers: T,
    _o: PhantomData<O>,
}

impl<T: IntoChoice<O>, O> Choice<T, O> {
    pub fn new(parsers: T) -> Self {
        Self {
            parsers,
            _o: PhantomData,
        }
    }
}

/// This whole `IntoChoice` thing is here to ensure that all parsers provided
/// when creating a `Choice` are actually parsers and that they all output the
/// same exact type.
pub trait IntoChoice<O> {
    fn into_choice(self) -> Choice<Self, O>
    where
        Self: Sized;
}

macro_rules! _impl_choice {
    () => {};

    ($head: ident $($X: ident)*) => {
        _impl_choice!($($X)*);
        _impl_choice!(~ $head $($X)*);
    };

    (~ $($X: ident)+) => {
        impl<$($X: Parser<Output>),+, Output> IntoChoice<Output> for ($($X,)+) {
            fn into_choice(self) -> Choice<Self, Output> where Self: Sized {
                Choice::new(self)
            }
        }

        // NOTE: This is required because of https://github.com/rust-lang/rust/issues/26925
        impl<$($X: Parser<Output> + Clone),+, Output> Clone for Choice<($($X,)+), Output> {
            fn clone(&self) -> Self {
                Self {
                    parsers: self.parsers.clone(),
                    _o: PhantomData
                }
            }
        }

        impl<$($X: Parser<Output>),+, Output> Parser<Output> for Choice<($($X,)+), Output> {
            fn parse_inner(&self, input: &mut ParserInput) -> PResult<Output> {
                // let mut errors = vec![];

                #[allow(non_snake_case)]
                let Choice { parsers: ($($X,)+), _o: _ } = &self;

                $(
                    // TODO: "parse_inner" instead?
                    match $X.parse(input) {
                        Ok(result) => return Ok(result),
                        Err(err) if err.is_critical() => return Err(err),
                        Err(_) => {} // errors.push(err)
                    }
                )+

                Err(input.range(0).custom_err("none of choices matched"))
            }
        }
    }
}

_impl_choice!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);
