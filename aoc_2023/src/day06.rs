use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
use itertools::Itertools;
use nom::bytes::complete::is_not;
use nom::bytes::complete::take_until;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::sequence::tuple;
use nom::Parser;
use nom_supreme::tag::complete::tag;
use nom_supreme::ParserExt;
use std::ops::Range;
use tracing::info;

boilerplate!(
    Day,
    6,
    "\
Time:      7  15   30
Distance:  9  40  200
",
    "data/06.txt"
);

#[derive(Clone, Debug)]
struct Races {
    times: Vec<u32>,
    distances: Vec<u32>,
}

#[tracing::instrument(skip(input))]
fn parse_nums(input: &str) -> IResult<Vec<u32>> {
    is_not("0123456789")
        .precedes(separated_list1(space1, complete::u32))
        .parse(input)
}

impl Solution for Day {
    type Parsed = Races;
    type Answer = u64;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    #[tracing::instrument(skip(data))]
    fn parse(data: &str) -> IResult<Self::Parsed> {
        let (input, (times, distances)) =
            separated_pair(parse_nums, line_ending, parse_nums).parse(data)?;
        info!(?times);
        info!(?distances);
        Ok((input, Races { times, distances }))
    }

    #[tracing::instrument(skip(data))]
    fn part1(data: Self::Parsed) -> Self::Answer {
        data.times
            .into_iter()
            .zip(data.distances)
            .map(|(t, d)| win_conditions(t as f64, d as f64))
            .product()
    }

    #[tracing::instrument(skip(data))]
    fn part2(data: Self::Parsed) -> Self::Answer {
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
}

fn win_conditions(total_time: f64, record_distance: f64) -> u64 {
    // distance_traveled = (total_time - hold_time) * hold_time
    // distance_traveled = (total_time)(hold_time) - (hold_time^2)
    // hold_time^2 - (total_time * hold_time) - distance_traveled = 0
    // quad formula?
    // hold_time = (total_time +/- sqrt(total_time^2 - 4 * distance_traveled)) / 2
    let largest = (total_time + (total_time.powi(2) - 4. * record_distance).sqrt()) / 2.;
    let smallest = (total_time - (total_time.powi(2) - 4. * record_distance).sqrt()) / 2.;
    dbg!(largest);
    dbg!(largest.next_down().floor());
    dbg!(smallest);
    dbg!(smallest.next_up().ceil());
    largest.next_down().floor() as u64 - smallest.next_up().ceil() as u64 + 1
}

tests! {
     const EXAMPLE: &str = "\
Time:      7  15   30
Distance:  9  40  200
";

    add_test!(part1_example, test_part1_example, EXAMPLE => 288);

    use rstest::rstest;

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
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 449820);
    add_test!(part2_example, test_part2_example, EXAMPLE => 71503);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 42250895);
}
