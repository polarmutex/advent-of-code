#![feature(associated_type_defaults)]

pub mod algorithms;
pub mod aoc_cli;
pub mod commands;
pub mod grid;
pub mod line;
pub mod ocr;
pub mod vec;

// use colored::Colorize;
use nom::character::complete::line_ending;
use nom_supreme::{final_parser::final_parser, ParserExt};
use std::fmt::{Debug, Display};

pub type OutResult = Result<(), Box<dyn std::error::Error>>;
pub type SubmissionResult = Result<String, Box<dyn std::error::Error>>;
pub type IResult<'a, T> = nom::IResult<&'a str, T>;

#[macro_export]
macro_rules! main {
    ($year:literal, $($day:tt,)*) => {
        $(mod $day;)*

        use clap::{Args, Parser, Subcommand};
        use framework::commands::Cli;
        use framework::commands::Commands;
        use framework::commands::all;
        use framework::commands::download;
        use framework::commands::scaffold;
        use framework::commands::submit;

        pub const YEAR:u32 = $year;

        pub fn main() {
            let args = Cli::parse();

            match args.command {
                Commands::All {} => all::handle(),
                Commands::Download {day} => download::handle($year, day),
                Commands::Scaffold {day} => scaffold::handle($year, day),
                Commands::Submit {day, part} => submit::handle($year, &[
                        $(&$day::day(),)*
                    ], day, part),
            }
        }
    };
}

#[macro_export]
macro_rules! boilerplate {
    ($day:ident,$nr:literal,$example:literal,$input:literal) => {
        struct $day;
        use framework::AdvSolution;
        use framework::OutResult;
        use framework::Runner;
        use framework::Solution;

        impl SolutionData for $day {
            const INPUT_DATA: &'static str = include_str!($input);
            const EXAMPLE_DATA: &'static str = $example;
        }
        impl Runner for $day {
            fn nr(&self) -> u8 {
                $nr
            }
            fn get_part1_submission(&self) -> String {
                Self::part1_submission().unwrap()
            }

            fn get_part2_submission(&self) -> String {
                Self::part2_submission().unwrap()
            }
        }
        pub fn day() -> impl Runner {
            $day
        }

        // #[cfg(test)]
        // mod tests {
        //     use super::*;
        //
        //     #[test]
        //     fn test_part1_example() -> OutResult {
        //         $day::test_part1_example()
        //     }
        //
        //     #[test]
        //     fn test_part1_input() -> OutResult {
        //         $day::test_part1_input()
        //     }
        //
        //     #[test]
        //     fn test_part2_example() -> OutResult {
        //         $day::test_part2_example()
        //     }
        //
        //     #[test]
        //     fn test_part2_input() -> OutResult {
        //         $day::test_part2_input()
        //     }
        // }
    };
}

#[macro_export]
macro_rules! tests {
    ($($x:tt)*) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use framework::add_test;
            use framework::add_test_external;
            use framework::OutResult;
            // use $crate::input_tests;
            // use $crate::simple_tests;

            $($x)*
        }
    };
}

#[macro_export]
macro_rules! add_test {
    ($pt:ident, $pt_name:ident, $input:expr => $expected:expr) => {
        #[test]
        fn $pt_name() -> OutResult {
            assert_eq!(
                <Day as AdvSolution>::$pt(Day::final_parse_example($input)?),
                $expected
            );
            //println!("a: {}", Self::a(Self::final_parse(Self::DATA)?));
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! add_test_external {
    ($pt:ident, $pt_name:ident, $input:expr => $expected:expr) => {
        #[test]
        fn $pt_name() -> OutResult {
            assert_eq!(Day::$pt(Day::final_parse_example($input)?), $expected);
            //println!("a: {}", Self::a(Self::final_parse(Self::DATA)?));
            Ok(())
        }
    };
}

pub trait Runner {
    fn nr(&self) -> u8;
    fn get_part1_submission(&self) -> String;
    fn get_part2_submission(&self) -> String;
    //fn exec_bench(&self, input: &str) -> Result<BenchOutputs>;
}

pub trait SolutionData {
    const INPUT_DATA: &'static str;
    const EXAMPLE_DATA: &'static str;
}

pub trait Solution: SolutionData {
    type Parsed: Debug + Clone = &'static str;
    type Answer: Debug + Display + PartialEq<Self::AnswerExample>;
    type AnswerExample: Debug = Self::Answer;
    const EXAMPLE_ANSWER_1: Self::AnswerExample;
    const EXAMPLE_ANSWER_2: Self::AnswerExample;
    const ANSWER_1: Self::AnswerExample;
    const ANSWER_2: Self::AnswerExample;

    fn parse(data: &'static str) -> IResult<Self::Parsed>;
    fn part1(data: Self::Parsed) -> Self::Answer;
    fn part2(data: Self::Parsed) -> Self::Answer;
}

pub trait AdvSolution: SolutionData {
    type Parsed: Debug + Clone = &'static str;
    type ParsedExample: Debug + Clone = Self::Parsed;
    type Answer: Debug + Display + PartialEq<Self::AnswerExample>;
    type AnswerExample: Debug = Self::Answer;
    // const EXAMPLE_ANSWER_1: Self::AnswerExample;
    // const EXAMPLE_ANSWER_2: Self::AnswerExample;
    // const ANSWER_1: Self::AnswerExample; //why not Answer?
    // const ANSWER_2: Self::AnswerExample;

    fn parse(data: &'static str) -> IResult<Self::Parsed>;
    fn part1(data: Self::Parsed) -> Self::Answer;
    fn part2(data: Self::Parsed) -> Self::Answer;
    fn parse_example(data: &'static str) -> IResult<Self::ParsedExample>;
    fn part1_example(data: Self::ParsedExample) -> Self::Answer;
    fn part2_example(data: Self::ParsedExample) -> Self::Answer;

    fn final_parse(data: &'static str) -> Result<Self::Parsed, nom::error::Error<&str>> {
        final_parser(Self::parse.terminated(line_ending.opt()))(data)
    }

    fn final_parse_example(
        data: &'static str,
    ) -> Result<Self::ParsedExample, nom::error::Error<&str>> {
        final_parser(Self::parse_example.terminated(line_ending.opt()))(data)
    }

    fn part1_submission() -> SubmissionResult {
        let result = Self::part1(Self::final_parse(Self::INPUT_DATA)?);
        Ok(result.to_string())
    }

    fn part2_submission() -> SubmissionResult {
        let result = Self::part2(Self::final_parse(Self::INPUT_DATA)?);
        Ok(result.to_string())
    }

    // fn test_part1_example() -> OutResult
    // where
    //     <Self as AdvSolution>::AnswerExample: PartialEq,
    // {
    //     dbg!(Self::EXAMPLE_DATA);
    //     assert_eq!(
    //         Self::part1_example(Self::final_parse_example(Self::EXAMPLE_DATA)?),
    //         Self::EXAMPLE_ANSWER_1
    //     );
    //     Ok(())
    // }

    // fn test_part1_input() -> OutResult {
    //     assert_eq!(
    //         Self::part1(Self::final_parse(Self::INPUT_DATA)?),
    //         Self::ANSWER_1
    //     );
    //     Ok(())
    // }

    // fn test_part2_example() -> OutResult
    // where
    //     <Self as AdvSolution>::AnswerExample: PartialEq,
    // {
    //     assert_eq!(
    //         Self::part2_example(Self::final_parse_example(Self::EXAMPLE_DATA)?),
    //         Self::EXAMPLE_ANSWER_2
    //     );
    //     Ok(())
    // }

    // fn test_part2_input() -> OutResult {
    //     assert_eq!(
    //         Self::part2(Self::final_parse(Self::INPUT_DATA)?),
    //         Self::ANSWER_2
    //     );
    //     Ok(())
    // }

    fn submit_part1() {}
}

impl<T: Solution> AdvSolution for T {
    type Parsed = <Self as Solution>::Parsed;
    type ParsedExample = Self::Parsed;
    type Answer = <Self as Solution>::Answer;
    type AnswerExample = <Self as Solution>::AnswerExample;
    // const EXAMPLE_ANSWER_1: <Self as Solution>::AnswerExample =
    //     <Self as Solution>::EXAMPLE_ANSWER_1;
    // const EXAMPLE_ANSWER_2: <Self as Solution>::AnswerExample =
    //     <Self as Solution>::EXAMPLE_ANSWER_2;
    // const ANSWER_1: <Self as Solution>::AnswerExample = <Self as Solution>::ANSWER_1;
    // const ANSWER_2: <Self as Solution>::AnswerExample = <Self as Solution>::ANSWER_2;

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        <Self as Solution>::parse(data)
    }

    fn part1(data: Self::Parsed) -> Self::Answer {
        <Self as Solution>::part1(data)
    }

    fn part2(data: Self::Parsed) -> Self::Answer {
        <Self as Solution>::part2(data)
    }

    fn parse_example(data: &'static str) -> IResult<Self::ParsedExample> {
        Self::parse(data)
    }
    fn part1_example(data: Self::ParsedExample) -> Self::Answer {
        Self::part1(data)
    }
    fn part2_example(data: Self::ParsedExample) -> Self::Answer {
        Self::part2(data)
    }
}

// println!(
//     "\n🎄 {} {} {} {} 🎄\n",
//     "Advent".bright_red().bold(),
//     "of".bright_green(),
//     "Code".blue().bold(),
//     "2021".white().bold()
// );

// print!(
//     "{} {} {} :: ",
//     "Submitting".bright_white(),
//     "Day".bright_blue(),
//     format!("{day_nr:>2}").bright_red().bold()
// );
// fn process_result(result: Result<submit::SubmitResult>) {
//     match result.unwrap() {
//         submit::SubmitResult::TooQuick => {
//             println!("{}", "Too Quick".bright_red());
//         }
//         submit::SubmitResult::Wrong => {
//             println!("{}", "Wrong".bright_red())
//         }
//         submit::SubmitResult::AlreadyDone => {
//             println!("{}", "already done".bright_yellow())
//         }
//         submit::SubmitResult::Error => println!("{}", "error".bright_red()),
//         submit::SubmitResult::Right => {
//             println!("{}", "RIGHT".bright_green())
//         }
//     };
// }
