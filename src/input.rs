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
