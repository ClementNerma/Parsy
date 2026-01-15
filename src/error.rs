use std::{borrow::Cow, fmt};

use crate::{InputLocation, InputRange, Span};

/// Result of a parsing operation
///
/// * In case of success, a [`Span<T>`] with the parsed value
/// * In case of failure, a [`ParsingError`] with the error details
pub type ParserResult<T> = ::std::result::Result<Span<T>, ParsingError>;

/// Result of a parsing error
#[derive(Debug)]
pub struct ParsingError {
    /// Content of the error
    inner: ParsingErrorInner,

    /// Optional atomic error
    atomic_error: Option<&'static str>,

    /// Optional critical error emssage
    critical: Option<Cow<'static, str>>,
}

impl ParsingError {
    /// Create a new parsing error
    pub const fn new(inner: ParsingErrorInner) -> Self {
        Self {
            inner,
            atomic_error: None,
            critical: None,
        }
    }

    /// Get the error's inner content
    pub const fn inner(&self) -> &ParsingErrorInner {
        &self.inner
    }

    /// Get the error's inner content (owned)
    pub fn into_inner(self) -> ParsingErrorInner {
        self.inner
    }

    /// Get the error's message if it is critical
    pub fn critical_message(&self) -> Option<&str> {
        self.critical.as_deref()
    }

    /// Check if the error is critical
    pub const fn is_critical(&self) -> bool {
        self.critical.is_some()
    }

    /// Make the error critical, with the provided message
    pub fn criticalize(mut self, critical: impl Into<Cow<'static, str>>) -> Self {
        if self.critical.is_none() {
            self.critical = Some(critical.into());
        }

        self
    }

    /// Get the error's message if it is atomic
    pub const fn atomic_error(&self) -> Option<&'static str> {
        self.atomic_error
    }

    /// make the error atomic
    pub const fn with_atomic_error(mut self, atomic_err: &'static str) -> Self {
        self.atomic_error = Some(atomic_err);
        self
    }

    /// Create an error stating a specific character was expected
    pub const fn expected_char(range: InputRange, expected: char) -> ParsingError {
        ParsingError::new(ParsingErrorInner::new(
            range,
            ParserExpectation::Char(expected),
        ))
    }

    /// Create an error stating a specific string was expected
    pub const fn expected_str(range: InputRange, expected: &'static str) -> ParsingError {
        ParsingError::new(ParsingErrorInner::new(
            range,
            ParserExpectation::Str(expected),
        ))
    }

    /// Create an error with a custom message
    pub const fn custom(range: InputRange, message: &'static str) -> ParsingError {
        ParsingError::new(ParsingErrorInner::new(
            range,
            ParserExpectation::Custom(message),
        ))
    }

    /// Create an error that is not actually an error, but a breakage indicator
    ///
    /// See [`ParserExpectation::Break`]
    pub const fn just_break(loc: InputLocation) -> ParsingError {
        ParsingError::new(ParsingErrorInner::new(
            loc.range(0),
            ParserExpectation::Break,
        ))
    }
}

/// Inner content of a parsing error
#[derive(Debug)]
#[must_use]
pub struct ParsingErrorInner {
    /// Location of the error
    at: InputRange,

    /// Failed parser's expectation
    expected: ParserExpectation,
}

impl ParsingErrorInner {
    /// Create a [`ParsingError`]'s inner content
    pub const fn new(at: InputRange, expected: ParserExpectation) -> Self {
        Self { at, expected }
    }

    /// Get the error's location in the source code
    pub const fn at(&self) -> InputRange {
        self.at
    }

    /// Check if the error covers an empty range
    pub const fn is_empty(&self) -> bool {
        self.at.len == 0
    }

    /// Get the expectation of the parser that failed
    pub const fn expected(&self) -> &ParserExpectation {
        &self.expected
    }
}

/// Type of parser expectation in an error
#[derive(Debug)]
pub enum ParserExpectation {
    /// The parser expected a specific character
    Char(char),

    /// The parser expected a specific string
    Str(&'static str),

    /// Custom error message
    Custom(&'static str),

    /// This is not actually an error, but an indicator that the
    /// current parser chain should abort. This being inside an error
    /// allows easier propagation through nested parsers.
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
