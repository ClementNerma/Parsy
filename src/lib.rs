#![forbid(unsafe_code)]
#![forbid(unused_must_use)]
#![warn(unused_extern_crates)]

mod error;
mod input;
mod parser;
mod parsers;
mod span;

pub use self::{error::*, input::*, parser::*, parsers::*, span::*};

#[cfg(feature = "error-reporting")]
mod report;
#[cfg(feature = "error-reporting")]
pub use report::*;
