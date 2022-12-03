use crate::parser::ParseResult;
use crate::submission::MulSubmission;
use crate::submission::{FinalResult, ToFinalResult};
use anyhow::Result;
use clap::ValueEnum;
use std::borrow::Borrow;
use std::marker::PhantomData;
use std::time::{Duration, Instant};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Part {
    Part1,
    Part2,
}

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

pub trait ToResult {
    type Output;
    fn to_result(self) -> Result<Self::Output, anyhow::Error>;
}

impl ToResult for u32 {
    type Output = u32;
    fn to_result(self) -> Result<u32, anyhow::Error> {
        Ok(self)
    }
}

impl ToResult for MulSubmission<u32> {
    type Output = MulSubmission<u32>;
    fn to_result(self) -> Result<MulSubmission<u32>, anyhow::Error> {
        Ok(self)
    }
}

impl ToResult for Result<MulSubmission<u32>, anyhow::Error> {
    type Output = MulSubmission<u32>;
    fn to_result(self) -> Result<MulSubmission<u32>, anyhow::Error> {
        self
    }
}

#[macro_export]
macro_rules! simple_tests {
    ($parse:expr, $pt:expr, $pt_name:ident, $($input:expr => $expected:expr),+$(,)*) => {
        #[test]
        fn $pt_name() -> ::anyhow::Result<()> {
            $({
                let input = $parse($input)?;
                let result = $crate::day::ToResult::to_result($pt(&input))?;
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
    ($year:ident, $parse:expr, $pt:expr, $pt_name:ident, $expected:expr) => {
        #[test]
        fn $pt_name() -> ::anyhow::Result<()> {
            let year = $year;
            let day = day().nr();
            let path = format!("../data/{year}/{day:0>2}.txt");
            let input = match std::fs::read_to_string(path) {
                Ok(mut input) => input,
                Err(e) => {
                    return Err(anyhow::anyhow!(format!(
                        "failed to get input {}",
                        e.to_string()
                    )))
                }
            };
            let parsed_input = $parse(&input)?;
            let result = $crate::day::ToResult::to_result($pt(&parsed_input))?;
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
        part1: Result<FinalResult>,
        part2: Result<FinalResult>,
    },
}

pub struct BenchOutputs {
    pub parse: Duration,
    pub part1: Duration,
    pub part2: Duration,
}

pub trait Day {
    fn nr(&self) -> u32;
    fn exec(&self, input: &str) -> DayResult;
    fn exec_bench(&self, input: &str) -> Result<BenchOutputs>;
}

pub struct DayCommon<P, P1, P2, I, I1, I2, O1, O2>
where
    P: for<'s> Fn(&'s str) -> ParseResult<I>,
    P1: Fn(&I1) -> O1,
    P2: Fn(&I2) -> O2,
    I: Borrow<I1> + Borrow<I2>,
    I1: ?Sized,
    I2: ?Sized,
    O1: ToFinalResult,
    O2: ToFinalResult,
{
    pub nr: u32,
    pub parser: P,
    pub pt1: P1,
    pub pt2: P2,
    pub phantom1: PhantomData<I1>,
    pub phantom2: PhantomData<I2>,
}

impl<P, P1, P2, I, I1, I2, O1, O2> Day for DayCommon<P, P1, P2, I, I1, I2, O1, O2>
where
    P: for<'s> Fn(&'s str) -> ParseResult<I>,
    P1: Fn(&I1) -> O1,
    P2: Fn(&I2) -> O2,
    I: Borrow<I1> + Borrow<I2>,
    I1: ?Sized,
    I2: ?Sized,
    O1: ToFinalResult,
    O2: ToFinalResult,
{
    fn nr(&self) -> u32 {
        self.nr
    }

    fn exec(&self, input: &str) -> DayResult {
        let input = match (self.parser)(input) {
            Ok(x) => x,
            Err(e) => return DayResult::ParseFailed(e),
        };
        let part1 = (self.pt1)(input.borrow());
        let part2 = (self.pt2)(input.borrow());
        DayResult::Ran {
            part1: part1.to_final_answer(),
            part2: part2.to_final_answer(),
        }
    }

    fn exec_bench(&self, input: &str) -> Result<BenchOutputs> {
        let start = Instant::now();
        let parse_result = (self.parser)(input);
        let parse = Instant::now() - start;

        let input = parse_result?;

        let start = Instant::now();
        (self.pt1)(input.borrow());
        let part1 = Instant::now() - start;

        let start = Instant::now();
        (self.pt2)(input.borrow());
        let part2 = Instant::now() - start;

        Ok(BenchOutputs {
            parse,
            part1,
            part2,
        })
    }
}
