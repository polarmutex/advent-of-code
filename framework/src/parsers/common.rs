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

pub fn token<T>(token: T) -> Token<T> {
    Token { value: token }
}
