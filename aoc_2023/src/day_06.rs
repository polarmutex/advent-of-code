use aoc_runner_macros::{aoc, generator, solver, solution};
use nom::bytes::complete::is_not;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::Parser;
use nom_supreme::ParserExt;

#[derive(Clone, Debug)]
pub struct Races {
    times: Vec<u32>,
    distances: Vec<u32>,
}

type Input = Races;

#[aoc(2023, day6)]
pub mod solutions {
    use super::*;

    fn parse_nums(input: &str) -> nom::IResult<&str, Vec<u32>> {
        is_not("0123456789")
            .precedes(separated_list1(space1, complete::u32))
            .parse(input)
    }

    #[generator(gen)]
    pub fn input_generator(data: &str) -> Input {
        let (_, (times, distances)) =
            separated_pair(parse_nums, line_ending, parse_nums).parse(data).unwrap();
        Races { times, distances }
    }

    #[solver(part1, main)]
    pub fn solve_part_1(data: Input) -> u64 {
        data.times
            .into_iter()
            .zip(data.distances)
            .map(|(t, d)| win_conditions(t as f64, d as f64))
            .product::<u64>()
    }

    #[solver(part2, main)]
    pub fn solve_part_2(data: Input) -> u64 {
        let time = data
            .times
            .iter()
            .map(ToString::to_string)
            .collect::<String>()
            .parse::<u64>()
            .expect("valid time");
        let distance = data
            .distances
            .iter()
            .map(ToString::to_string)
            .collect::<String>()
            .parse::<u64>()
            .expect("valid distances");
        win_conditions(time as f64, distance as f64)
    }

    #[solution(part1, main)]
    pub fn part_1(input: &str) -> u64 {
        let data = input_generator(input);
        solve_part_1(data)
    }

    #[solution(part2, main)]
    pub fn part_2(input: &str) -> u64 {
        let data = input_generator(input);
        solve_part_2(data)
    }
}

pub fn win_conditions(total_time: f64, record_distance: f64) -> u64 {
    // distance_traveled = (total_time - hold_time) * hold_time
    // distance_traveled = (total_time)(hold_time) - (hold_time^2)
    // hold_time^2 - (total_time * hold_time) - distance_traveled = 0
    // quad formula?
    // hold_time = (total_time +/- sqrt(total_time^2 - 4 * distance_traveled)) / 2
    let largest = (total_time + (total_time.powi(2) - 4. * record_distance).sqrt()) / 2.;
    let smallest = (total_time - (total_time.powi(2) - 4. * record_distance).sqrt()) / 2.;
    largest.next_down().floor() as u64 - smallest.next_up().ceil() as u64 + 1
}

#[cfg(test)]
mod test {
    use aoc_runner_macros::aoc_case;
    use super::win_conditions;
    use indoc::indoc;
    use rstest::rstest;

    #[aoc_case(288, 71503)]
    const EXAMPLE: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[cfg(feature = "real-input")]
    #[aoc_case(449820, 42250895)]
    const REAL: &str = include_str!("../input/2023/day6.txt");

    #[rstest]
    #[case( 7, 9, 4)]
    #[case( 15, 40, 8)]
    #[case( 30, 200, 9)]
    fn line_test(
        #[case] time: u64,
        #[case] distance: u64,
        #[case] expected: u64,
    ) {
        assert_eq!(expected, win_conditions(time as f64, distance as f64))
    }
}
