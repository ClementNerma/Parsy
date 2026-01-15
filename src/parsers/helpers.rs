//!
//! A collection of helper functions to build parsers easily
//!

use std::{any::Any, collections::HashSet};

use crate::{ParserInput, ParserResult, parser::Parser};

use super::{
    combinators::{Choice, IntoChoice, IntoSilentChoice, Lookahead, Not, SilentChoice},
    contentless::{Empty, End, Start},
    context::GetContext,
    custom::Custom,
    tails::StaticRef,
    textuals::{
        Char, Digit, DynamicFilter, Filter, Just, Newline, OneOfChars, Whitespace, Whitespaces,
    },
    timed::{LazilyDefined, ToDefine, ToDefineShared},
};

/// Match the start of the input (doesn't consume the input)
///
/// Fails if some part of the input has already been consumed
pub const fn start() -> Start {
    Start
}

/// Match the end of the input (doesn't consume)
///
/// Fails if the input hasn't been consumed entirely yet
pub const fn end() -> End {
    End
}

/// Always match, doesn't consume
pub const fn empty() -> Empty {
    Empty
}

/// Match exactly one UTF-8 whitespace
pub const fn whitespace() -> Whitespace {
    Whitespace::new()
}

/// Match any number of UTF-8 whitespaces
///
/// Better performance than `whitespace().repeated()`
pub const fn whitespaces() -> Whitespaces {
    Whitespaces::new()
}

/// Match a newline character
pub const fn newline() -> Newline {
    Newline
}

/// Match the provided character
pub const fn char(char: char) -> Char {
    Char::new(char)
}

/// Match any of the provided characters
pub const fn one_of_chars(set: HashSet<char>) -> OneOfChars {
    OneOfChars::new(set)
}

/// Match exactly the provided string
pub const fn just(str: &'static str) -> Just {
    Just::new(str)
}

/// Match any digit in the provided base
pub const fn digit(radix: u32) -> Digit {
    Digit::new(radix)
}

/// Match any character that passes the provided filter
pub const fn filter(func: fn(char) -> bool) -> Filter {
    Filter::new(func)
}

/// Equivalent to [`filter`] but allows using a non-pointer function
pub const fn dynamic_filter<F: Fn(char) -> bool>(func: F) -> DynamicFilter<F> {
    DynamicFilter::new(func)
}

/// Create a parser that returns the value of the first parser to succeed in a set
///
/// Parsers are in the order they were provided during initialization
pub const fn choice<O, T: IntoChoice<O>>(parsers: T) -> Choice<T, O> {
    Choice::new(parsers)
}

/// Equivalent to [`choice`], but does ignores the parse value and return a `()` instead
///
/// Unlike [`choice`], allows using parser that evaluated to different parsed types
pub const fn silent_choice<O, T: IntoSilentChoice<O>>(parsers: T) -> SilentChoice<T, O> {
    SilentChoice::new(parsers)
}

/// Succeed if and only if the provided parser fails, doesn't consume the input
pub const fn not<T, P: Parser<T>>(parser: P) -> Not<T, P> {
    Not::new(parser)
}

/// Succeed if and only if the provided parser suceeds, but doesn't consume the input
pub const fn lookahead<T, P: Parser<T>>(parser: P) -> Lookahead<T, P> {
    Lookahead::new(parser)
}

/// Create a parser to be defined later, non-thread safe
///
/// For a thread-safe variant, see [`to_define_shared`].
///
/// To define a parser recursively, see [`recursive`]
pub fn to_define<T>() -> ToDefine<T> {
    ToDefine::new()
}

/// Thread-safe equivalent of [`to_define`]
pub fn to_define_shared<T>() -> ToDefineShared<T> {
    ToDefineShared::new()
}

/// Define a parser that can parse content through itself, non-thread safe
///
/// For a a thread-safe variant, see [`recursive_shared`]
///
/// Uses [`to_define`] under the hood
pub fn recursive<T, P: Parser<T> + 'static>(decl: impl FnOnce(ToDefine<T>) -> P) -> ToDefine<T> {
    let parser = to_define::<T>();
    parser.define(decl(parser.clone()));
    parser
}

/// Thread-safe equivalent of [`recursive`]
pub fn recursive_shared<T, P: Parser<T> + Send + Sync + 'static>(
    decl: impl FnOnce(ToDefineShared<T>) -> P,
) -> ToDefineShared<T> {
    let parser = to_define_shared::<T>();
    parser.define(decl(parser.clone()));
    parser
}

/// Define a parser that will be evaluated only once and shared among all threads afterwards
///
/// Only accepts a dynamic parser wrapped in a [`Box`], see [`crate::ParserNonConstUtils::erase_type`]
pub const fn lazily_define<T>(setup: fn() -> Box<dyn Parser<T> + Send + Sync>) -> LazilyDefined<T> {
    LazilyDefined::new(setup)
}

/// Use a parser from a non-movable type (e;g. [`LazilyDefined`])
pub const fn static_ref<T, P: Parser<T>>(parser: &'static P) -> StaticRef<T, P> {
    StaticRef::new(parser)
}

/// Create a parser using a custom parsing function
pub const fn custom<F: Fn(&mut ParserInput) -> ParserResult<O>, O>(func: F) -> Custom<F, O> {
    Custom::new(func)
}

/// Fetch the parser's context (see [`ParserInput::new_with_ctx`]) with the provided type
///
/// If the context is missing or has a different type than the provided one, a critical error
/// will be emitted
pub const fn get_context<C: Any>() -> GetContext<C> {
    GetContext::new()
}
