use std::any::Any;

use crate::{CodeLocation, CodeRange, FileId, Span};

#[derive(Debug, Clone, Copy)]
pub struct ParserInput<'a> {
    str: &'a str,
    at: CodeLocation,
    original: &'a str,
    ctx: Option<fn() -> Box<dyn Any>>,
}

impl<'a> ParserInput<'a> {
    pub const fn new(str: &'a str, file_id: FileId) -> Self {
        Self {
            str,
            at: CodeLocation { file_id, offset: 0 },
            original: str,
            ctx: None,
        }
    }

    pub const fn new_with_ctx(str: &'a str, file_id: FileId, ctx: fn() -> Box<dyn Any>) -> Self {
        Self {
            str,
            at: CodeLocation { file_id, offset: 0 },
            original: str,
            ctx: Some(ctx),
        }
    }

    pub fn ctx(&self) -> Option<fn() -> Box<dyn Any>> {
        self.ctx
    }

    pub const fn inner(&self) -> &str {
        self.str
    }

    pub const fn at(&self) -> CodeLocation {
        self.at
    }

    pub const fn range(&self, len: usize) -> CodeRange {
        CodeRange::new(self.at, len)
    }

    pub const fn offset(&self) -> usize {
        self.at.offset()
    }

    pub const fn original(&self) -> &'a str {
        self.original
    }

    pub fn apply<T>(&mut self, from: &Span<T>) {
        assert_eq!(
            self.at, from.at.start,
            "Provided span does not start at the same position as the input"
        );

        self.at = self.at.add(from.at.len);
        self.str = &self.str[from.at.len..];
    }

    pub fn try_eat(&mut self, len: usize) -> Option<Span<&str>> {
        if len > self.str.len() || !self.str.is_char_boundary(len) {
            return None;
        }

        let ate = Span {
            at: self.range(len),
            data: &self.str[..len],
        };

        self.str = &self.str[len..];
        self.at = self.at.add(len);

        Some(ate)
    }

    pub fn try_eat_char(&mut self) -> Option<Span<char>> {
        let char = self.str.chars().next()?;

        let ate = self.try_eat(char.len_utf8()).unwrap();

        Some(ate.replace(char))
    }

    pub fn extract(&self, range: CodeRange) -> &str {
        &self.original[range.start.offset..range.start.offset + range.len]
    }
}
