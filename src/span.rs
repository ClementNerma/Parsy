/// Parse content from a parser
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Span<T> {
    /// Location in the input content
    pub at: InputRange,

    /// Parsed value
    pub data: T,
}

impl<T> Span<T> {
    /// Create a span from an [`InputRange`] and a parsed value
    pub const fn ate(at: InputRange, data: T) -> Span<T> {
        Span { at, data }
    }

    /// Change this span's parsed value
    pub fn change_value(&mut self, data: T) {
        self.data = data;
    }

    /// Create a new span at the same location, but with a different parsed value
    pub const fn forge_here<U>(&self, data: U) -> Span<U> {
        Span { at: self.at, data }
    }

    /// Create a new span at the same location, but with a different parsed value
    ///
    /// The current one will be passed through the provided mapper
    pub fn map<U>(self, func: impl FnOnce(T) -> U) -> Span<U> {
        Span {
            at: self.at,
            data: func(self.data),
        }
    }

    /// Create a new span at the same location, but with a different parsed value
    ///
    /// Does not consume this span
    ///
    /// The current one will be passed through the provided mapper
    pub fn map_ref<U>(&self, func: impl FnOnce(&T) -> U) -> Span<U> {
        Span {
            at: self.at,
            data: func(&self.data),
        }
    }

    /// Create a new span at the same location, but with a different parsed value
    ///
    /// The current span will be passed through the provided mapper
    pub fn map_full<U>(self, func: impl FnOnce(Self) -> U) -> Span<U> {
        Span {
            at: self.at,
            data: func(self),
        }
    }

    /// Combine this span with a contiguous one
    ///
    /// Panics if the provided span doesn't immediately follow this one,
    /// or if the [`FileId`] is different
    pub fn combine<U>(self, other: Span<U>) -> Span<(T, U)> {
        assert_eq!(other.at.start, self.at.start.add(self.at.len));

        Span {
            at: InputRange {
                start: self.at.start,
                len: self.at.len + other.at.len,
            },
            data: (self.data, other.data),
        }
    }
}

/// Location in the source input
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputLocation {
    pub file_id: FileId,
    pub offset: usize,
}

impl InputLocation {
    /// Create a new location with the provided offset added
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub const fn add(self, offset: usize) -> Self {
        Self {
            file_id: self.file_id,
            offset: self.offset + offset,
        }
    }

    /// Get the location's file ID
    pub const fn file_id(self) -> FileId {
        self.file_id
    }

    /// Get the location's offset in the file
    pub const fn offset(self) -> usize {
        self.offset
    }

    /// Create a range starting from this location, with the provided length
    pub const fn range(self, len: usize) -> InputRange {
        InputRange::new(self, len)
    }

    /// Compute the line and column number of this location inside the provided input
    pub fn compute_coords_in(&self, input: &str) -> Result<CoordsInString, LocationOutOfBoundsErr> {
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

        Ok(CoordsInString {
            line: line_number.saturating_sub(1),
            col,
        })
    }

    /// Extract the provided input's content covered by this location
    pub fn extract_str<'a>(&self, input: &'a str) -> Result<&'a str, LocationOutOfBoundsErr> {
        if self.offset < input.len() {
            Ok(&input[self.offset..])
        } else {
            Err(LocationOutOfBoundsErr)
        }
    }
}

/// Coordinates in a string
#[derive(Debug, Clone, Copy)]
pub struct CoordsInString {
    /// Line number (starting at 0)
    pub line: usize,

    /// Column number (starting at 0)
    pub col: usize,
}

/// Indicate a [`InputRange`] is out-of-bounds of the provided input content
#[derive(Debug, Clone, Copy)]
pub struct LocationOutOfBoundsErr;

impl std::fmt::Debug for InputLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { file_id, offset } = self;
        write!(f, "offset {offset} @ {file_id:?}")
    }
}

/// Range in the source input
///
/// Combination of an [`InputLocation`] and a length
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputRange {
    /// Start of the input
    pub start: InputLocation,

    /// Length of the range, in bytes
    pub len: usize,
}

impl InputRange {
    /// Create a new range from a location and a length
    pub const fn new(start: InputLocation, len: usize) -> Self {
        Self { start, len }
    }

    /// Check if this range contains another one entirely
    pub const fn contains(&self, other: InputRange) -> Result<bool, InputRangeComparisonError> {
        match (self.start.file_id, other.start.file_id) {
            (FileId::None | FileId::Internal, _) | (_, FileId::None | FileId::Internal) => {
                Err(InputRangeComparisonError::FileIdIsNoneOrInternal)
            }

            (FileId::Custom(_), FileId::SourceFile(_))
            | (FileId::SourceFile(_), FileId::Custom(_)) => Ok(false),

            (FileId::SourceFile(id), FileId::SourceFile(other_id)) => Ok(id.const_eq(other_id)
                && other.start.offset >= self.start.offset
                && other.start.offset + other.len <= self.start.offset + self.len),

            (FileId::Custom(id), FileId::Custom(other_id)) => Ok(id == other_id
                && other.start.offset >= self.start.offset
                && other.start.offset + other.len <= self.start.offset + self.len),
        }
    }
}

impl std::fmt::Debug for InputRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { start, len } = self;
        let InputLocation { file_id, offset } = start;
        write!(
            f,
            "offset {} to {} @ {file_id:?}",
            offset,
            start.offset + len.max(&1) - 1
        )
    }
}

/// Comparison failure between two input ranges
#[derive(Debug, Clone, Copy)]
pub enum InputRangeComparisonError {
    /// Input ranges cannot be compared as the [`FileId`] is [`FileId::None`] or [`FileId::Internal`],
    /// as these may not refer to the same place
    FileIdIsNoneOrInternal,

    /// Input ranges cannot be compared as the [`FileId`] is different between the two ranges
    NotInSameFile,
}

/// ID of a source file
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FileId {
    /// No source file
    None,

    /// Internal marker, indicates the source cannot be shown
    Internal,

    /// Specific source file
    SourceFile(SourceFileID),

    /// Custom marker that doesn't correspond to a source file
    Custom(u64),
}

/// ID of a source file, to use in [`FileId::SourceFile`]
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

impl SourceFileID {
    /// Check at build time if this source file ID is equal to another
    pub const fn const_eq(self, other: Self) -> bool {
        self.0 == other.0
    }
}
