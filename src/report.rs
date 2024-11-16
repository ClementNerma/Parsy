use annotate_snippets::{Level, Renderer, Snippet};

use crate::{ParserExpectation, ParsingError};

pub fn pretty_error_report(source: &str, source_path: &str, err: ParsingError) -> String {
    let offset = err.inner().at().start.offset;
    let len = err.inner().at().len;

    let msg = match err.critical_message() {
        Some(nature) => nature.to_string(),
        None => match err.inner().expected() {
            ParserExpectation::Char(c) => format!("expected char '{c}'"),
            ParserExpectation::Str(str) => format!("expected '{str}'"),
            ParserExpectation::Custom(msg) => msg.to_string(),
            ParserExpectation::Break => unreachable!(),
        },
    };

    let line = source[..offset].chars().filter(|&c| c == '\n').count();

    let extract_start_line = line.saturating_sub(2) + 1;

    let extract_start_offset = if extract_start_line == 1 {
        0
    } else {
        let mut line_counter = 1;
        let mut shift = 0;

        source[..offset]
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
    let range_chars_len = if offset + len.max(1) == source.len() + 1 {
        1
    } else {
        source[offset..offset + len.max(1)].len()
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
                    .label(&msg),
            ),
    );

    format!("{}", Renderer::styled().render(snippet))
}
