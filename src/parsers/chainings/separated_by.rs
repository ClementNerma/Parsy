use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{
    Parser, ParserInput, ParserNonConstUtils, ParserResult, ParsingError, Span,
    containers::Container,
};

/// See [`separated_by`](`crate::ParserConstUtils::separated_by`)
#[perfect_derive(Debug, Clone, Copy)]
pub struct SeparatedBy<T, TP: Parser<T>, S, SP: Parser<S>, C: Container<T>> {
    parser: TP,
    separator: SP,
    min: Option<usize>,
    max: Option<usize>,
    exactly: Option<usize>,
    critical_if_fails_after_sep: Option<&'static str>,
    _p: PhantomData<(T, S, C)>,
}

impl<T, TP: Parser<T>, S, SP: Parser<S>, C: Container<T>> SeparatedBy<T, TP, S, SP, C> {
    pub const fn new(parser: TP, separator: SP) -> Self {
        Self {
            parser,
            separator,
            min: None,
            max: None,
            exactly: None,
            critical_if_fails_after_sep: None,
            _p: PhantomData,
        }
    }

    /// Require the parser to succeed at least the provided number of times (successively)
    ///
    /// Panics if [`Self::at_most`] or [`Self::exactly`] were called
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

    /// Require the parser to succeed at most the provided number of times (successively)
    ///
    /// Panics if [`Self::at_least`] or [`Self::exactly`] were called
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

    /// Require the parser to succeed exactly the provided number of times (successively)
    ///
    /// Panics if [`Self::at_least`] or [`Self::at_most`] were called
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

    /// Return a critical error if the parser fails after the separator
    ///
    /// If not set, no error will be returned, as the parser will simply stop with
    /// what it will have collected into the container
    pub fn critical_if_fails_after_sep(mut self, msg: &'static str) -> Self {
        self.critical_if_fails_after_sep = Some(msg);
        self
    }
}

impl<T, TP: Parser<T>, S, SP: Parser<S>, C: Container<T>> Parser<C>
    for SeparatedBy<T, TP, S, SP, C>
{
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<C> {
        let mut out = C::create();
        let mut size = 0;
        let mut ate = 0;
        let mut ate_separator = None;
        let start = input.at();

        let err = loop {
            let parsed = match self.parser.parse(input) {
                Ok(parsed) => parsed,
                Err(err) => {
                    if err.is_critical() {
                        return Err(err);
                    } else if size > 0
                        && let Some(msg) = self.critical_if_fails_after_sep
                    {
                        return Err(ParsingError::custom(
                            input.at().add(ate + ate_separator.unwrap_or(0)).range(0),
                            "repeated value parser failed after separator",
                        )
                        .criticalize(msg));
                    } else {
                        break Some(err);
                    }
                }
            };

            if let Some(ate_separator) = ate_separator {
                ate += ate_separator;
            }

            out.push(parsed.data);
            ate += parsed.at.len;
            size += 1;

            if let Some(max) = self.max
                && size > max
            {
                break None;
            }

            if let Some(exactly) = self.exactly
                && size == exactly
            {
                break None;
            }

            match self.separator.parse(input) {
                Ok(parsed) => {
                    ate_separator = Some(parsed.at.len);
                }
                Err(err) => {
                    if err.is_critical() {
                        return Err(err);
                    } else {
                        break None;
                    }
                }
            }
        };

        if let Some(min) = self.min
            && size < min
        {
            return Err(err.filter(|_| size == 0).unwrap_or_else(|| {
                ParsingError::custom(input.at().range(ate), "Not enough repetitions")
            }));
        }

        Ok(Span::ate(start.range(ate), out))
    }
}
