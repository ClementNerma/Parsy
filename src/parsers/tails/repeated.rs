use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{
    Parser, ParserInput, ParserNonConstUtils, ParserResult, ParsingError, Span,
    containers::Container,
};

/// See [`repeated`](`crate::ParserConstUtils::repeated`)
#[perfect_derive(Debug, Clone, Copy)]
pub struct Repeated<T, P: Parser<T>, C: Container<T>> {
    parser: P,
    min: Option<usize>,
    max: Option<usize>,
    exactly: Option<usize>,
    _p: PhantomData<(T, C)>,
}

impl<T, P: Parser<T>, C: Container<T>> Repeated<T, P, C> {
    pub const fn new(parser: P) -> Self {
        Self {
            parser,
            min: None,
            max: None,
            exactly: None,
            _p: PhantomData,
        }
    }

    pub fn at_least(mut self, min: usize) -> Self {
        assert!(
            self.exactly.is_none(),
            "Cannot specify both a minimum and an exact number of repetitions"
        );

        if let Some(max) = self.max {
            assert!(
                min <= max,
                "Minimum number of repetitions ({min}) cannot be higher than the maximum ({max}) number of repetitoins"
            );
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
            assert!(
                min <= max,
                "Minimum number of repetitions ({min}) cannot be higher than the maximum ({max}) number of repetitoins"
            );
        }

        self.max = Some(max);
        self
    }

    pub const fn exactly(mut self, exactly: usize) -> Self {
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

impl<T, P: Parser<T>, C: Container<T>> Parser<C> for Repeated<T, P, C> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<C> {
        let start = input.at();
        let mut ate = 0;

        let mut out = C::create();
        let mut count = 0;

        let err = loop {
            match self.parser.parse(input) {
                Err(err) if err.is_critical() => return Err(err),
                Err(err) => break Some(err),
                Ok(span) => {
                    ate += span.at.len;
                    count += 1;

                    out.add(span.data);

                    if let Some(max) = self.max
                        && count > max
                    {
                        break None;
                    }

                    if let Some(exactly) = self.exactly
                        && count == exactly
                    {
                        break None;
                    }
                }
            }
        };

        if let Some(min) = self.min
            && count < min
        {
            return Err(err.filter(|_| count == 0).unwrap_or_else(|| {
                ParsingError::custom(input.at().range(ate), "Not enough repetitions")
            }));
        }

        Ok(Span::ate(start.range(ate), out))
    }
}
