use std::borrow::Cow;

use annotate_snippets::{Level, Renderer, Snippet};

use crate::{InputRange, ParserExpectation, ParsingError};

/// An error report, that will display a nice, human-readable extract of the
/// input content along with the error message at the location it happened.
///
/// Uses [`annotate_snippets`] under the hood.
#[derive(Clone)]
pub struct ErrorReport<'a, 'b, 'c> {
    /// Source content the parser originally used
    source: &'a str,

    /// Path to the source
    source_path: &'b str,

    /// Error message
    err_msg: Cow<'c, str>,

    /// Offset in the source content
    offset: usize,

    /// Number of input bytes covered by the error
    len: usize,
}

impl<'a, 'b, 'c> ErrorReport<'a, 'b, 'c> {
    /// Create an error report from a [`ParsingError`]
    pub fn parsing_error(
        source: &'a str,
        source_path: &'b str,
        parsing_err: &'c ParsingError,
    ) -> Self {
        Self {
            source,
            source_path,
            err_msg: match parsing_err.critical_message() {
                Some(nature) => Cow::Borrowed(nature),
                None => match parsing_err.inner().expected() {
                    ParserExpectation::Char(c) => Cow::Owned(format!("expected char '{c}'")),
                    ParserExpectation::Str(str) => Cow::Owned(format!("expected '{str}'")),
                    ParserExpectation::Custom(msg) => Cow::Borrowed(msg),
                    ParserExpectation::Break => Cow::Borrowed("got break (should not be possible)"),
                },
            },
            offset: parsing_err.inner().at().start.offset,
            len: parsing_err.inner().at().len,
        }
    }

    /// Create an error report from a [`CodeRange`]
    pub const fn with_range(
        source: &'a str,
        source_path: &'b str,
        at: InputRange,
        msg: &'c str,
    ) -> Self {
        Self {
            source,
            source_path,
            offset: at.start.offset,
            len: at.len,
            err_msg: Cow::Borrowed(msg),
        }
    }
}

impl std::fmt::Display for ErrorReport<'_, '_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ErrorReport {
            source,
            source_path,
            err_msg,
            offset,
            len,
        } = self;

        let line = source[..*offset].chars().filter(|&c| c == '\n').count();

        let extract_start_line = line.saturating_sub(2) + 1;

        let extract_start_offset = if extract_start_line == 1 {
            0
        } else {
            let mut line_counter = 1;
            let mut shift = 0;

            source[..*offset]
                .chars()
                .find_map(|c| {
                    if c == '\n' {
                        line_counter += 1;
                    }

                    shift += c.len_utf8();

                    if line_counter == extract_start_line {
                        Some(shift)
                    } else {
                        None
                    }
                })
                .unwrap_or(0)
        };

        let mut line_counter = 0;

        let afterwards = &source[offset + len..].chars().position(|c| {
            if c == '\n' {
                line_counter += 1;
            }

            line_counter == 2
        });

        let extract_end = match afterwards {
            Some(pos) => offset + len + pos + 1,
            None => source.len(),
        };

        // NOTE: we add a space at the end of the error's line
        // as the reporting library doesn't support displaying
        // offsets after a line's last character
        let extract = format!("{} ", &source[extract_start_offset..extract_end]);

        // Same thing for the error range source
        let range_chars_len = if offset + (*len).max(1) == source.len() + 1 {
            1
        } else {
            source[*offset..*offset + (*len).max(1)].len()
        };

        let snippet = Level::Error.title("Parsing failed").snippet(
            Snippet::source(&extract)
                .line_start(extract_start_line)
                .origin(source_path)
                .fold(false)
                .annotation(
                    Level::Error
                        .span(
                            offset - extract_start_offset
                                ..offset - extract_start_offset + range_chars_len,
                        )
                        .label(err_msg.as_ref()),
                ),
        );

        let renderer = Renderer::styled();
        let rendered = renderer.render(snippet);

        write!(f, "{rendered}")
    }
}
