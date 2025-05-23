use std::{borrow::Cow, fmt};

use crate::{CodeLocation, CodeRange, Span};

pub type ParserResult<T> = ::std::result::Result<Span<T>, ParsingError>;

#[derive(Debug)]
pub struct ParsingError {
    inner: ParsingErrorInner,
    atomic_error: Option<&'static str>,
    critical: Option<Cow<'static, str>>,
}

impl ParsingError {
    pub const fn new(inner: ParsingErrorInner) -> Self {
        Self {
            inner,
            atomic_error: None,
            critical: None,
        }
    }

    pub const fn inner(&self) -> &ParsingErrorInner {
        &self.inner
    }

    pub fn into_inner(self) -> ParsingErrorInner {
        self.inner
    }

    pub fn critical_message(&self) -> Option<&str> {
        self.critical.as_deref()
    }

    pub const fn is_critical(&self) -> bool {
        self.critical.is_some()
    }

    pub fn criticalize(mut self, critical: impl Into<Cow<'static, str>>) -> Self {
        if self.critical.is_none() {
            self.critical = Some(critical.into());
        }

        self
    }

    pub const fn atomic_error(&self) -> Option<&'static str> {
        self.atomic_error
    }

    pub const fn with_atomic_error(mut self, atomic_err: &'static str) -> Self {
        self.atomic_error = Some(atomic_err);
        self
    }

    pub const fn expected_char(range: CodeRange, expected: char) -> ParsingError {
        ParsingError::new(ParsingErrorInner::new(
            range,
            ParserExpectation::Char(expected),
        ))
    }

    pub const fn expected_str(range: CodeRange, expected: &'static str) -> ParsingError {
        ParsingError::new(ParsingErrorInner::new(
            range,
            ParserExpectation::Str(expected),
        ))
    }

    pub const fn custom(range: CodeRange, message: &'static str) -> ParsingError {
        ParsingError::new(ParsingErrorInner::new(
            range,
            ParserExpectation::Custom(message),
        ))
    }

    pub const fn just_break(loc: CodeLocation) -> ParsingError {
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
    pub const fn new(at: CodeRange, expected: ParserExpectation) -> Self {
        Self { at, expected }
    }

    pub const fn at(&self) -> CodeRange {
        self.at
    }

    pub const fn is_empty(&self) -> bool {
        self.at.len == 0
    }

    pub const fn expected(&self) -> &ParserExpectation {
        &self.expected
    }
}
