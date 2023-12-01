use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;

boilerplate!(
    Day,
    3,
    "\
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
01010
",
    "data/03.txt"
);

type Diagnostics = Vec<u32>;
type DiagnosticsRef = [u32];

impl Solution for Day {
    type Parsed = Diagnostics;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 198;
    const ANSWER_1: Self::Answer = 845186;
    const EXAMPLE_ANSWER_2: Self::Answer = 230;
    const ANSWER_2: Self::Answer = 4636702;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let lines: Vec<&str> = input.lines().collect();
        let diags: Diagnostics = lines
            .iter()
            .map(|line| u32::from_str_radix(line, 2).expect(""))
            .collect();
        Ok(("", diags))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        solve_part1::<12>(&input)
    }

    fn part2(input: Self::Parsed) -> Self::Answer {
        solve_part2::<12>(&input)
    }
}

fn solve_part1<const BITS: usize>(input: &DiagnosticsRef) -> u32 {
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
    gamma * epsilon
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rating {
    OxygenGenerator,
    CO2Scrubber,
}

fn solve_part2<const BITS: usize>(input: &DiagnosticsRef) -> u32 {
    let oxygen_generator_rating = part2_compute_rating::<BITS>(Rating::OxygenGenerator, input);
    let co2_scrubber_rating = part2_compute_rating::<BITS>(Rating::CO2Scrubber, input);
    oxygen_generator_rating * co2_scrubber_rating
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
