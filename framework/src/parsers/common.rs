pub use super::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Token<T> {
    value: T,
}

// Parser for Token u8
impl Parser for Token<u8> {
    type Output<'s> = ();
    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, ()> {
        if let Some(&c) = input.first() {
            if c == self.value {
                return Ok(((), &input[1..]));
            }
        }
        Err((ParseError::TokenDoesNotMatch, input))
    }
}

impl<T: Clone> Parser for Token<(u8, T)> {
    type Output<'s> = T;
    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, T> {
        if let Some(&c) = input.first() {
            if c == self.value.0 {
                return Ok((self.value.1.clone(), &input[1..]));
            }
        }
        Err((ParseError::TokenDoesNotMatch, input))
    }
}
impl<'t> Parser for Token<&'t [u8]> {
    type Output<'s> = ();
    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, ()> {
        if input.starts_with(self.value) {
            Ok(((), &input[self.value.len()..]))
        } else {
            Err((ParseError::TokenDoesNotMatch, input))
        }
    }
}

impl<'t, T: Clone> Parser for Token<(&'t [u8], T)> {
    type Output<'s> = T;
    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, T> {
        if input.starts_with(self.value.0) {
            Ok((self.value.1.clone(), &input[self.value.0.len()..]))
        } else {
            Err((ParseError::TokenDoesNotMatch, input))
        }
    }
}

impl<'t, const N: usize> Parser for Token<&'t [u8; N]> {
    type Output<'s> = ();
    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, ()> {
        if input.starts_with(self.value) {
            Ok(((), &input[self.value.len()..]))
        } else {
            Err((ParseError::TokenDoesNotMatch, input))
        }
    }
}

impl<'t, T: Clone, const N: usize> Parser for Token<(&'t [u8; N], T)> {
    type Output<'s> = T;
    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, T> {
        if input.starts_with(self.value.0) {
            Ok((self.value.1.clone(), &input[self.value.0.len()..]))
        } else {
            Err((ParseError::TokenDoesNotMatch, input))
        }
    }
}

pub fn token<T>(token: T) -> Token<T> {
    Token { value: token }
}
