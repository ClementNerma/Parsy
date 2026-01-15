use std::marker::PhantomData;

use perfect_derive::perfect_derive;

use crate::{ParserInput, ParserNonConstUtils, ParserResult, parser::Parser};

/// See [`crate::ParserConstUtils::debug`]
#[perfect_derive(Debug, Clone, Copy)]
pub struct Debugging<T, P: Parser<T>, F: for<'a, 'b> Fn(DebugType<'a, 'b, T>)> {
    parser: P,
    debugger: F,
    _p: PhantomData<T>,
}

impl<T, P: Parser<T>, F: for<'a, 'b> Fn(DebugType<'a, 'b, T>)> Debugging<T, P, F> {
    pub const fn new(parser: P, debugger: F) -> Self {
        Self {
            parser,
            debugger,
            _p: PhantomData,
        }
    }
}

impl<T, P: Parser<T>, F: for<'a, 'b> Fn(DebugType<'a, 'b, T>)> Parser<T> for Debugging<T, P, F> {
    fn parse_inner(&self, input: &mut ParserInput) -> ParserResult<T> {
        (self.debugger)(DebugType::Input(input));

        let result = self.parser.parse(input);

        (self.debugger)(DebugType::Result(&result));

        result
    }
}

#[derive(Debug)]
pub enum DebugType<'a, 'b, T> {
    Input(&'a ParserInput<'b>),
    Result(&'a ParserResult<T>),
}
