use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{container::Container, Eaten, PResult, Parser, ParserInput, ParsingError};

#[perfect_derive(Debug, Clone, Copy)]
pub struct SeparatedBy<T, TP: Parser<T>, S, SP: Parser<S>, C: Container<T>> {
    parser: TP,
    separator: SP,
    min: Option<usize>,
    max: Option<usize>,
    exactly: Option<usize>,
    _t: PhantomData<T>,
    _s: PhantomData<S>,
    _c: PhantomData<C>,
}

impl<T, TP: Parser<T>, S, SP: Parser<S>, C: Container<T>> SeparatedBy<T, TP, S, SP, C> {
    pub fn new(parser: TP, separator: SP) -> Self {
        Self {
            parser,
            separator,
            min: None,
            max: None,
            exactly: None,
            _t: PhantomData,
            _s: PhantomData,
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

impl<T, TP: Parser<T>, S, SP: Parser<S>, C: Container<T>> Parser<C>
    for SeparatedBy<T, TP, S, SP, C>
{
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<C> {
        let mut out = C::create();
        let mut size = 0;
        let mut ate = 0;
        let start = input.at();

        let err = loop {
            let parsed = match self.parser.parse(input) {
                Ok(parsed) => parsed,
                Err(err) if err.is_critical() => return Err(err),
                Err(err) => break Some(err),
            };

            out.add(parsed.data);
            ate += parsed.at.len;
            size += 1;

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

            match self.separator.parse(input) {
                Ok(parsed) => ate += parsed.at.len,
                Err(err) => {
                    if err.is_critical() {
                        return Err(err);
                    } else {
                        break None;
                    }
                }
            }
        };

        if let Some(min) = self.min {
            if size < min {
                return Err(err.filter(|_| size == 0).unwrap_or_else(|| {
                    ParsingError::custom(input.at().range(ate), "Not enough repetitions")
                }));
            }
        }

        Ok(Eaten::ate(start.range(ate), out))
    }
}
