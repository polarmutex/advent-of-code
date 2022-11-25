//! # `impl Solution<'_, Day25, Part2> for AdventOfCode2021<Day25>`
//!
//! ## What is this?
//!
//! This is [`advent_of_code_traits`](https://github.com/drmason13/advent_of_code_traits), a set of traits to implement solutions to Advent of Code in Rust.
//!
//! It takes a trait-based approach using const-generics and autoderef specialization.
//!
//! It's basically an excuse to play with rust's type system.
//!
//! ## Usage
//!
//! Please see also the [examples](https://github.com/drmason13/advent_of_code_traits/tree/main/examples).
//!
//! Implement traits with your solutions to each day of Advent of Code.
//!
//! ### Import the machinery:
//!
//! ```
//! use advent_of_code_traits::{days::*, MissingPartTwo, Part1, Part2, ParseInput, run, Solution, SolutionRunner};
//! ```
//!
//! ### Implement [`Solution`] for your struct.
//!
//! ```
//! # use advent_of_code_traits::{days::*, MissingPartTwo, Part1, Part2, ParseInput, run, Solution, SolutionRunner};
//! pub struct AdventOfCode2021<const DAY: u32>;
//!
//! impl Solution<'_, Day25, Part1> for AdventOfCode2021<Day25> {
//!     type Input = Vec<u32>;
//!     type Output = u32;
//!
//!     fn solve(&self, input: &Self::Input) -> Self::Output {
//!         // your solution to Part1 here...
//! #       1
//!     }
//! }
//!
//! # impl ParseInput<'_, Day25, Part1> for AdventOfCode2021<Day25> {
//! #     type Parsed = Vec<u32>; // <-- the input to both PartOne and PartTwo for Solution<Day1>
//! #
//! #     fn parse_input(&self, input: &str) -> Self::Parsed {
//! #         input
//! #             .lines()
//! #             .map(|s| s.parse().expect("invalid integer"))
//! #             .collect()
//! #     }
//! # }
//! # impl MissingPartTwo<Day25> for AdventOfCode2021<Day25> {}
//! ```
//!
//! That's how we solve the solution given a nicely typed `Vec<u32>`, but Advent of Code gives us plaintext input.
//!
//! So first we need to parse the input...
//!
//! ### Implement [`ParseInput`] for your struct
//!
//! ```
//! # use advent_of_code_traits::{days::*, MissingPartTwo, Part1, Part2, ParseInput, run, Solution, SolutionRunner};
//! # pub struct AdventOfCode2021<const DAY: u32>;
//! #
//! # impl Solution<'_, Day25, Part1> for AdventOfCode2021<Day25> {
//! #     type Input = Vec<u32>;
//! #     type Output = u32;
//! #
//! #     fn solve(&self, input: &Vec<u32>) -> u32 {
//! #         // your solution to Part1 here...
//! #         1
//! #     }
//! # }
//! // ..continued from above
//!
//! impl ParseInput<'_, Day25, Part1> for AdventOfCode2021<Day25> {
//!     type Parsed = Vec<u32>; // <-- the input type fed to Solution::solve
//!
//!     fn parse_input(&self, input: &'_ str) -> Self::Parsed {
//!         input
//!             .lines()
//!             .map(|s| s.parse().expect("invalid integer"))
//!             .collect()
//!     }
//! }
//! # let input = "1\n2\n3";
//! # impl MissingPartTwo<Day25> for AdventOfCode2021<Day25> {}
//! # let prb = AdventOfCode2021::<Day25>;
//! # let parsed = &prb.parse_input(&input);
//! # let ans = prb.solve(&parsed);
//! # assert_eq!(1, ans);
//! ```
//!
//! ### Mark Part2 as missing
//!
//! To run only Part1 of a day of Advent of Code, you currently need to impl `MissingPartTwo` to help disambiguate the specialization:
//! ```no_run
//! # use advent_of_code_traits::{days::*, MissingPartTwo, Part1, Part2, ParseInput, run, Solution, SolutionRunner};
//! # pub struct AdventOfCode2021<const DAY: u32>;
//! impl MissingPartTwo<Day25> for AdventOfCode2021<Day25> {}
//! ```
//!
//! If you don't do this (and haven't implemented Solution for Part2) you'll see an error like:
//! ```text
//! the method `run` exists for reference `&&&AdventOfCode2021<25_u32>`, but its trait bounds were not satisfied
//! the following trait bounds were not satisfied:
//! `AdventOfCode2021<25_u32>: MissingPartTwo<25_u32>`
//! which is required by `AdventOfCode2021<25_u32>: SolutionRunner<25_u32, 1_u16>`rustcE0599
//! ```
//!
//! Please refer to the [examples](https://github.com/drmason13/advent_of_code_traits/tree/main/examples) for more demonstrations.
//!
//! ### Run from `main.rs`
//!
//! Here comes the part where we actually run our solution!
//! ```
//! # use advent_of_code_traits::{days::*, MissingPartTwo, Part1, Part2, ParseInput, run, Solution, SolutionRunner};
//! # pub struct AdventOfCode2021<const DAY: u32>;
//! #
//! # impl Solution<'_, Day25, Part1> for AdventOfCode2021<Day25> {
//! #     type Input = Vec<u32>;
//! #     type Output = u32;
//! #
//! #     fn solve(&self, input: &Vec<u32>) -> u32 {
//! #         // your solution to Part1 here...
//! #         1
//! #     }
//! # }
//! # impl ParseInput<'_, Day25, Part1> for AdventOfCode2021<Day25> {
//! #     type Parsed = Vec<u32>; // <-- the input to both PartOne and PartTwo for Solution<Day1>
//! #
//! #     fn parse_input(&self, input: &str) -> Self::Parsed {
//! #         input
//! #             .lines()
//! #             .map(|s| s.parse().expect("invalid integer"))
//! #             .collect()
//! #     }
//! # }
//! # impl MissingPartTwo<Day25> for AdventOfCode2021<Day25> {}
//! # // for test purposes, circumvent the example code
//! # if false {
//! let input = std::fs::read_to_string("./input/2021/day25.txt").expect("failed to read input");
//! # }
//! # let input = "1\n2\n3";
//! run!(AdventOfCode2021::<Day25>, &input);
//! ```
//! This reads input from a file and passes it to your struct to parse and then solve.
//! It will print the output of your solution (which must impl Debug).
//!
//! [`run`] is currently a humble `macro_rules!` declarative macro and is *very* simple.
//! It's main purpose is to veil the use of autoderef [`specialization`].

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

//! This crate uses autoderef specialization to choose an implementation of [`SolutionRunner`]
//! that is most appropriate to the user's implementation of [`Solution`].
//!
//! There are 3 scenarios I wanted to support without you having to specify anything other than `your_struct.run()`*:
//! 1. You implemented Solution for both Part1 and Part2 and they share the same Input type
//! 2. You implemented Solution for both Part1 and Part2 but they use different Input types
//! 3. You implemented Solution for both Part1 only
//!
//! * for scenario 3, you do have to help the specialization as a user by implementing a marker trait: [`MissingPartTwo`]. We'll talk more about that soon.
//!
//! For a primer on autoderef specialization, I refer you to Lukas' [excellent article](http://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html), and of course the [original concept](https://github.com/dtolnay/case-studies/tree/master/autoref-specialization) of autoref specialization from Dtolnay. I'm standing on the shoulders of those standing on the shoulders of giants... and the view up here is wonderful.
//!
//! Where this implementation... struggled at first is because we are exposing so much to the user.
//!
//! We provide a trait that the user has to implement. This is difficult because the author setting up autoderef specialization (me, hello!) is usually the one both writing and implementing the traits. This would leave me free to impl <scenario 1> for some struct behind 2 references, and impl <scenario 3> for some struct behind 0 references and then only expose that struct and tada! it can specialize (or even better, use it internally and never expose it in a public api).
//!
//! But I want you to implement my traits, and I want you to not have to see or deal with the faff of `impl SolutionRunner for &&&YourStruct` because it's gross (I love it, but it's gross).
//!
//! Part of that comes down to providing the [`run!`] macro, which will write `(&&&your_struct).run()` for you.
//!
//! But also it means providing a blanket impl of Solution for &T where T: Solution, this is a double edged-sword.
//! On the one hand it's great because as a user you only need to impl Solution for YourStruct regardless of which specialised method you want to use.
//!
//! On the other hand, as an author it means I have to ensure no overlap in the impls of SolutionRunner I provide.
//! The difficult part of that was stopping scenario 3 (Part1 only) from overlapping with scenario 1 (Part1 and Part2).
//!
//! The trick is to tell the compiler about the **only** part of scenario 3, which is what the [`MissingPartTwo`] trait is for.
//!
//! Another trick used here, which isn't entirely necessary but is quite nice, is to (yet again) use const generics to separate the traits.
//!
//! You know how all the examples of auto(de)ref specialization use separate traits like `ViaString` and `ViaDisplay`?
//! Well one trait with different generic parameters is "different enough" for specialization to work.
//!
//! This is great for this crate because the user is importing these traits, a single trait with a generic parameter is easier to import.
//! It also saves coming up with contrived trait names for each scenario... one of the hard problems in programming sidestepped!

use std::fmt::Debug;

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

#[allow(non_upper_case_globals)]
pub const Part1: u8 = 1;

#[allow(non_upper_case_globals)]
pub const Part2: u8 = 2;

/// Implement this trait with your solution to the Advent of Code problem for a particular day and part.
///
/// Remember: Day, then Part.
///
/// The compiler will complain about u32 (days) vs u8 (parts) if you mix this up.
pub trait Solution<'a, const DAY: u32, const PART: u8> {
    type Input;
    type Output: Debug;

    fn solve(&'a self, input: &Self::Input) -> Self::Output;
}

/// Implement this trait to parse the raw input into a more useful type for your [`Solution`] to use as input.
///
/// See [`Solution::Input`]
pub trait ParseInput<'a, const DAY: u32, const PART: u8> {
    type Parsed;

    fn parse_input(&'a self, input: &'a str) -> Self::Parsed;
}

// Import this trait to run your advent of code solutions once they implement [`Solution`].
///
/// This trait doesn't need to be implemented outside of this crate.
///
/// Blanket implementations are provided that specialize if your solutions share a parsed input type or if [`MissingPartTwo`] is implemented.
pub trait SolutionRunner<'a, const DAY: u32, const IMPL: u16> {
    fn run(&'a self, input: &'a str);
}

/// The [`run`] macro expands
/// ```ignore
/// run!(AdventOfCode2021::<Day25>, &input);
/// ```
/// to
/// ```ignore
/// {
///     let problem = AdventOfCode2021::<Day25>;
///     (&&&problem).run(&input)
/// }
/// ```
/// and that's it!
///
/// What's with all the `&`s? It's autoderef specialization, see [`specialization`].
#[macro_export]
macro_rules! run {
    ($day: expr, $input: expr) => {{
        let problem = $day;
        (&&&problem).run($input)
    }};
}

/// [`MissingPartTwo`] is a marker trait to tell the compiler that your struct doesn't impl Solution for Part2.
///
/// Implementing this is required in order to run only Part1 using [`SolutionRunner::run`] without specifying which SolutionRunner impl to use manually.
///
/// Why? It's to guide the specialization by ensuring that each impl is unique. See [`specialization`] for more details.
/// ```
/// # use advent_of_code_traits::{days::Day1, MissingPartTwo, ParseInput, Part1, Solution, SolutionRunner };
/// struct AdventOfCode2021<const DAY: u32>;
///
/// impl<'a> Solution<'a, Day1, Part1> for AdventOfCode2021<Day1> {
///     // your solution to part 1
/// #     type Input = u32; type Output = u32;
/// #     fn solve(&'a self, _input: &Self::Input) -> Self::Output { 1 }
/// }
/// # impl<'a> ParseInput<'a, Day1, Part1> for AdventOfCode2021::<Day1> {
/// #     type Parsed = u32;
/// #     fn parse_input(&'a self, input: &'a str) -> Self::Parsed { 1 }
/// # }
///
/// // add this to be able to .run() AdventOfCode2021::<Day25> without an implemention for Part2
/// impl MissingPartTwo<Day1> for AdventOfCode2021<Day1> {}
///
/// # let input: &'static str = "fake input";
/// # let problem = AdventOfCode2021::<Day1>;
/// // ...
/// problem.run(&input);
/// ```
pub trait MissingPartOne<const DAY: u32> {}
pub trait MissingPartTwo<const DAY: u32> {}

// Using auto*de*ref specialization to allow more than 2 specificity levels: http://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html
// our autoderef specializations are enumerated as a const parameter to the trait (so the user can import a single trait to run their solutions)
const PART_ONE_ONLY: u16 = 1;
const FULL: u16 = 2;
const SHARED_INPUT_FULL: u16 = 3;
const NOT_IMPLEMENTED: u16 = 4;

// required for users to be able to simply implement Solution for T and get the implementation for &T required for autoderef specialization
impl<'a, T, const DAY: u32, const PART: u8> Solution<'a, DAY, PART> for &T
where
    T: Solution<'a, DAY, PART>,
{
    type Input = <T as Solution<'a, DAY, PART>>::Input;
    type Output = <T as Solution<'a, DAY, PART>>::Output;

    fn solve(&'a self, input: &Self::Input) -> Self::Output {
        <T as Solution<'a, DAY, PART>>::solve(self, input)
    }
}

// required for users to be able to simply implement ParseInput for T and get the implementation for &T required for autoderef specialization
impl<'a, T, const DAY: u32, const PART: u8> ParseInput<'a, DAY, PART> for &T
where
    T: ParseInput<'a, DAY, PART>,
{
    type Parsed = <T as ParseInput<'a, DAY, PART>>::Parsed;

    fn parse_input(&'a self, input: &'a str) -> Self::Parsed {
        <T as ParseInput<'a, DAY, PART>>::parse_input(self, input)
    }
}

// autoderef specialization
impl<'a, T: 'a, const DAY: u32> SolutionRunner<'a, DAY, SHARED_INPUT_FULL> for &'a &'a T
where
    &'a &'a T: Solution<'a, DAY, 1>
        + Solution<'a, DAY, 2>
        + ParseInput<'a, DAY, 1>
        + Solution<'a, DAY, 1, Input = <Self as ParseInput<'a, DAY, 1>>::Parsed>
        + Solution<'a, DAY, 2, Input = <Self as ParseInput<'a, DAY, 1>>::Parsed>,
{
    fn run(&'a self, input: &'a str) {
        let parsed_input = <Self as ParseInput<'a, DAY, 1>>::parse_input(self, input);
        let part1_output = <Self as Solution<'a, DAY, 1>>::solve(self, &parsed_input);
        let part2_output = <Self as Solution<'a, DAY, 2>>::solve(self, &parsed_input);
        println!("Day {DAY}\npart 1: {part1_output:?}\npart 2: {part2_output:?}");
    }
}

// autoderef specialization
impl<'a, T: 'a, const DAY: u32> SolutionRunner<'a, DAY, FULL> for &'a T
where
    &'a T: Solution<'a, DAY, 1>
        + Solution<'a, DAY, 2>
        + ParseInput<'a, DAY, 1>
        + ParseInput<'a, DAY, 2>
        + Solution<'a, DAY, 1, Input = <Self as ParseInput<'a, DAY, 1>>::Parsed>
        + Solution<'a, DAY, 2, Input = <Self as ParseInput<'a, DAY, 2>>::Parsed>,
{
    fn run(&'a self, input: &'a str) {
        let parsed_input = <Self as ParseInput<DAY, 1>>::parse_input(self, input);
        let part1_output = <Self as Solution<'a, DAY, 1>>::solve(self, &parsed_input);

        let parsed_input = <Self as ParseInput<DAY, 2>>::parse_input(self, input);
        let part2_output = <Self as Solution<'a, DAY, 2>>::solve(self, &parsed_input);

        println!("Day {DAY}\npart 1: {part1_output:?}\npart 2: {part2_output:?}");
    }
}

// autoderef specialization
impl<'a, T, const DAY: u32> SolutionRunner<'a, DAY, PART_ONE_ONLY> for T
where
    T: Solution<'a, DAY, 1>
        + ParseInput<'a, DAY, 1>
        + Solution<'a, DAY, 1, Input = <Self as ParseInput<'a, DAY, 1>>::Parsed>
        + MissingPartTwo<DAY>,
{
    fn run(&'a self, input: &'a str) {
        let parsed_input = <Self as ParseInput<DAY, 1>>::parse_input(self, input);
        let part1_output = <Self as Solution<'a, DAY, 1>>::solve(self, &parsed_input);

        println!("Day {DAY}\npart 1: {part1_output:?}");
    }
}

// autoderef specialization
impl<'a, T, const DAY: u32> SolutionRunner<'a, DAY, NOT_IMPLEMENTED> for T
where
    T: MissingPartOne<DAY> + MissingPartTwo<DAY>,
{
    fn run(&'a self, _: &'a str) {
        println!("Day {DAY}\nNOT Implemented");
    }
}
