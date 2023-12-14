use crate::{CodeRange, Eaten, FileId, Location};

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

    pub fn try_eat(&mut self, bytes: usize) -> Option<Eaten<&str>> {
        if bytes > self.str.len() {
            return None;
        }

        let str = &self.str[..bytes];

        Some(self.eat(str.len(), str))
    }

    pub fn extract(&self, range: CodeRange) -> &str {
        &self.original[range.start.offset..range.start.offset + range.len]
    }
}
