use crate::aoc_cli;
use std::process;

pub fn handle(year: i32, day: u8) {
    if aoc_cli::check().is_err() {
        eprintln!("command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it.");
        process::exit(1);
    }

    if let Err(e) = aoc_cli::download(year, day) {
        eprintln!("failed to call aoc-cli: {e}");
        process::exit(1);
    };

    process::exit(0)
}
