#![forbid(unsafe_code)]
#![forbid(unused_must_use)]
#![warn(unused_extern_crates)]
// TODO: remove once stabilized (nightly)
#![feature(const_trait_impl)]

mod containers;
mod error;
mod input;
mod parser;
mod span;

pub mod parsers;

pub use self::{containers::*, error::*, input::*, parser::*, span::*};

#[cfg(feature = "error-reporting")]
mod report;
#[cfg(feature = "error-reporting")]
pub use report::*;
