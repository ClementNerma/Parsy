use std::marker::PhantomData;

use crate::{PResult, Parser, ParserInput};

pub struct SilentChoice<T: IntoSilentChoice<Outputs>, Outputs> {
    parsers: T,
    _o: PhantomData<Outputs>,
}

impl<T: IntoSilentChoice<Outputs>, Outputs> SilentChoice<T, Outputs> {
    pub fn new(parsers: T) -> Self {
        Self {
            parsers,
            _o: PhantomData,
        }
    }
}

pub trait IntoSilentChoice<Outputs> {
    fn into_silent_choice(self) -> SilentChoice<Self, Outputs>
    where
        Self: Sized;
}

macro_rules! _impl_silent_choice {
    () => {};

    ($head: ident[$head_o: ident], $($X: ident[$Xo: ident],)*) => {
        _impl_silent_choice!($($X[$Xo],)*);
        _impl_silent_choice!(~ $head[$head_o], $($X[$Xo],)*);
    };

    (~ $($X: ident[$Xo: ident],)+) => {
        impl<$($X: Parser<$Xo>, $Xo),+> IntoSilentChoice<($($Xo,)+)> for ($($X,)+) {
            fn into_silent_choice(self) -> SilentChoice<Self, ($($Xo,)+)> where Self: Sized {
                SilentChoice::<Self, ($($Xo,)+)>::new(self)
            }
        }

        // NOTE: This is required because of https://github.com/rust-lang/rust/issues/26925
        impl<$($X: Parser<$Xo> + Clone, $Xo),+> Clone for SilentChoice<($($X,)+), ($($Xo,)+)> {
            fn clone(&self) -> Self {
                Self { parsers: self.parsers.clone(), _o: PhantomData }
            }
        }

        impl<$($X: Parser<$Xo>, $Xo),+> Parser<()> for SilentChoice<($($X,)+), ($($Xo,)+)> {
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

_impl_silent_choice!(
    A[AA], B[BB], C[CC], D[DD], E[EE], F[FF], G[GG], H[HH], I[II], J[JJ], K[KK], L[LL], M[MM],
    N[NN], O[OO], P[PP], Q[QQ], R[RR], S[SS], T[TT], U[UU], V[VV], W[WW], X[XX], Y[YY], Z[ZZ],
);
