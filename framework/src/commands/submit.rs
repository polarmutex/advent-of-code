use crate::aoc_cli;
use crate::commands::Part;
use crate::Runner;
use std::process;

pub fn handle(year: u32, days: &[&dyn Runner], specific_day: u8, part: Part) {
    if aoc_cli::check().is_err() {
        eprintln!("command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it.");
        process::exit(1);
    }

    for &day in days {
        if day.nr() == specific_day {
            let answer = if part == Part::Part1 {
                day.get_part1_submission()
            } else {
                day.get_part2_submission()
            };
            println!("Submitting result via aoc-cli...");
            if let Err(e) = aoc_cli::submit(specific_day, part.to_number(), answer.as_str()) {
                eprintln!("failed to call aoc-cli: {e}");
                process::exit(1);
            };
        }
    }

    process::exit(0)
}
