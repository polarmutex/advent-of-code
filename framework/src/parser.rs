use anyhow::Result;
use thiserror::Error;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseError {
    #[error("empty input")]
    EmptyInput,
    #[error("expected a digit")]
    ExpectedDigit,
    #[error("overflow")]
    Overflow,
    #[error("token does not match")]
    TokenDoesNotMatch,
    #[error("unexpected char")]
    UnexpectedChar,
    #[error("grid cell out of range, x: {0}, y: {0}")]
    GridCellOutOfRange(usize, usize),
    #[error("expected a grid cell")]
    ExpectedGridCell,
    #[error("{0}")]
    Custom(&'static str),
}
