//! Constants for all 25 days of Advent
//!
//! ## Examples
//!
//! In your module containing a solution for day 1
//! ```
//! use advent_of_code_traits::days::Day1;
//! ```
//!
//! They are just plain `u32`s.
//! ```
//! # use advent_of_code_traits::days::Day1;
//! assert_eq!(1_u32, Day1);
//! ```
//!
//! They pair well with the [`Part1`](super::Part1) and [`Part2`](super::Part2) consts
//! which are just 1 and 2, but u8 instead of u32 to help avoid mixing them up by accident.
//!
//! You don't have to use these consts at all if you don't want to.
//! ```no_run
//! # use advent_of_code_traits::{ParseInput, Solution};
//! # pub struct Problem;
//! // this works the same as using `Solution<Day2, Part1>`
//! impl Solution<'_, 2, 1> for Problem {
//!     # type Input = u32;
//!     # type Output = u32;
//!     # fn solve(&self, input: &Self::Input) -> Self::Output { 1 }
//!     // ...
//! }
//! # impl ParseInput<'_, 2, 1> for Problem {
//! #     type Parsed = ();
//! #
//! #     fn parse_input(&self, input: &str) -> Self::Parsed {
//! #         todo!()
//! #     }
//! # }
//! ```

#[allow(non_upper_case_globals)]
pub const Day1: u32 = 1;
#[allow(non_upper_case_globals)]
pub const Day2: u32 = 2;
#[allow(non_upper_case_globals)]
pub const Day3: u32 = 3;
#[allow(non_upper_case_globals)]
pub const Day4: u32 = 4;
#[allow(non_upper_case_globals)]
pub const Day5: u32 = 5;
#[allow(non_upper_case_globals)]
pub const Day6: u32 = 6;
#[allow(non_upper_case_globals)]
pub const Day7: u32 = 7;
#[allow(non_upper_case_globals)]
pub const Day8: u32 = 8;
#[allow(non_upper_case_globals)]
pub const Day9: u32 = 9;
#[allow(non_upper_case_globals)]
pub const Day10: u32 = 10;
#[allow(non_upper_case_globals)]
pub const Day11: u32 = 11;
#[allow(non_upper_case_globals)]
pub const Day12: u32 = 12;
#[allow(non_upper_case_globals)]
pub const Day13: u32 = 13;
#[allow(non_upper_case_globals)]
pub const Day14: u32 = 14;
#[allow(non_upper_case_globals)]
pub const Day15: u32 = 15;
#[allow(non_upper_case_globals)]
pub const Day16: u32 = 16;
#[allow(non_upper_case_globals)]
pub const Day17: u32 = 17;
#[allow(non_upper_case_globals)]
pub const Day18: u32 = 18;
#[allow(non_upper_case_globals)]
pub const Day19: u32 = 19;
#[allow(non_upper_case_globals)]
pub const Day20: u32 = 20;
#[allow(non_upper_case_globals)]
pub const Day21: u32 = 21;
#[allow(non_upper_case_globals)]
pub const Day22: u32 = 22;
#[allow(non_upper_case_globals)]
pub const Day23: u32 = 23;
#[allow(non_upper_case_globals)]
pub const Day24: u32 = 24;
#[allow(non_upper_case_globals)]
pub const Day25: u32 = 25;
