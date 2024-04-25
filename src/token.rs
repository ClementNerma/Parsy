use crate::{ParserExpectation, ParsingError, ParsingErrorInner};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Eaten<T> {
    pub at: CodeRange,
    pub data: T,
}

impl<T> Eaten<T> {
    pub fn ate(at: CodeRange, data: T) -> Eaten<T> {
        Eaten { at, data }
    }

    pub fn replace<U>(self, data: U) -> Eaten<U> {
        Eaten { at: self.at, data }
    }

    pub fn change_value(&mut self, data: T) {
        self.data = data;
    }

    pub fn forge_here<U>(&self, data: U) -> Eaten<U> {
        Eaten { at: self.at, data }
    }

    pub fn map<U>(self, func: impl FnOnce(T) -> U) -> Eaten<U> {
        Eaten {
            at: self.at,
            data: func(self.data),
        }
    }

    pub fn map_ref<U>(&self, func: impl FnOnce(&T) -> U) -> Eaten<U> {
        Eaten {
            at: self.at,
            data: func(&self.data),
        }
    }

    pub fn map_full<U>(self, func: impl FnOnce(Self) -> U) -> Eaten<U> {
        Eaten {
            at: self.at,
            data: func(self),
        }
    }

    pub fn combine<U>(self, other: Eaten<U>) -> Eaten<(T, U)> {
        assert_eq!(other.at.start, self.at.start.add(self.at.len));

        Eaten {
            at: CodeRange {
                start: self.at.start,
                len: self.at.len + other.at.len,
            },
            data: (self.data, other.data),
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Location {
    pub file_id: FileId,
    pub offset: usize,
}

impl Location {
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, offset: usize) -> Self {
        Self {
            file_id: self.file_id,
            offset: self.offset + offset,
        }
    }

    pub fn file_id(self) -> FileId {
        self.file_id
    }

    pub fn offset(self) -> usize {
        self.offset
    }

    pub fn range(self, len: usize) -> CodeRange {
        CodeRange::new(self, len)
    }

    pub fn compute_offset_in(
        &self,
        input: &str,
    ) -> Result<LocationInString, LocationOutOfBoundsErr> {
        if self.offset >= input.len() {
            return Err(LocationOutOfBoundsErr);
        }

        let bef = &input[..self.offset];

        let mut line_number: usize = 0;
        let mut last_line = None;

        for line in bef.split('\n') {
            last_line = Some(line.strip_suffix('\r').unwrap_or(line));
            line_number += 1;
        }

        let col = match last_line {
            None => 0,
            Some(line) => line.len(),
        };

        Ok(LocationInString {
            line: line_number.saturating_sub(1),
            col,
        })
    }

    pub fn expected_char(self, expected: char, len: usize) -> ParsingError {
        ParsingError::new(ParsingErrorInner::new(
            self,
            ParserExpectation::Char(expected),
            len,
        ))
    }

    pub fn expected_str(self, expected: &'static str, len: usize) -> ParsingError {
        ParsingError::new(ParsingErrorInner::new(
            self,
            ParserExpectation::Str(expected),
            len,
        ))
    }

    pub fn custom_err(self, message: &'static str, len: usize) -> ParsingError {
        ParsingError::new(ParsingErrorInner::new(
            self,
            ParserExpectation::Custom(message),
            len,
        ))
    }

    pub fn just_break(self) -> ParsingError {
        ParsingError::new(ParsingErrorInner::new(self, ParserExpectation::Break, 0))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LocationInString {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct LocationOutOfBoundsErr;

impl std::fmt::Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { file_id, offset } = self;
        write!(f, "offset {offset} @ {file_id:?}")
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CodeRange {
    pub start: Location,
    pub len: usize, // In bytes
}

impl CodeRange {
    pub fn new(start: Location, len: usize) -> Self {
        Self { start, len }
    }

    pub fn contains(&self, other: CodeRange) -> Result<bool, CodeRangeComparisonError> {
        match (self.start.file_id, other.start.file_id) {
            (FileId::None | FileId::Internal, _) | (_, FileId::None | FileId::Internal) => {
                Err(CodeRangeComparisonError::FileIdIsNoneOrInternal)
            }

            (FileId::Custom(_), FileId::SourceFile(_))
            | (FileId::SourceFile(_), FileId::Custom(_)) => Ok(false),

            (FileId::SourceFile(id), FileId::SourceFile(other_id)) => Ok(id == other_id
                && other.start.offset >= self.start.offset
                && other.start.offset + other.len <= self.start.offset + self.len),

            (FileId::Custom(id), FileId::Custom(other_id)) => Ok(id == other_id
                && other.start.offset >= self.start.offset
                && other.start.offset + other.len <= self.start.offset + self.len),
        }
    }
}

impl std::fmt::Debug for CodeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { start, len } = self;
        let Location { file_id, offset } = start;
        write!(
            f,
            "offset {} to {} @ {file_id:?}",
            offset,
            start.offset + len.max(&1) - 1
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CodeRangeComparisonError {
    FileIdIsNoneOrInternal,
    NotInSameFile,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FileId {
    None,
    Internal,
    SourceFile(SourceFileID),
    Custom(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SourceFileID(u64);

impl From<u64> for SourceFileID {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<SourceFileID> for u64 {
    fn from(value: SourceFileID) -> Self {
        value.0
    }
}
