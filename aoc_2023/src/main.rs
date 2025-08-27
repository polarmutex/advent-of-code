use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <day>", args[0]);
        eprintln!("Available days: 1-25");
        std::process::exit(1);
    }

    let day: u32 = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Invalid day number: {}", args[1]);
        std::process::exit(1);
    });

    if !(1..=25).contains(&day) {
        eprintln!("Day {} not in valid range 1-25", day);
        std::process::exit(1);
    }

    println!("Day {} solutions are available in the solutions module", day);
}