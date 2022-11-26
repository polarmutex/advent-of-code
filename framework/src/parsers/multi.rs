use super::*;
use std::marker::PhantomData;

pub trait ParserMultiExt: Sized + Parser {
    // Repeatedly applies the parser, interspersing applications of `separator`.
    /// Fails if parser cannot be applied at least once.
    fn sep_by<'s, S, C: Default + Extend<Self::Output<'s>>>(self, separator: S) -> SepBy<Self, S, C>
    where
        S: Parser,
    {
        SepBy {
            parser: self,
            separator,
            _collection: PhantomData,
        }
    }
}

impl<P: Parser> ParserMultiExt for P {}

#[derive(Debug, Clone, Copy)]
pub struct SepBy<P, S, C> {
    parser: P,
    separator: S,
    _collection: PhantomData<C>,
}
impl<P, S, C> Parser for SepBy<P, S, C>
where
    P: Parser,
    S: Parser,
    C: Default + for<'s> Extend<P::Output<'s>>,
{
    type Output<'s> = C;

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
        let (element, mut remainder) = self.parser.parse(input)?;
        let mut elements = C::default();
        elements.extend(Some(element));
        loop {
            let after_sep = match self.separator.parse(remainder) {
                Ok((_, after_sep)) => after_sep,
                Err(_) => return Ok((elements, remainder)),
            };
            match self.parser.parse(after_sep) {
                Ok((element, after_value)) => {
                    remainder = after_value;
                    elements.extend(Some(element));
                }
                Err(_) => return Ok((elements, remainder)),
            };
        }
    }
}
