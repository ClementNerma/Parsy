use std::marker::PhantomData;

use crate::{container::Container, Eaten, PResult, Parser, ParserInput};

pub struct Repeated<T, P: Parser<T>, C: Container<T>> {
    parser: P,
    min: Option<usize>,
    max: Option<usize>,
    exactly: Option<usize>,
    _p: PhantomData<T>,
    _c: PhantomData<C>,
}

impl<T, P: Parser<T>, C: Container<T>> Repeated<T, P, C> {
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            min: None,
            max: None,
            exactly: None,
            _p: PhantomData,
            _c: PhantomData,
        }
    }

    pub fn at_least(mut self, min: usize) -> Self {
        assert!(
            self.exactly.is_none(),
            "Cannot specify both a minimum and an exact number of repetitions"
        );

        if let Some(max) = self.max {
            assert!(min <= max, "Minimum number of repetitions ({min}) cannot be higher than the maximum ({max}) number of repetitoins");
        }

        self.min = Some(min);
        self
    }

    pub fn at_most(mut self, max: usize) -> Self {
        assert!(
            self.exactly.is_none(),
            "Cannot specify both a maximum and an exact number of repetitions"
        );

        if let Some(min) = self.min {
            assert!(min <= max, "Minimum number of repetitions ({min}) cannot be higher than the maximum ({max}) number of repetitoins");
        }

        self.max = Some(max);
        self
    }

    pub fn exactly(mut self, exactly: usize) -> Self {
        assert!(
            self.min.is_none(),
            "Cannot specify both a minimum and an exact number of repetitions"
        );

        assert!(
            self.max.is_none(),
            "Cannot specify both a maximum and an exact number of repetitions"
        );

        self.exactly = Some(exactly);
        self
    }
}

// NOTE: This is required because of https://github.com/rust-lang/rust/issues/26925
impl<T, P: Parser<T> + Clone, C: Container<T>> Clone for Repeated<T, P, C> {
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            min: self.min,
            max: self.max,
            exactly: self.exactly,
            _p: PhantomData,
            _c: PhantomData,
        }
    }
}

impl<T, P: Parser<T>, C: Container<T>> Parser<C> for Repeated<T, P, C> {
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<C> {
        let start = input.at();
        let mut eaten_len = 0;

        let mut out = C::create();
        let mut size = 0;

        let err = loop {
            match self.parser.parse(input) {
                Err(err) if err.is_critical() => return Err(err),
                Err(err) => break Some(err),
                Ok(eaten) => {
                    eaten_len += eaten.at.len;
                    size += 1;

                    out.add(eaten.data);

                    if let Some(max) = self.max {
                        if size > max {
                            break None;
                        }
                    }

                    if let Some(exactly) = self.exactly {
                        if size == exactly {
                            break None;
                        }
                    }
                }
            }
        };

        if let Some(min) = self.min {
            if size < min {
                return Err(err
                    .filter(|_| min == 0)
                    .unwrap_or_else(|| input.range(0).custom_err("Not enough repetitions")));
            }
        }

        Ok(Eaten::ate(start.range(eaten_len), out))
    }
}
