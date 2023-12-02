pub mod atoms;
pub mod chainings;
pub mod combinators;
pub mod container;
pub mod late;
pub mod recursive;
pub mod simples;
pub mod textuals;

use crate::{parser::Parser, PResult, ParserInput};
use combinators::*;
use simples::*;
use textuals::*;

use self::late::Late;
use self::recursive::{Recursive, RecursiveRef};

pub fn start() -> Start {
    Start::new()
}

pub fn end() -> End {
    End::new()
}

pub fn empty() -> Empty {
    Empty::new()
}

pub fn whitespaces() -> Whitespaces {
    Whitespaces::new()
}

pub fn char(char: char) -> Char {
    Char::new(char)
}

pub fn just(str: &'static str) -> Just {
    Just::new(str)
}

pub fn filter<F: Fn(char) -> bool>(func: F) -> Filter<F> {
    Filter::new(func)
}

pub fn choice<T: IntoChoice<O>, O>(parsers: T) -> Choice<T, O> {
    Choice::new(parsers)
}

pub fn silent_choice<T: IntoSilentChoice<O>, O>(parsers: T) -> SilentChoice<T, O> {
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

pub fn late<T>() -> Late<T> {
    Late::new()
}

pub fn custom<F: Fn(&mut ParserInput) -> PResult<O>, O>(func: F) -> Custom<F, O> {
    Custom::new(func)
}
