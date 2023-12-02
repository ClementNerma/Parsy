use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

pub struct SilentChoice<T: IntoSilentChoice<O>, O> {
    parsers: T,
    _o: PhantomData<O>,
}

impl<T: IntoSilentChoice<O>, O> SilentChoice<T, O> {
    pub fn new(parsers: T) -> Self {
        Self {
            parsers,
            _o: PhantomData,
        }
    }
}

/// This whole `IntoSilentChoice` thing is here to ensure that all parsers provided
/// when creating a `SilentChoice` are actually parsers and that they all output the
/// same exact type.
pub trait IntoSilentChoice<O> {
    fn into_silent_choice(self) -> SilentChoice<Self, O>
    where
        Self: Sized;
}

macro_rules! _impl_silent_choice {
    () => {};

    ($head: ident $($X: ident)*) => {
        _impl_silent_choice!($($X)*);
        _impl_silent_choice!(~ $head $($X)*);
    };

    (~ $($X: ident)+) => {
        impl<$($X: Parser<Output>),+, Output> IntoSilentChoice<Output> for ($($X,)+) {
            fn into_silent_choice(self) -> SilentChoice<Self, Output> where Self: Sized {
                SilentChoice::new(self)
            }
        }

        // NOTE: This is required because of https://github.com/rust-lang/rust/issues/26925
        impl<$($X: Parser<Output> + Clone),+, Output> Clone for SilentChoice<($($X,)+), Output> {
            fn clone(&self) -> Self {
                Self {
                    parsers: self.parsers.clone(),
                    _o: PhantomData
                }
            }
        }

        impl<$($X: Parser<Output>),+, Output> Parser<()> for SilentChoice<($($X,)+), Output> {
            fn parse_inner(&self, input: &mut ParserInput) -> PResult<()> {
                #[allow(non_snake_case)]
                let SilentChoice { parsers: ($($X,)+), _o: _ } = &self;

                // let mut errors = vec![];

                $(
                    match $X.parse(input) {
                        Ok(result) => return Ok(result.replace(())),
                        Err(err) if err.is_critical() => return Err(err),
                        Err(_) => {} // errors.push(err)
                    }
                )+

                Err(input.range(0).custom_err("none of choices matched"))
            }
        }
    }
}

_impl_silent_choice!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);
