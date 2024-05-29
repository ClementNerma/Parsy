use std::{borrow::Cow, fmt};

use crate::{CodeLocation, CodeRange, Eaten};

pub type PResult<T> = ::std::result::Result<Eaten<T>, ParsingError>;

#[derive(Debug)]
pub struct ParsingError {
    inner: ParsingErrorInner,
    atomic_error: Option<&'static str>,
    critical: Option<Cow<'static, str>>,
}

impl ParsingError {
    pub fn new(inner: ParsingErrorInner) -> Self {
        Self {
            inner,
            atomic_error: None,
            critical: None,
        }
    }

    pub fn inner(&self) -> &ParsingErrorInner {
        &self.inner
    }

    pub fn into_inner(self) -> ParsingErrorInner {
        self.inner
    }

    pub fn critical(&self) -> Option<&str> {
        self.critical.as_deref()
    }

    pub fn is_critical(&self) -> bool {
        self.critical.is_some()
    }

    pub fn criticalize(mut self, critical: impl Into<Cow<'static, str>>) -> Self {
        if self.critical.is_none() {
            self.critical = Some(critical.into());
        }

        self
    }

    pub fn atomic_error(&self) -> Option<&'static str> {
        self.atomic_error
    }

    pub fn with_atomic_error(mut self, atomic_err: &'static str) -> Self {
        self.atomic_error = Some(atomic_err);
        self
    }

    pub fn expected_char(range: CodeRange, expected: char) -> ParsingError {
        ParsingError::new(ParsingErrorInner::new(
            range,
            ParserExpectation::Char(expected),
        ))
    }

    pub fn expected_str(range: CodeRange, expected: &'static str) -> ParsingError {
        ParsingError::new(ParsingErrorInner::new(
            range,
            ParserExpectation::Str(expected),
        ))
    }

    pub fn custom(range: CodeRange, message: &'static str) -> ParsingError {
        ParsingError::new(ParsingErrorInner::new(
            range,
            ParserExpectation::Custom(message),
        ))
    }

    pub fn just_break(loc: CodeLocation) -> ParsingError {
        ParsingError::new(ParsingErrorInner::new(
            loc.range(0),
            ParserExpectation::Break,
        ))
    }
}

#[derive(Debug)]
pub enum ParserExpectation {
    Char(char),
    Str(&'static str),
    Custom(&'static str),
    Break,
}

impl fmt::Display for ParserExpectation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Char(c) => write!(f, "expected character '{c}'"),
            Self::Str(str) => write!(f, "expected string '{str}'"),
            Self::Custom(custom) => write!(f, "{custom}"),
            Self::Break => {
                write!(f, "parser returned a break instruction")
            }
        }
    }
}

#[derive(Debug)]
#[must_use]
pub struct ParsingErrorInner {
    at: CodeRange,
    expected: ParserExpectation,
}

impl ParsingErrorInner {
    pub fn new(at: CodeRange, expected: ParserExpectation) -> Self {
        Self { at, expected }
    }

    pub fn at(&self) -> CodeRange {
        self.at
    }

    pub fn is_empty(&self) -> bool {
        self.at.len == 0
    }

    pub fn expected(&self) -> &ParserExpectation {
        &self.expected
    }
}
