use crate::{ParserExpectation, ParsingError, ParsingErrorInner};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
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

impl<T: std::fmt::Debug> std::fmt::Debug for Eaten<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { at, data } = self;
        write!(f, "Eaten({at:?} => {data:?})")
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MaybeEaten<T> {
    Eaten(Eaten<T>),
    Raw(T),
}

impl<T> MaybeEaten<T> {
    pub fn data(&self) -> &T {
        match &self {
            Self::Eaten(eaten) => &eaten.data,
            Self::Raw(raw) => raw,
        }
    }

    pub fn eaten(&self) -> Option<&Eaten<T>> {
        match self {
            MaybeEaten::Eaten(eaten) => Some(eaten),
            MaybeEaten::Raw(_) => None,
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
}

impl std::fmt::Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { file_id, offset } = self;
        write!(f, "{file_id:?} offset {offset}")
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

    pub fn expected_char(self, expected: char) -> ParsingError {
        ParsingError::new(ParsingErrorInner::new(
            self,
            ParserExpectation::Char(expected),
        ))
    }

    pub fn expected_str(self, expected: &'static str) -> ParsingError {
        ParsingError::new(ParsingErrorInner::new(
            self,
            ParserExpectation::Str(expected),
        ))
    }

    pub fn custom_err(self, message: &'static str) -> ParsingError {
        ParsingError::new(ParsingErrorInner::new(
            self,
            ParserExpectation::Custom(message),
        ))
    }

    pub fn just_break(self) -> ParsingError {
        ParsingError::new(ParsingErrorInner::new(self, ParserExpectation::Break))
    }

    pub fn contains(&self, other: CodeRange) -> Result<bool, CodeRangeComparisonError> {
        match (self.start.file_id, other.start.file_id) {
            (FileId::None | FileId::Internal, _) | (_, FileId::None | FileId::Internal) => {
                Err(CodeRangeComparisonError::FileIdIsNoneOrInternal)
            }

            (FileId::Custom(_), FileId::SourceFile(_))
            | (FileId::SourceFile(_), FileId::Custom(_)) => Ok(false),

            (FileId::SourceFile(id), FileId::SourceFile(other_id))
            | (FileId::Custom(id), FileId::Custom(other_id)) => Ok(id == other_id
                && other.start.offset >= self.start.offset
                && other.start.offset + other.len <= self.start.offset + self.len),
        }
    }
}

impl std::fmt::Debug for CodeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { start, len } = self;

        write!(
            f,
            "{start:?} to {} (len = {len})",
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
    SourceFile(u64),
    Custom(u64),
}
