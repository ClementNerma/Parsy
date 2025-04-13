pub mod arc_late_init;
pub mod atoms;
pub mod chainings;
pub mod combinators;
pub mod container;
pub mod late_init;
pub mod recursive;
pub mod simples;
pub mod textuals;

use std::collections::HashSet;

use crate::{parser::Parser, ParserInput, ParserResult};

use self::{
    arc_late_init::ArcLateInit,
    combinators::*,
    late_init::LateInit,
    recursive::{Recursive, RecursiveRef},
    simples::*,
    textuals::*,
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

pub fn recursive<T, P: Parser<T> + 'static>(
    decl: impl FnOnce(RecursiveRef<T>) -> P,
) -> Recursive<T> {
    Recursive::declarative(decl)
}

pub fn recursive_with_value<T, P: Parser<T> + 'static, R>(
    decl: impl FnOnce(RecursiveRef<T>) -> (P, R),
) -> (Recursive<T>, R) {
    Recursive::declarative_with_value(decl)
}

pub fn late_init<T>() -> LateInit<T> {
    LateInit::new()
}

pub fn arc_late_init<T>() -> ArcLateInit<T> {
    ArcLateInit::new()
}

pub fn custom<F: Fn(&mut ParserInput) -> ParserResult<O>, O>(func: F) -> Custom<F, O> {
    Custom::new(func)
}
