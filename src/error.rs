use std::{borrow::Cow, fmt};

use crate::{CodeRange, Eaten};

pub type PResult<T> = ::std::result::Result<Eaten<T>, ParsingError>;

#[derive(Debug)]
pub struct ParsingError {
    inner: ParsingErrorInner,
    critical: Option<Cow<'static, str>>,
}

impl ParsingError {
    pub fn new(inner: ParsingErrorInner) -> Self {
        Self {
            inner,
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

    pub fn criticalize(mut self, critical: Cow<'static, str>) -> Self {
        if self.critical.is_none() {
            self.critical = Some(critical);
        }

        self
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
    len: usize,
    expected: ParserExpectation,
}

impl ParsingErrorInner {
    pub fn new(at: CodeRange, expected: ParserExpectation) -> Self {
        Self {
            at,
            expected,
            len: 1,
        }
    }

    pub fn at(&self) -> CodeRange {
        self.at
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn expected(&self) -> &ParserExpectation {
        &self.expected
    }
}
