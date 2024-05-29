use crate::{CodeLocation, CodeRange, Eaten, FileId};

#[derive(Debug, Clone, Copy)]
pub struct ParserInput<'a> {
    str: &'a str,
    at: CodeLocation,
    original: &'a str,
}

impl<'a> ParserInput<'a> {
    pub fn new(str: &'a str, file_id: FileId) -> Self {
        Self {
            str,
            at: CodeLocation { file_id, offset: 0 },
            original: str,
        }
    }

    pub fn inner(&self) -> &str {
        self.str
    }

    pub fn at(&self) -> CodeLocation {
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

    pub fn apply<T>(&mut self, from: &Eaten<T>) {
        assert_eq!(
            self.at, from.at.start,
            "Provided eaten content does not start at the same position as the input"
        );

        self.at = self.at.add(from.at.len);
        self.str = &self.str[from.at.len..];
    }

    pub fn try_eat(&mut self, len: usize) -> Option<Eaten<&str>> {
        if len > self.str.len() || !self.str.is_char_boundary(len) {
            return None;
        }

        let ate = Eaten {
            at: self.range(len),
            data: &self.str[..len],
        };

        self.str = &self.str[len..];
        self.at = self.at.add(len);

        Some(ate)
    }

    pub fn try_eat_char(&mut self) -> Option<Eaten<char>> {
        let char = self.str.chars().next()?;

        let ate = self.try_eat(char.len_utf8()).unwrap();

        Some(ate.replace(char))
    }

    pub fn extract(&self, range: CodeRange) -> &str {
        &self.original[range.start.offset..range.start.offset + range.len]
    }
}
