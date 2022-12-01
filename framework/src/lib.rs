pub mod day;
pub mod inputs;
pub mod line;
mod parser;
pub mod prelude;
mod submission;
mod submit;
pub mod vec;

use crate::submission::FinalResult;
//use ahash::AHashSet;
use crate::day::Part;
use anyhow::Result;
use colored::Colorize;
use day::{Day, DayResult};
use inputs::Inputs;
use submit::Submit;

#[macro_export]
macro_rules! main {
    ($year:literal, $($day:tt,)*) => {
        $(mod $day;)*

        use clap::{Args, Parser, Subcommand};
        use framework::day::Part;

        #[derive(Parser)]
        #[command(author, version, about, long_about = None)]
        #[command(propagate_version = true)]
        struct Cli {
            #[command(subcommand)]
            command: Commands,
        }

        #[derive(Subcommand)]
        enum Commands {
            Run(Run),
            Submit(Submit),
        }

        #[derive(Args)]
        struct Run {
            days: Vec<u32>,
        }

        #[derive(Args)]
        struct Submit {
            day: u32,
            #[arg(value_enum)]
            part: Part,
        }

        pub fn main() {
            let cli = Cli::parse();
            match &cli.command {
                Commands::Run(obj) => {
                    framework::run($year, &[
                        $(&$day::day(),)*
                    ], &obj.days);
                },
                Commands::Submit(obj) => {
                    framework::submit($year, &[
                        $(&$day::day(),)*
                    ], obj.day, obj.part);
                }
            }
        }
    };
}

pub fn run(year: u32, days: &[&dyn Day], specific_days: &[u32]) {
    println!(
        "\n🎄 {} {} {} {} 🎄\n",
        "Advent".bright_red().bold(),
        "of".bright_green(),
        "Code".blue().bold(),
        "2021".white().bold()
    );

    //let args = sd::env::args().collect::<Vec<String>>();
    //let is_bench = args.iter().any(|x| x == "--bench");
    //let is_submit = args.iter().any(|x| x == "--submit");
    //let specific_days = args
    //    .iter()
    //    .filter_map(|x| x.parse::<u32>().ok())
    //    .collect::<AHashSet<u32>>();

    let mut inputs = Inputs::new();
    for &day in days {
        if !specific_days.is_empty() && !specific_days.contains(&day.nr()) {
            continue;
        }
        //if is_bench {
        //    bench_day(&mut inputs, day);
        //} else {
        exec_day(&mut inputs, year, day);
        //}
    }
    println!();
}

fn exec_day(inputs: &mut Inputs, year: u32, day: &dyn Day) {
    let day_nr = day.nr();
    print!(
        "{} {}",
        "Day".bright_blue(),
        format!("{day_nr:>2}").bright_red().bold()
    );
    let result = match inputs.get(year, day_nr) {
        Ok(input) => day.exec(&input),
        Err(e) => DayResult::NoInput(e),
    };
    fn err_to_str(e: anyhow::Error) -> FinalResult {
        FinalResult {
            answer: "".bold().bright_red().to_string(),
            display: e.to_string().red().bold().to_string(),
        }
    }
    fn fmt_output(result: Result<FinalResult>) -> FinalResult {
        result.unwrap_or_else(err_to_str)
    }
    let (pt1_key, pt1_value, pt2_value) = match result {
        DayResult::NoInput(e) => ("no input".bright_red(), err_to_str(e), None),
        DayResult::ParseFailed(e) => ("parse error".bright_red(), err_to_str(e), None),
        DayResult::Ran { part1, part2 } => (
            "part1".bright_green(),
            fmt_output(part1),
            Some(format!("{}", part2.unwrap()).bright_green().to_string()),
        ),
    };
    print!(" :: {} {}", pt1_key, pt1_value);
    if let Some(pt2_value) = pt2_value {
        print!(" :: {} {}", "part2".bright_green(), pt2_value);
    }
    println!();
}

pub fn submit(year: u32, days: &[&dyn Day], specific_day: u32, part: Part) {
    println!(
        "\n🎄 {} {} {} {} 🎄\n",
        "Advent".bright_red().bold(),
        "of".bright_green(),
        "Code".blue().bold(),
        "2021".white().bold()
    );

    let mut inputs = Inputs::new();
    let mut submit = Submit::new();
    for &day in days {
        if day.nr() == specific_day {
            let day_nr = day.nr();
            print!(
                "{} {} {} :: ",
                "Submitting".bright_white(),
                "Day".bright_blue(),
                format!("{day_nr:>2}").bright_red().bold()
            );
            let result = match inputs.get(year, day_nr) {
                Ok(input) => day.exec(&input),
                Err(e) => DayResult::NoInput(e),
            };
            fn process_result(result: Result<submit::SubmitResult>) {
                match result.unwrap() {
                    submit::SubmitResult::TooQuick => {
                        println!("{}", "Too Quick".bright_red());
                    }
                    submit::SubmitResult::Wrong => {
                        println!("{}", "Wrong".bright_red())
                    }
                    submit::SubmitResult::AlreadyDone => {
                        println!("{}", "already done".bright_yellow())
                    }
                    submit::SubmitResult::Error => println!("{}", "error".bright_red()),
                    submit::SubmitResult::Right => {
                        println!("{}", "RIGHT".bright_green())
                    }
                };
            }
            match result {
                DayResult::NoInput(e) => println!("{}: {}", "no input".bright_red(), e),
                DayResult::ParseFailed(e) => {
                    println!("{}: {}", "parsed failed".bright_red(), e)
                }
                DayResult::Ran { part1, part2 } => {
                    match part {
                        Part::Part1 => match part1 {
                            Err(_) => println!("{}", "Part1 failed:".bright_red()),
                            _ => {
                                let result = submit.submit(
                                    year,
                                    specific_day,
                                    Part::Part1,
                                    &part1.unwrap().answer,
                                );
                                process_result(result);
                            }
                        },
                        Part::Part2 => match part2 {
                            Err(_) => println!("Part2 failed:"),
                            _ => {
                                let result = submit.submit(
                                    year,
                                    specific_day,
                                    Part::Part2,
                                    &part2.unwrap().answer,
                                );
                                process_result(result);
                            }
                        },
                    };
                }
            };
            break;
        }
    }
    println!();
}
