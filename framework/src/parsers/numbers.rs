use super::ParseError;
use super::ParseResult;
use std::marker::PhantomData;

pub trait IsParsableNumber {}
#[derive(Debug, Clone, Copy)]
pub struct NumberParser<T: IsParsableNumber>(PhantomData<T>);

pub const fn number<T: IsParsableNumber>() -> NumberParser<T> {
    NumberParser(PhantomData)
}

macro_rules! impl_uint_parsing {
    ($kind:tt) => {
        impl $crate::parsers::numbers::IsParsableNumber for $kind {}
        impl $crate::parsers::Parser for NumberParser<$kind> {
            type Output<'s> = $kind;

            fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
                let first_char = *input.first().ok_or((ParseError::EmptyInput, input))?;
                if !matches!(first_char, b'0'..=b'9') {
                    return Err((ParseError::ExpectedDigit, input));
                }

                let mut remainder = &input[1..];

                let mut x = (first_char as $kind) - ('0' as $kind);
                loop {
                    let next_digit = match remainder.first() {
                        Some(&c @ b'0'..=b'9') => (c as $kind) - ('0' as $kind),
                        _ => break,
                    };
                    x = x
                        .checked_mul(10)
                        .and_then(|x| x.checked_add(next_digit))
                        .ok_or((ParseError::Overflow, input))?;
                    remainder = &remainder[1..];
                }

                Ok((x, remainder))
            }
        }
    };
}

impl_uint_parsing!(u8);
impl_uint_parsing!(u16);
impl_uint_parsing!(u32);
impl_uint_parsing!(u64);
impl_uint_parsing!(u128);
