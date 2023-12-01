use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;

boilerplate!(
    Day,
    6,
    "\
3,4,3,1,2",
    "data/06.txt"
);

impl Solution for Day {
    type Parsed = Vec<u64>;
    type Answer = u64;
    const EXAMPLE_ANSWER_1: Self::Answer = 5934;
    const ANSWER_1: Self::Answer = 388739;
    const EXAMPLE_ANSWER_2: Self::Answer = 26984457539;
    const ANSWER_2: Self::Answer = 1741362314973;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let i = input
            .split(',')
            .map(|num| num.parse::<u64>().expect("u8 digit"))
            .collect();
        Ok(("", i))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        sim_fish(80, &input)
    }

    fn part2(input: Self::Parsed) -> Self::Answer {
        sim_fish(256, &input)
    }
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
