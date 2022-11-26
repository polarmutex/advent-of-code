pub mod day;
pub mod inputs;
pub mod parsers;
pub mod prelude;

use ahash::AHashSet;
use colored::Colorize;
use day::{Day, DayResult};
use inputs::Inputs;

#[macro_export]
macro_rules! main {
    ($year:literal, $($day:tt,)*) => {
        $(mod $day;)*

        pub fn main() {
            framework::run($year, &[
                $(&$day::day(),)*
            ])
        }
    };
}

pub fn run(year: u32, days: &[&dyn Day]) {
    println!(
        "\n🎄 {} {} {} {} 🎄\n",
        "Advent".bright_red().bold(),
        "of".bright_green(),
        "Code".blue().bold(),
        "2021".white().bold()
    );

    let args = std::env::args().collect::<Vec<String>>();
    //let is_bench = args.iter().any(|x| x == "--bench");
    //let is_submit = args.iter().any(|x| x == "--submit");
    let specific_days = args
        .iter()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<AHashSet<u32>>();

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
    let (pt1_key, pt1_value, pt2_value) = match result {
        DayResult::NoInput(e) => (
            "no input".bright_red(),
            e.to_string().red().bold().to_string(),
            None,
        ),
        DayResult::ParseFailed(e) => (
            "parse error".bright_red(),
            e.to_string().red().bold().to_string(),
            None,
        ),
        DayResult::Ran { part1, part2 } => (
            "part1".bright_green(),
            format!("{}", part1.unwrap()).bright_green().to_string(),
            Some(format!("{}", part2.unwrap()).bright_green().to_string()),
        ),
    };
    print!(" :: {} {}", pt1_key, pt1_value);
    if let Some(pt2_value) = pt2_value {
        print!(" :: {} {}", "part2".bright_green(), pt2_value);
    }
    println!();
}
