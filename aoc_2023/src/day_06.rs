use common::{solution, Answer};
use nom::bytes::complete::is_not;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::Parser;
use nom_supreme::ParserExt;
use tracing::info;

solution!("Wait For It", 6);

type Input = Races;

#[derive(Clone, Debug)]
struct Races {
    times: Vec<u32>,
    distances: Vec<u32>,
}

#[tracing::instrument(skip(input))]
fn parse_nums(input: &str) -> nom::IResult<&str, Vec<u32>> {
    is_not("0123456789")
        .precedes(separated_list1(space1, complete::u32))
        .parse(input)
}

#[tracing::instrument(skip(data))]
fn parse(data: &str) -> nom::IResult<&str, Input> {
    let (input, (times, distances)) =
        separated_pair(parse_nums, line_ending, parse_nums).parse(data)?;
    info!(?times);
    info!(?distances);
    Ok((input, Races { times, distances }))
}

#[tracing::instrument(skip(input))]
fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let result = data.times
        .into_iter()
        .zip(data.distances)
        .map(|(t, d)| win_conditions(t as f64, d as f64))
        .product::<u64>();
        
    Ok(result.into())
}

#[tracing::instrument(skip(input))]
fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
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
    let result = win_conditions(time as f64, distance as f64);
    
    Ok(result.into())
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

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;
    use rstest::rstest;

    const EXAMPLE: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 288.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 71503.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2023, 6)?;
        assert_eq!(super::part_1(input.as_str())?, 449820.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2023, 6)?;
        assert_eq!(super::part_2(input.as_str())?, 42250895.into());
        Ok(())
    }

    #[rstest]
    #[case( 7, 9, 4)]
    #[case( 15, 40, 8)]
    #[case( 30, 200, 9)]
    fn line_test(
        #[case] time: u64,
        #[case] distance: u64,
        #[case] expected: u64,
    ) {
        assert_eq!(expected, super::win_conditions(time as f64, distance as f64))
    }
}
