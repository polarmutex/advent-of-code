pub mod day;
pub mod inputs;
pub mod line;
mod parser;
pub mod prelude;
mod submission;
pub mod vec;

use crate::submission::FinalResult;
//use ahash::AHashSet;
use anyhow::Result;
use colored::Colorize;
use day::{Day, DayResult};
use inputs::Inputs;

#[macro_export]
macro_rules! main {
    ($year:literal, $($day:tt,)*) => {
        $(mod $day;)*

        use clap::{Args, Parser, Subcommand};

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
        }

        #[derive(Args)]
        struct Run {
            days: Vec<u32>,
        }

        pub fn main() {
            let cli = Cli::parse();
            match &cli.command {
                Commands::Run(name) => {
                    framework::run($year, &[
                        $(&$day::day(),)*
                    ], &name.days);
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

    //let args = std::env::args().collect::<Vec<String>>();
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
