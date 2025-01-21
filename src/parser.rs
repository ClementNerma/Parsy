use crate::{
    chainings::*, combinators::*, container::*, textuals::*, FileId, ParserInput, ParserResult,
    ParsingError,
};

pub trait Parser<T> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<T>;

    fn parse(&self, input: &mut ParserInput) -> ParserResult<T> {
        // "Clone" (copy) 'input'
        let mut input_copy = *input;

        let result = self.parse_inner(&mut input_copy);

        // Only apply changes to input (cursor advance) if the parsing was successful
        // Otherwise, keep the original intact (this is equivalent to rollbacking in case of error)
        result.inspect(|span| input.apply(span))
    }

    fn parse_str(&self, str: &str) -> ParserResult<T> {
        self.parse_str_as_file(str, FileId::None)
    }

    fn parse_str_as_file(&self, str: &str, file_id: FileId) -> ParserResult<T> {
        self.parse(&mut ParserInput::new(str, file_id))
    }

    fn then<U, P: Parser<U>>(self, other: P) -> Then<T, Self, U, P>
    where
        Self: Sized,
    {
        Then::new(self, other)
    }

    fn then_ignore<U, P: Parser<U>>(self, other: P) -> ThenIgnore<T, Self, U, P>
    where
        Self: Sized,
    {
        ThenIgnore::new(self, other)
    }

    fn ignore_then<U, P: Parser<U>>(self, other: P) -> IgnoreThen<T, Self, U, P>
    where
        Self: Sized,
    {
        IgnoreThen::new(self, other)
    }

    fn followed_by<U, P: Parser<U>>(self, other: P) -> FollowedBy<T, Self, U, P>
    where
        Self: Sized,
    {
        FollowedBy::new(self, other)
    }

    fn not_followed_by<U, P: Parser<U>>(self, other: P) -> NotFollowedBy<T, Self, U, P>
    where
        Self: Sized,
    {
        NotFollowedBy::new(self, other)
    }

    fn repeated(self) -> Repeated<T, Self, NoAllocContainer>
    where
        Self: Sized,
    {
        Repeated::new(self)
    }

    fn repeated_vec(self) -> Repeated<T, Self, Vec<T>>
    where
        Self: Sized,
    {
        Repeated::new(self)
    }

    fn repeated_custom<C: Container<T>>(self) -> Repeated<T, Self, C>
    where
        Self: Sized,
    {
        Repeated::new(self)
    }

    fn or_not(self) -> OrNot<T, Self>
    where
        Self: Sized,
    {
        OrNot::new(self)
    }

    fn map<U, F: Fn(T) -> U + Clone>(self, mapper: F) -> Map<T, Self, U, F>
    where
        Self: Sized,
    {
        Map::new(self, mapper)
    }

    fn map_str<U, F: Fn(&str) -> U + Clone>(self, mapper: F) -> MapStr<T, Self, U, F>
    where
        Self: Sized,
    {
        MapStr::new(self, mapper)
    }

    fn and_then<U, F: Fn(T) -> Result<U, ParsingError>>(self, mapper: F) -> AndThen<T, Self, U, F>
    where
        Self: Sized,
    {
        AndThen::new(self, mapper)
    }

    fn and_then_or_str_err<U, F: Fn(T) -> Result<U, String>>(
        self,
        mapper: F,
    ) -> AndThenOrStrErr<T, Self, U, F>
    where
        Self: Sized,
    {
        AndThenOrStrErr::new(self, mapper)
    }

    fn spanned(self) -> Spanned<T, Self>
    where
        Self: Sized,
    {
        Spanned::new(self)
    }

    fn collect<C>(self) -> Map<T, Self, C, fn(T) -> C>
    where
        Self: Sized,
        T: IntoIterator,
        C: FromIterator<T::Item>,
    {
        self.map(C::from_iter)
    }

    fn collect_string(self) -> StringCollected<T, Self>
    where
        Self: Sized,
    {
        StringCollected::new(self)
    }

    fn atomic_err(self, message: &'static str) -> AtomicErr<T, Self>
    where
        Self: Sized,
    {
        AtomicErr::new(self, message)
    }

    fn critical(self, message: &'static str) -> Critical<T, Self>
    where
        Self: Sized,
    {
        Critical::new(self, Some(message))
    }

    fn critical_with_no_message(self) -> Critical<T, Self>
    where
        Self: Sized,
    {
        Critical::new(self, None)
    }

    fn silent(self) -> Silenced<T, Self>
    where
        Self: Sized,
    {
        Silenced::new(self)
    }

    fn padded_by<P, PP: Parser<P>>(self, padding: PP) -> Padded<T, Self, P, PP>
    where
        Self: Sized,
    {
        Padded::new(self, padding)
    }

    fn line_padded(self) -> LinePadded<T, Self>
    where
        Self: Sized,
    {
        LinePadded::line_padded(self)
    }

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

    fn separated_by<S, P: Parser<S>>(self, sep: P) -> SeparatedBy<T, Self, S, P, Vec<T>>
    where
        Self: Sized,
    {
        SeparatedBy::new(self, sep)
    }

    fn separated_by_custom<S, P: Parser<S>, C: Container<T>>(
        self,
        sep: P,
    ) -> SeparatedBy<T, Self, S, P, C>
    where
        Self: Sized,
    {
        SeparatedBy::new(self, sep)
    }

    fn flatten<U, S>(self) -> Flattened<U, S, T, Self, NoAllocContainer>
    where
        Self: Sized,
        T: IntoIterator<Item = S>,
        S: IntoIterator<Item = U>,
    {
        Flattened::new(self)
    }

    fn flatten_vec<U, S>(self) -> Flattened<U, S, T, Self, Vec<U>>
    where
        Self: Sized,
        T: IntoIterator<Item = S>,
        S: IntoIterator<Item = U>,
    {
        Flattened::new(self)
    }

    fn flatten_custom<U, S, C: Container<U>>(self) -> Flattened<U, S, T, Self, C>
    where
        Self: Sized,
        T: IntoIterator<Item = S>,
        S: IntoIterator<Item = U>,
    {
        Flattened::new(self)
    }

    fn to<U: Copy>(self, data: U) -> To<T, Self, U>
    where
        Self: Sized,
    {
        To::new(self, data)
    }

    fn full(self) -> Full<T, Self>
    where
        Self: Sized,
    {
        Full::new(self)
    }

    fn or<P: Parser<T>>(self, other: P) -> Choice<(Self, P), T>
    where
        Self: Sized,
    {
        Choice::<(Self, P), T>::new((self, other))
    }

    fn validate<F: Fn(&T) -> bool>(self, validator: F) -> Validate<T, Self, F>
    where
        Self: Sized,
    {
        Validate::new(self, validator)
    }

    fn debug<F: for<'a, 'b> Fn(DebugType<'a, 'b, T>) + Clone>(
        self,
        debugger: F,
    ) -> Debugging<T, Self, F>
    where
        Self: Sized,
    {
        Debugging::new(self, debugger)
    }
}
