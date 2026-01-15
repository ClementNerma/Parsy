use std::any::Any;

use crate::{FileId, InputLocation, InputRange, Span};

/// Input provided to a [`crate::Parser`]
#[derive(Debug, Clone, Copy)]
pub struct ParserInput<'a> {
    /// Input's current content, cut from the left when consumed by parsers
    str: &'a str,

    /// Current location of the input
    at: InputLocation,

    /// Input's original content, from which [`Self::str`] is a part of
    original: &'a str,

    /// Parser's context (see [`Self::new_with_ctx`])
    ctx: Option<fn() -> Box<dyn Any>>,
}

impl<'a> ParserInput<'a> {
    /// Create a new input for parsers
    pub const fn new(str: &'a str, file_id: FileId) -> Self {
        Self {
            str,
            at: InputLocation { file_id, offset: 0 },
            original: str,
            ctx: None,
        }
    }

    /// Create a new input for parsers, with the provided context
    ///
    /// The context can be of any type ; it can be fetched by any parser, even nested,
    /// using the [`Self::ctx`] method.
    ///
    /// For easier handling, prefer using the [`crate::helpers::get_context`] helper function.
    pub const fn new_with_ctx(str: &'a str, file_id: FileId, ctx: fn() -> Box<dyn Any>) -> Self {
        Self {
            str,
            at: InputLocation { file_id, offset: 0 },
            original: str,
            ctx: Some(ctx),
        }
    }

    /// Get the raw context from the input
    ///
    /// For easier manipulation (especially downcasting), prefer using the
    /// [`crate::helpers::get_context`] helper function.
    pub fn ctx(&self) -> Option<fn() -> Box<dyn Any>> {
        self.ctx
    }

    /// Get the current input's non-consumed content
    ///
    /// When parsers consume the input, this content is sliced from the left
    ///
    /// To get the original content, use [`Self::original`]
    pub const fn inner(&self) -> &str {
        self.str
    }

    /// Get the current location of the input
    pub const fn at(&self) -> InputLocation {
        self.at
    }

    /// Create a [`InputRange`] starting from the input's current location,
    /// with the provided length
    pub const fn range(&self, len: usize) -> InputRange {
        InputRange::new(self.at, len)
    }

    /// Get the input's current offset in its original content
    pub const fn offset(&self) -> usize {
        self.at.offset()
    }

    /// Get the parser's original content
    ///
    /// To get the non-consumed part, use [`Self::inner`]
    pub const fn original(&self) -> &'a str {
        self.original
    }

    /// Advance the input by the provided span
    pub fn advance(&mut self, from: InputRange) {
        assert_eq!(
            self.at, from.start,
            "Provided span does not start at the same position as the input"
        );

        self.at = self.at.add(from.len);
        self.str = &self.str[from.len..];
    }

    /// Consume the `len` next bytes from the input
    ///
    /// If the provided length ends up inside a character boundary, or
    /// if it exceeds the input's non-consumed content's length, a [`None`]
    /// variant will be returned instead, and nothing will be consumed
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

    /// Consume the next character from the input
    pub fn try_eat_char(&mut self) -> Option<Span<char>> {
        let char = self.str.chars().next()?;

        let ate = self.try_eat(char.len_utf8()).unwrap();

        Some(ate.forge_here(char))
    }

    /// Extract the part matching the provided [`InputRange`] from the input's original content
    pub fn extract(&self, range: InputRange) -> &str {
        &self.original[range.start.offset..range.start.offset + range.len]
    }
}
