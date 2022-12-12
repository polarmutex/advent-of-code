use crate::prelude::*;

day!(3, parse => part1::<12>, part2::<12>);

type Diagnostics = Vec<u32>;
type DiagnosticsRef = [u32];

fn parse(input: &str) -> ParseResult<Diagnostics> {
    let lines: Vec<&str> = input.lines().collect();
    let diags: Diagnostics = lines
        .iter()
        .map(|line| u32::from_str_radix(line, 2).expect(""))
        .collect();
    Ok(diags)
}

fn part1<const BITS: usize>(input: &DiagnosticsRef) -> MulSubmission<u32> {
    let mut bit_counts = [0; BITS];

    for &diag in input {
        //for bit in 0..BITS {
        for (bit, count) in bit_counts.iter_mut().enumerate().take(BITS) {
            *count += (diag >> bit) & 1;
        }
    }

    let num_rows = input.len();
    let half = (num_rows / 2) as u32;

    let gamma = bit_counts
        .iter()
        .map(|bit_count| bit_count > &half)
        .enumerate()
        .fold(0, |gamma, (index, bit)| {
            gamma | ((if bit { 1 } else { 0 }) << index)
        });
    let epsilon = bit_counts
        .iter()
        .map(|bit_count| bit_count <= &half)
        .enumerate()
        .fold(0, |gamma, (index, bit)| {
            gamma | ((if bit { 1 } else { 0 }) << index)
        });
    MulSubmission(gamma, epsilon)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rating {
    OxygenGenerator,
    CO2Scrubber,
}

fn part2<const BITS: usize>(input: &DiagnosticsRef) -> MulSubmission<u32> {
    let oxygen_generator_rating = part2_compute_rating::<BITS>(Rating::OxygenGenerator, input);
    let co2_scrubber_rating = part2_compute_rating::<BITS>(Rating::CO2Scrubber, input);
    MulSubmission(oxygen_generator_rating, co2_scrubber_rating)
}

fn part2_compute_rating<const BITS: usize>(rating: Rating, input: &DiagnosticsRef) -> u32 {
    let mut temp = input.to_vec();
    // reverse because we code them into ints
    for bit in (0..BITS).rev() {
        let mut ones = 0;
        for &num in temp.iter() {
            ones += (num >> bit) & 1;
        }
        let zeroes = (temp.len() as u32) - ones;

        //println!("ones: {}", ones);
        //println!("zeroes: {}", zeroes);
        //println!("len: {}", temp.len());

        if rating == Rating::OxygenGenerator {
            let keep = if ones >= zeroes { 1 } else { 0 };
            temp.retain(|&num| (num >> bit) & 1 == keep)
        } else {
            let keep = if ones >= zeroes { 0 } else { 1 };
            temp.retain(|&num| (num >> bit) & 1 == keep)
        }

        /*
        for &t in temp.iter() {
            println!("{t:b}");
        }
        println!("");
        println!("");
        */
        if temp.len() == 1 {
            break;
        }
    }
    assert_eq!(1, temp.len());
    temp[0]
}

tests! {
    const EXAMPLE: &str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    const INPUT: &str = include_str!("data/03.txt");

    simple_tests!(parse, part1::<5>, part1_example_test, EXAMPLE => MulSubmission(22,9));
    simple_tests!(parse, part1::<12>, part1_input_test, INPUT => MulSubmission(218,3877));
    simple_tests!(parse, part2::<5>, part2_example_test, EXAMPLE => MulSubmission(23,10));
    simple_tests!(parse, part2::<12>, part2_input_test, INPUT => MulSubmission(1459,3178));

}
