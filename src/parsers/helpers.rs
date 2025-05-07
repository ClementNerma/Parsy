use std::collections::HashSet;

use crate::{ParserInput, ParserResult, parser::Parser};

use super::{
    combinators::{Choice, IntoChoice, IntoSilentChoice, Lookahead, Not, SilentChoice, StaticRef},
    contentless::{Empty, End, Start},
    custom::Custom,
    textuals::{
        Char, Digit, DynamicFilter, Filter, Just, Newline, OneOfChars, Whitespace, Whitespaces,
    },
    timed::{LazilyDefined, ToDefine, ToDefineShared},
};

pub const fn start() -> Start {
    Start
}

pub const fn end() -> End {
    End
}

pub const fn empty() -> Empty {
    Empty
}

pub const fn whitespace() -> Whitespace {
    Whitespace::new()
}

pub const fn whitespaces() -> Whitespaces {
    Whitespaces::new()
}

pub const fn newline() -> Newline {
    Newline
}

pub const fn char(char: char) -> Char {
    Char::new(char)
}

pub const fn one_of_chars(set: HashSet<char>) -> OneOfChars {
    OneOfChars::new(set)
}

pub const fn just(str: &'static str) -> Just {
    Just::new(str)
}

pub const fn digit(radix: u32) -> Digit {
    Digit::new(radix)
}

pub const fn filter(func: fn(char) -> bool) -> Filter {
    Filter::new(func)
}

pub const fn dynamic_filter<F: Fn(char) -> bool>(func: F) -> DynamicFilter<F> {
    DynamicFilter::new(func)
}

pub const fn choice<O, T: IntoChoice<O>>(parsers: T) -> Choice<T, O> {
    Choice::new(parsers)
}

pub const fn silent_choice<O, T: IntoSilentChoice<O>>(parsers: T) -> SilentChoice<T, O> {
    SilentChoice::new(parsers)
}

pub const fn not<T, P: Parser<T>>(parser: P) -> Not<T, P> {
    Not::new(parser)
}

pub const fn lookahead<T, P: Parser<T>>(parser: P) -> Lookahead<T, P> {
    Lookahead::new(parser)
}

pub fn to_define<T>() -> ToDefine<T> {
    ToDefine::new()
}

pub fn to_define_shared<T>() -> ToDefineShared<T> {
    ToDefineShared::new()
}

pub const fn lazily_defined<T>(
    setup: fn() -> Box<dyn Parser<T> + Send + Sync>,
) -> LazilyDefined<T> {
    LazilyDefined::new(setup)
}

pub const fn static_ref<T, P: Parser<T>>(parser: &'static P) -> StaticRef<T, P> {
    StaticRef::new(parser)
}

pub const fn custom<F: Fn(&mut ParserInput) -> ParserResult<O>, O>(func: F) -> Custom<F, O> {
    Custom::new(func)
}

pub fn recursive<T, P: Parser<T> + 'static>(decl: impl FnOnce(ToDefine<T>) -> P) -> ToDefine<T> {
    let parser = to_define::<T>();
    parser.define(decl(parser.clone()));
    parser
}

pub fn recursive_shared<T, P: Parser<T> + Send + Sync + 'static>(
    decl: impl FnOnce(ToDefineShared<T>) -> P,
) -> ToDefineShared<T> {
    let parser = to_define_shared::<T>();
    parser.define(decl(parser.clone()));
    parser
}
