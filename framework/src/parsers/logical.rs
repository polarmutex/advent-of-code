use super::*;

pub trait ParserLogicalExt: Sized + Parser {
    /// Evaluates two parsers sequentially, and returns a tuple of their outputs
    fn and<P2: Parser>(self, parser: P2) -> And<Self, P2> {
        And(self, parser)
    }

    /// Attempts the first parser, and upon failure attempts the second parser
    fn or<'s, P2: Parser<Output<'s> = Self::Output<'s>>>(self, parser: P2) -> Or<Self, P2> {
        Or(self, parser)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct And<P1, P2>(P1, P2);

#[derive(Debug, Clone, Copy)]
pub struct Or<P1, P2>(P1, P2);

impl<P1: Parser> ParserLogicalExt for P1 {}

impl<P1: Parser, P2: Parser> Parser for And<P1, P2> {
    type Output<'s> = (P1::Output<'s>, P2::Output<'s>);

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
        let (o1, remainder) = self.0.parse(input)?;
        let (o2, remainder) = self.1.parse(remainder)?;
        Ok(((o1, o2), remainder))
    }
}

impl<P1: Parser, P2: for<'s> Parser<Output<'s> = P1::Output<'s>>> Parser for Or<P1, P2> {
    type Output<'s> = P1::Output<'s>;

    fn parse<'s>(&self, input: &'s [u8]) -> ParseResult<'s, Self::Output<'s>> {
        self.0.parse(input).or_else(|_| self.1.parse(input))
    }
}
