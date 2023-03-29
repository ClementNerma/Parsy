use std::borrow::Cow;

use crate::{CodeRange, Eaten, FileId, Location};

pub type PResult<T> = ::std::result::Result<Eaten<T>, ParsingError>;

#[derive(Debug)]
pub struct ParsingError {
    inner: ParsingErrorInner,
    label: Option<Cow<'static, str>>,
    critical: Option<CriticalErrorNature>,
}

impl ParsingError {
    pub fn new(inner: ParsingErrorInner) -> Self {
        Self {
            inner,
            label: None,
            critical: None,
        }
    }

    pub fn inner(&self) -> &ParsingErrorInner {
        &self.inner
    }

    pub fn into_inner(self) -> ParsingErrorInner {
        self.inner
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn critical(&self) -> Option<&CriticalErrorNature> {
        self.critical.as_ref()
    }

    pub fn is_critical(&self) -> bool {
        self.critical.is_some()
    }

    pub fn labellize(mut self, label: impl Into<Cow<'static, str>>) -> Self {
        if self.label.is_none() {
            self.label = Some(label.into());
        }

        self
    }

    pub fn criticalize(mut self, critical: CriticalErrorNature) -> Self {
        if self.critical.is_none() {
            self.critical = Some(critical);
        }

        self
    }
}

#[derive(Debug)]
pub enum ParserExpectation {
    Char(char),
    Str(Cow<'static, str>),
    Custom(Cow<'static, str>),
    Break,
}

#[derive(Debug)]
pub enum CriticalErrorNature {
    Direct(CriticalErrorMsgContent),
    UnexpectedEndOfInput(CriticalErrorMsgContent),
}

#[derive(Debug)]
pub enum CriticalErrorMsgContent {
    Inherit,
    Custom(Cow<'static, str>),
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

    pub fn expected(&self) -> &ParserExpectation {
        &self.expected
    }
}

#[derive(Debug, Clone)]
pub struct ParserInput<'a> {
    str: &'a str,
    at: Location,
    original: &'a str,
}

impl<'a> ParserInput<'a> {
    pub fn new(str: &'a str, file_id: FileId) -> Self {
        Self {
            str,
            at: Location { file_id, offset: 0 },
            original: str,
        }
    }

    pub fn inner(&self) -> &str {
        self.str
    }

    pub fn at(&self) -> Location {
        self.at
    }

    pub fn range(&self, len: usize) -> CodeRange {
        CodeRange::new(self.at, len)
    }

    pub fn offset(&self) -> usize {
        self.at.offset()
    }

    pub fn original(&self) -> &'a str {
        self.original
    }

    pub fn mirror_from(&mut self, other: &ParserInput<'a>) {
        self.str = other.str;
        self.at = other.at;
    }

    pub fn apply<T>(&mut self, from: &Eaten<T>) {
        // if cfg!(debug_assertions) {
        assert_eq!(
            self.at, from.at.start,
            "Provided eaten content does not start at the same position as the input"
        );
        // }

        self.at = self.at.add(from.at.len);
        self.str = &self.str[from.at.len..];
    }

    fn eat<T>(&mut self, len: usize, data: T) -> Eaten<T> {
        let ate = Eaten {
            at: self.range(len),
            data,
        };

        self.str = &self.str[len..];
        self.at = self.at.add(len);

        ate
    }

    pub fn try_eat_char(&mut self) -> Option<Eaten<char>> {
        let char = self.str.chars().next()?;

        Some(self.eat(char.len_utf8(), char))
    }

    pub fn try_eat(&mut self, len: usize) -> Option<Eaten<&str>> {
        let count = self.str.chars().take(len).map(char::len_utf8).sum();
        let str = &self.str[0..count];

        if str.len() < len {
            return None;
        }

        Some(self.eat(str.as_bytes().len(), str))
    }

    pub fn eat_at_most(&mut self, len: usize) -> Eaten<&str> {
        let count = self.str.chars().take(len).map(char::len_utf8).sum();
        let str = &self.str[0..count];

        self.eat(str.as_bytes().len(), str)
    }

    pub fn eat_exact(&mut self, len: usize) -> Result<Eaten<&str>, usize> {
        let count = self.str.chars().take(len).map(char::len_utf8).sum();

        if count < len {
            return Err(count);
        }

        let str = &self.str[0..count];

        Ok(self.eat(str.as_bytes().len(), str))
    }

    pub fn extract(&self, range: CodeRange) -> &str {
        &self.original[range.start.offset..range.start.offset + range.len]
    }
}
