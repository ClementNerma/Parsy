use std::collections::HashSet;

use crate::{ParserInput, ParserResult, parser::Parser};

use super::{
    combinators::{Choice, IntoChoice, IntoSilentChoice, Lookahead, Not, SilentChoice},
    contentless::{Empty, End, Start},
    custom::Custom,
    textuals::{Char, Filter, Just, Newline, OneOfChars, Whitespace, Whitespaces},
    timed::{ToDefine, ToDefineShared},
};

pub fn start() -> Start {
    Start
}

pub fn end() -> End {
    End
}

pub fn empty() -> Empty {
    Empty
}

pub fn whitespace() -> Whitespace {
    Whitespace::default()
}

pub fn whitespaces() -> Whitespaces {
    Whitespaces::default()
}

pub fn newline() -> Newline {
    Newline
}

pub fn char(char: char) -> Char {
    Char::new(char)
}

pub fn one_of_chars(set: HashSet<char>) -> OneOfChars {
    OneOfChars::new(set)
}

pub fn just(str: &'static str) -> Just {
    Just::new(str)
}

pub fn filter<F: Fn(char) -> bool>(func: F) -> Filter<F> {
    Filter::new(func)
}

pub fn choice<O, T: IntoChoice<O>>(parsers: T) -> Choice<T, O> {
    Choice::new(parsers)
}

pub fn silent_choice<O, T: IntoSilentChoice<O>>(parsers: T) -> SilentChoice<T, O> {
    SilentChoice::new(parsers)
}

pub fn not<T, P: Parser<T>>(parser: P) -> Not<T, P> {
    Not::new(parser)
}

pub fn lookahead<T, P: Parser<T>>(parser: P) -> Lookahead<T, P> {
    Lookahead::new(parser)
}

pub fn to_define<T>() -> ToDefine<T> {
    ToDefine::new()
}

pub fn to_define_shared<T>() -> ToDefineShared<T> {
    ToDefineShared::new()
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

pub fn custom<F: Fn(&mut ParserInput) -> ParserResult<O>, O>(func: F) -> Custom<F, O> {
    Custom::new(func)
}
