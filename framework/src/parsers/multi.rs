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

    /// Repeatedly applies the parser, repeatedly invoking `func` with the
    /// output value, updating the accumulator which starts out as `initial`.
    fn fold<A, F>(self, initial: A, func: F) -> Fold<Self, A, F>
    where
        A: Clone,
        F: Fn(A, Self::Output<'_>) -> A,
    {
        Fold {
            parser: self,
            initial,
            func,
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

#[derive(Debug, Clone, Copy)]
pub struct Fold<P, A, F> {
    parser: P,
    initial: A,
    func: F,
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

impl<P, A, F> Parser for Fold<P, A, F>
where
    P: Parser,
    A: Clone + std::fmt::Display,
    F: Fn(A, P::Output<'_>) -> A,
{
    type Output<'s> = A;

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
        let mut accumulator = self.initial.clone();
        let mut remainder = input;
        while let Ok((value, new_remainder)) = self.parser.parse(remainder) {
            accumulator = (self.func)(accumulator, value);
            remainder = new_remainder;
        }
        Ok((accumulator, remainder))
    }
}
