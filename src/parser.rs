use crate::{
    chainings::*, combinators::*, container::*, textuals::*, FileId, ParserInput, ParserResult,
    ParsingError,
};

/// A parser takes an input and tries to consume the upcoming character(s) and transform it
/// into a value.
pub trait Parser<T> {
    /// Inner parsing function, to implement
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<T>;

    /// Parse an input with the current parser
    ///
    /// The input position will advance if the parsing is successful,
    /// and will not advance if the parsing fails
    fn parse(&self, input: &mut ParserInput) -> ParserResult<T> {
        // "Clone" (copy) 'input'
        let mut input_copy = *input;

        let result = self.parse_inner(&mut input_copy);

        // Only apply changes to input (cursor advance) if the parsing was successful
        // Otherwise, keep the original intact (this is equivalent to rollbacking in case of error)
        result.inspect(|span| input.apply(span))
    }

    /// Parse a string
    ///
    /// Will use [`FileId::None`] as the source
    fn parse_str(&self, str: &str) -> ParserResult<T> {
        self.parse_str_with_file_id(str, FileId::None)
    }

    /// Parse a string as a file
    ///
    /// Will use the provided file ID
    fn parse_str_with_file_id(&self, str: &str, file_id: FileId) -> ParserResult<T> {
        self.parse(&mut ParserInput::new(str, file_id))
    }

    /// Chain this parser with another, getting both parsers' results combined
    fn then<U, P: Parser<U>>(self, other: P) -> Then<T, Self, U, P>
    where
        Self: Sized,
    {
        Then::new(self, other)
    }

    /// Chain this parser with another but discard the latter's parsed value
    fn then_ignore<U, P: Parser<U>>(self, other: P) -> ThenIgnore<T, Self, U, P>
    where
        Self: Sized,
    {
        ThenIgnore::new(self, other)
    }

    /// Chain this parser with another but discard the former's parsed value
    fn ignore_then<U, P: Parser<U>>(self, other: P) -> IgnoreThen<T, Self, U, P>
    where
        Self: Sized,
    {
        IgnoreThen::new(self, other)
    }

    /// Only match if this parser succeeds and the provided parser succeeds as well
    ///
    /// The second parser will not make the input's position advance
    fn followed_by<U, P: Parser<U>>(self, other: P) -> FollowedBy<T, Self, U, P>
    where
        Self: Sized,
    {
        FollowedBy::new(self, other)
    }

    /// Only match if this parser succeeds and the provided parser doesn't
    fn not_followed_by<U, P: Parser<U>>(self, other: P) -> NotFollowedBy<T, Self, U, P>
    where
        Self: Sized,
    {
        NotFollowedBy::new(self, other)
    }

    /// Parse as many times as possible, until the parser eventually fails
    ///
    /// This will not allocate. To get the results directly in a [`Vec`], see [`Parser::repeated_into_vec`]
    fn repeated(self) -> Repeated<T, Self, NoAllocContainer>
    where
        Self: Sized,
    {
        Repeated::new(self)
    }

    /// Parse as many times as possible, until the parser eventually fails
    ///
    /// All the parsed values will be put in a [`Vec`].
    /// To use another container, see [`Parser::repeated_into_container`]
    fn repeated_into_vec(self) -> Repeated<T, Self, Vec<T>>
    where
        Self: Sized,
    {
        Repeated::new(self)
    }

    /// Parse as many times as possible, until the parser eventually fails
    ///
    /// All the parsed values will be forwarded to the provided [`Container`] type.
    /// The container will then be returned.
    fn repeated_into_container<C: Container<T>>(self) -> Repeated<T, Self, C>
    where
        Self: Sized,
    {
        Repeated::new(self)
    }

    /// Try to parse
    ///
    /// * In case of success, the parser will succeed and return the parsed value wrapped in a [`Some`]
    /// * In case of failure, the parser will succeed and return a [`None`]
    fn or_not(self) -> OrNot<T, Self>
    where
        Self: Sized,
    {
        OrNot::new(self)
    }

    /// Map the parsed value using a function
    ///
    /// Aking to [`Option::map`]
    fn map<U, F: Fn(T) -> U>(self, mapper: F) -> Map<T, Self, U, F>
    where
        Self: Sized,
    {
        Map::new(self, mapper)
    }

    /// Get the input string matched by the parser and map it using a function
    fn map_str<U, F: Fn(&str) -> U>(self, mapper: F) -> MapStr<T, Self, U, F>
    where
        Self: Sized,
    {
        MapStr::new(self, mapper)
    }

    /// Try to map the parsed value using a function
    fn try_map<U, F: Fn(T) -> Option<U>>(self, mapper: F) -> TryMap<T, Self, U, F>
    where
        Self: Sized,
    {
        TryMap::new(self, mapper)
    }

    /// Transform and validate the parsed value using the provided function
    ///
    /// If you want to only return an error message, see [`Parser::and_then_or_str`]
    fn and_then<U, F: Fn(T) -> Result<U, ParsingError>>(self, mapper: F) -> AndThen<T, Self, U, F>
    where
        Self: Sized,
    {
        AndThen::new(self, mapper)
    }

    /// Transform and validate the parsed value using the provided function
    fn and_then_or_str<U, F: Fn(T) -> Result<U, &'static str>>(
        self,
        mapper: F,
    ) -> AndThenOrStrErr<T, Self, U, F>
    where
        Self: Sized,
    {
        AndThenOrStrErr::new(self, mapper)
    }

    /// Transform and validate the parsed value using the provided function
    /// Failures are [critical](`Parser::critical`)
    fn and_then_or_critical<U, F: Fn(T) -> Result<U, String>>(
        self,
        mapper: F,
    ) -> AndThenOrCritical<T, Self, U, F>
    where
        Self: Sized,
    {
        AndThenOrCritical::new(self, mapper)
    }

    /// Wrap the parsed value in a [`Spanned`]
    fn spanned(self) -> Spanned<T, Self>
    where
        Self: Sized,
    {
        Spanned::new(self)
    }

    /// Collect the parsed value using the provided iterator type
    fn collect<C>(self) -> Map<T, Self, C, fn(T) -> C>
    where
        Self: Sized,
        T: IntoIterator,
        C: FromIterator<T::Item>,
    {
        self.map(C::from_iter)
    }

    /// Collect the input string matched by the parser
    fn collect_string(self) -> StringCollected<T, Self>
    where
        Self: Sized,
    {
        StringCollected::new(self)
    }

    /// Provide an atomic error if the parser fails
    ///
    /// Atomic errors are the smallest possible error types,
    /// every error nested below their level is discarded
    fn atomic_err(self, message: &'static str) -> AtomicErr<T, Self>
    where
        Self: Sized,
    {
        AtomicErr::new(self, message)
    }

    /// Mark the parser as critical
    ///
    /// In case of failure, the whole chain of parsing will fail with the provided message
    fn critical(self, message: &'static str) -> Critical<T, Self>
    where
        Self: Sized,
    {
        Critical::new(self, Some(message))
    }

    /// Mark the parser as critical
    ///
    /// In case of failure, the whole chain of parsing will fail with a default message
    fn critical_auto_msg(self) -> Critical<T, Self>
    where
        Self: Sized,
    {
        Critical::new(self, None)
    }

    /// Make the parser silent
    ///
    /// The parsed value will be `()`. Akin to using `.map(|_| ())` on the parser.
    fn silent(self) -> Silenced<T, Self>
    where
        Self: Sized,
    {
        Silenced::new(self)
    }

    /// Require the parser to be preceded by and followed by the provided padding
    ///
    /// The padding parser's values are discarded
    fn padded_by<P, PP: Parser<P>>(self, padding: PP) -> Padded<T, Self, P, PP>
    where
        Self: Sized,
    {
        Padded::new(self, padding)
    }

    /// Allow the parser to be surrounded by whitespaces (not newlines)
    fn line_padded(self) -> LinePadded<T, Self>
    where
        Self: Sized,
    {
        LinePadded::line_padded(self)
    }

    /// Require the parser to be preceded by and followed by the provided parsers
    ///
    /// The parsers' values are discarded
    fn delimited_by<L, LP: Parser<L>, R, RP: Parser<R>>(
        self,
        left: LP,
        right: RP,
    ) -> DelimitedBy<L, LP, T, Self, R, RP>
    where
        Self: Sized,
    {
        DelimitedBy::new(left, self, right)
    }

    /// Repeat the parser with the required provided separator between each repetition
    ///
    /// If you want to collect the results, see [`Parser::separated_by_into_vec`].
    fn separated_by<S, P: Parser<S>>(self, sep: P) -> SeparatedBy<T, Self, S, P, NoAllocContainer>
    where
        Self: Sized,
    {
        SeparatedBy::new(self, sep)
    }

    /// Repeat the parser with the required provided separator between each repetition
    ///
    /// All results are collected into a [`Vec`].
    /// To use a custom container, see [`Parser::separated_by_into_container`]
    fn separated_by_into_vec<S, P: Parser<S>>(self, sep: P) -> SeparatedBy<T, Self, S, P, Vec<T>>
    where
        Self: Sized,
    {
        SeparatedBy::new(self, sep)
    }

    /// Repeat the parser with the required provided separator between each repetition
    ///
    /// All results are forwarded to the provided [`Container`] type, which is then returned.
    fn separated_by_into_container<C: Container<T>, S, P: Parser<S>>(
        self,
        sep: P,
    ) -> SeparatedBy<T, Self, S, P, C>
    where
        Self: Sized,
    {
        SeparatedBy::new(self, sep)
    }

    /// Flatten the parser
    ///
    /// Requires the parser to return a nested iterator.
    /// The values are discarded. To collect them, see [`Parser::flatten_into_vec`]
    fn flatten<U, S>(self) -> Flattened<U, S, T, Self, NoAllocContainer>
    where
        Self: Sized,
        T: IntoIterator<Item = S>,
        S: IntoIterator<Item = U>,
    {
        Flattened::new(self)
    }

    /// Flatten the parser
    /// Requires the parser to return a nested iterator.
    ///
    /// The values are collected into a [`Vec`].
    /// To use a custom container, see [`Parser::flatten_into_container`]
    fn flatten_into_vec<U, S>(self) -> Flattened<U, S, T, Self, Vec<U>>
    where
        Self: Sized,
        T: IntoIterator<Item = S>,
        S: IntoIterator<Item = U>,
    {
        Flattened::new(self)
    }

    /// Flatten the parser
    /// Requires the parser to return a nested iterator.
    ///
    /// All results are forwarded to the provided [`Container`] type, which is then returned.
    fn flatten_into_container<U, S, C: Container<U>>(self) -> Flattened<U, S, T, Self, C>
    where
        Self: Sized,
        T: IntoIterator<Item = S>,
        S: IntoIterator<Item = U>,
    {
        Flattened::new(self)
    }

    /// Discard the parsed value and replace it with a fixed value
    fn to<U: Copy>(self, data: U) -> To<T, Self, U>
    where
        Self: Sized,
    {
        To::new(self, data)
    }

    /// Require the parser to match the entire input
    fn full(self) -> Full<T, Self>
    where
        Self: Sized,
    {
        Full::new(self)
    }

    /// Allow the parser to fallback to another parser in case of failure
    ///
    /// If you have multiple choices, see [`crate::parsers::choice`]
    fn or<P: Parser<T>>(self, other: P) -> Choice<(Self, P), T>
    where
        Self: Sized,
    {
        Choice::<(Self, P), T>::new((self, other))
    }

    /// Validate the parsed value with a predicate
    fn validate<F: Fn(&T) -> bool>(self, validator: F) -> Validate<T, Self, F>
    where
        Self: Sized,
    {
        Validate::new(self, validator)
    }

    /// Debug the input and output values of the parser using the provided debugger
    fn debug<F: for<'a, 'b> Fn(DebugType<'a, 'b, T>)>(self, debugger: F) -> Debugging<T, Self, F>
    where
        Self: Sized,
    {
        Debugging::new(self, debugger)
    }
}
