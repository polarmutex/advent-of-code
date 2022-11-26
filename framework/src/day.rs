use crate::parsers::error::Finish;
use crate::parsers::ParseResult;
use anyhow::Result;
use std::borrow::Borrow;
use std::marker::PhantomData;
use std::time::{Duration, Instant};

#[macro_export]
macro_rules! day {
    ($nr:literal, $parser:expr => $pt1:expr, $pt2:expr) => {
        pub fn day() -> impl Day {
            $crate::day::DayCommon {
                nr: $nr,
                parser: $parser,
                pt1: $pt1,
                pt2: $pt2,
                phantom1: ::std::marker::PhantomData,
                phantom2: ::std::marker::PhantomData,
            }
        }
    };
}

#[macro_export]
macro_rules! tests {
    ($($x:tt)*) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use $crate::input_tests;
            use $crate::simple_tests;

            $($x)*
        }
    };
}

#[macro_export]
macro_rules! simple_tests {
    ($parse:expr, $pt:expr, $pt_name:ident, $($input:expr => $expected:expr),+$(,)*) => {
        #[test]
        fn $pt_name() -> ::anyhow::Result<()> {
            $({
                let input = $crate::parsers::error::Finish::finish($parse($input))?;
                let result = $pt(&input)?;
                let expected = $expected;
                if result != expected {
                    return Err(anyhow::anyhow!("Expected: {expected}, but got: {result}"));
                }
            })+
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! input_tests {
    ($parse:expr, $pt:expr, $pt_name:ident, $expected:expr) => {
        #[test]
        fn $pt_name() -> ::anyhow::Result<()> {
            let path = std::path::Path::new("../data/2021/01.txt");
            let input = match std::fs::read(path) {
                Ok(mut input) => {
                    input.retain(|c| *c != b'\r');
                    input
                }
                Err(e) => {
                    return Err(anyhow::anyhow!(format!(
                        "failed to get input {}",
                        e.to_string()
                    )))
                }
            };
            let parsed_input = $crate::parsers::error::Finish::finish($parse(&input))?;
            let result = $pt(&parsed_input)?;
            let expected = $expected;
            if result != expected {
                return Err(anyhow::anyhow!("Expected: {expected}, but got: {result}"));
            }
            Ok(())
        }
    };
}

pub enum DayResult {
    NoInput(anyhow::Error),
    ParseFailed(anyhow::Error),
    Ran {
        part1: Result<u32>,
        part2: Result<u32>,
    },
}

pub struct BenchOutputs {
    pub parse: Duration,
    pub part1: Duration,
    pub part2: Duration,
}

pub trait Day {
    fn nr(&self) -> u32;
    fn exec(&self, input: &[u8]) -> DayResult;
    fn exec_bench(&self, input: &[u8]) -> Result<BenchOutputs>;
}

pub struct DayCommon<P, P1, P2, I, I1, I2>
where
    P: for<'s> Fn(&'s [u8]) -> ParseResult<'s, I>,
    P1: Fn(&I1) -> Result<u32>,
    P2: Fn(&I2) -> Result<u32>,
    I: Borrow<I1> + Borrow<I2>,
    I1: ?Sized,
    I2: ?Sized,
{
    pub nr: u32,
    pub parser: P,
    pub pt1: P1,
    pub pt2: P2,
    pub phantom1: PhantomData<I1>,
    pub phantom2: PhantomData<I2>,
}

impl<P, P1, P2, I, I1, I2> Day for DayCommon<P, P1, P2, I, I1, I2>
where
    P: for<'s> Fn(&'s [u8]) -> ParseResult<'s, I>,
    P1: Fn(&I1) -> Result<u32>,
    P2: Fn(&I2) -> Result<u32>,
    I: Borrow<I1> + Borrow<I2>,
    I1: ?Sized,
    I2: ?Sized,
{
    fn nr(&self) -> u32 {
        self.nr
    }

    fn exec(&self, input: &[u8]) -> DayResult {
        let input = match (self.parser)(input).finish() {
            Ok(x) => x,
            Err(e) => return DayResult::ParseFailed(e),
        };
        let part1 = (self.pt1)(input.borrow());
        let part2 = (self.pt2)(input.borrow());
        DayResult::Ran { part1, part2 }
    }

    fn exec_bench(&self, input: &[u8]) -> Result<BenchOutputs> {
        let start = Instant::now();
        let parse_result = (self.parser)(input);
        let parse = Instant::now() - start;

        let input = parse_result.finish()?;

        let start = Instant::now();
        (self.pt1)(input.borrow()).unwrap();
        let part1 = Instant::now() - start;

        let start = Instant::now();
        (self.pt2)(input.borrow()).unwrap();
        let part2 = Instant::now() - start;

        Ok(BenchOutputs {
            parse,
            part1,
            part2,
        })
    }
}
