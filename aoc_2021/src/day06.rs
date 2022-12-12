use crate::prelude::*;

day!(6, parse => part1, part2);

fn parse(input: &str) -> ParseResult<Vec<u64>> {
    let i = input
        .split(',')
        .map(|num| num.parse::<u64>().expect("u8 digit"))
        .collect();
    Ok(i)
}

fn sim_fish(days: u32, input: &[u64]) -> u64 {
    let mut laternfish_timer_state: Vec<u64> = vec![0; 10];

    // initial
    for fish in input.iter() {
        let fish = *fish as usize;
        laternfish_timer_state[fish] += 1;
    }

    for _day in 0..days {
        let new_fish = laternfish_timer_state[0];
        laternfish_timer_state[0] = 0;
        for fish_state in 1..9 {
            laternfish_timer_state[fish_state - 1] += laternfish_timer_state[fish_state];
            laternfish_timer_state[fish_state] = 0;
        }
        laternfish_timer_state[6] += new_fish;
        laternfish_timer_state[8] = new_fish;
    }
    laternfish_timer_state.iter().sum()
}

fn part1(input: &[u64]) -> u64 {
    sim_fish(80, input)
}

fn part2(input: &[u64]) -> u64 {
    sim_fish(256, input)
}

tests! {
    const EXAMPLE: &str = "\
3,4,3,1,2";
    const INPUT: &str = include_str!("data/06.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 5934);
    simple_tests!(parse, part1, part1_input_test, INPUT => 388739);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 26984457539);
    simple_tests!(parse, part2, part2_input_test, INPUT => 1741362314973);
}
