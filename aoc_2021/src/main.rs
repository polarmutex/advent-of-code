//#[allow(unused_imports)]
use framework::run;
use framework::traits::{
    Day1, Day10, Day11, Day12, Day13, Day14, Day15, Day16, Day17, Day18, Day19, Day2, Day20, Day21,
    Day22, Day23, Day24, Day25, Day3, Day4, Day5, Day6, Day7, Day8, Day9, SolutionRunner,
};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

struct AdventOfCode<const DAY: u32>;

fn main() {
    let day = std::env::args()
        .nth(1)
        .expect(
            "need a day to know which solution to run, e.g. `cargo run --example cli 1` to run day 1 solutions",
        )
        .parse()
        .expect("unable to parse day, just use a number like `1`");

    let input = std::fs::read_to_string(&find_input(day)).expect("no input available for that day");

    match day {
        // we have to match because the const generic cannot be a runtime value
        1 => run!(AdventOfCode::<Day1>, &input),
        2 => run!(AdventOfCode::<Day2>, &input),
        3 => run!(AdventOfCode::<Day3>, &input),
        4 => run!(AdventOfCode::<Day4>, &input),
        5 => run!(AdventOfCode::<Day5>, &input),
        6 => run!(AdventOfCode::<Day6>, &input),
        7 => run!(AdventOfCode::<Day7>, &input),
        8 => run!(AdventOfCode::<Day8>, &input),
        9 => run!(AdventOfCode::<Day9>, &input),
        10 => run!(AdventOfCode::<Day10>, &input),
        11 => run!(AdventOfCode::<Day11>, &input),
        12 => run!(AdventOfCode::<Day12>, &input),
        13 => run!(AdventOfCode::<Day13>, &input),
        14 => run!(AdventOfCode::<Day14>, &input),
        15 => run!(AdventOfCode::<Day15>, &input),
        16 => run!(AdventOfCode::<Day16>, &input),
        17 => run!(AdventOfCode::<Day17>, &input),
        18 => run!(AdventOfCode::<Day18>, &input),
        19 => run!(AdventOfCode::<Day19>, &input),
        20 => run!(AdventOfCode::<Day20>, &input),
        21 => run!(AdventOfCode::<Day21>, &input),
        22 => run!(AdventOfCode::<Day22>, &input),
        23 => run!(AdventOfCode::<Day23>, &input),
        24 => run!(AdventOfCode::<Day24>, &input),
        25 => run!(AdventOfCode::<Day25>, &input),

        // run!(day1::Problem, &input) expands to...
        // {
        //     let problem = day1::Problem;
        //     (&&&problem).run(&input)
        // },

        // the below fails to compile (until you implement the solution to Day4)
        // 2 => run!(day4::Problem, &input),
        x => unimplemented!("invalid day: {x}"),
    }
}

// you can mostly ignore this, this makes the example run more reliably from different directories
fn find_input(day: u32) -> String {
    let parent_dir = "data/2021";
    let parent_dir_path = std::path::Path::new(parent_dir);
    assert!(parent_dir_path.exists());

    format!("{}/day{:02}.txt", parent_dir, day)
}
