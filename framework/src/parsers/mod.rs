mod common;
pub mod error;
mod logical;
mod multi;
pub mod numbers;

pub use common::*;
pub use error::{ParseError, ParseResult};
pub use logical::ParserLogicalExt;
pub use multi::ParserMultiExt;
pub use numbers::number;

pub trait Parser {
    type Output<'s>;
    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>>;
}
