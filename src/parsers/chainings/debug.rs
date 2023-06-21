use std::marker::PhantomData;

use crate::{parser::Parser, PResult, ParserInput};

#[derive(Debug)]
pub enum DebugType<'a, 'b, T> {
    Input(&'a ParserInput<'b>),
    Result(&'a PResult<T>),
}

pub struct Debugging<T, P: Parser<T>, F: for<'a, 'b> Fn(DebugType<'a, 'b, T>) + Clone> {
    parser: P,
    debugger: F,
    _t: PhantomData<T>,
}

impl<T, P: Parser<T>, F: for<'a, 'b> Fn(DebugType<'a, 'b, T>) + Clone> Debugging<T, P, F> {
    pub fn new(parser: P, debugger: F) -> Self {
        Self {
            parser,
            debugger,
            _t: PhantomData,
        }
    }
}

impl<T, P: Parser<T> + Clone, F: for<'a, 'b> Fn(DebugType<'a, 'b, T>) + Clone> Clone
    for Debugging<T, P, F>
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            debugger: self.debugger.clone(),
            _t: self._t,
        }
    }
}

impl<T, P: Parser<T>, F: for<'a, 'b> Fn(DebugType<'a, 'b, T>) + Clone> Parser<T>
    for Debugging<T, P, F>
{
    fn parse_inner(&self, input: &mut ParserInput) -> PResult<T> {
        (self.debugger)(DebugType::Input(input));

        let result = self.parser.parse(input);

        (self.debugger)(DebugType::Result(&result));

        result
    }
}
