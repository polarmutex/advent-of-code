use aoc_runner_macros::{aoc, generator, solver, solution};


type Input = Vec<u32>;

#[aoc(2021, day3)]
pub mod solutions {
    use super::*;

    fn parse(input: &str) -> nom::IResult<&str, Input> {
        let diags: Result<Vec<u32>, _> = input
            .lines()
            .map(|line| u32::from_str_radix(line, 2))
            .collect();
        
        match diags {
            Ok(d) => Ok(("", d)),
            Err(_) => Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::MapRes))),
        }
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> u32 {
        super::solve_part1::<12>(input)
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> u32 {
        super::solve_part2::<12>(input)
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> u32 {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> u32 {
        let data = input_generator(input);
        solve_part2(&data)
    }
}

fn solve_part1<const BITS: usize>(input: &[u32]) -> u32 {
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

fn solve_part2<const BITS: usize>(input: &[u32]) -> u32 {
    let oxygen_generator_rating = part2_compute_rating::<BITS>(Rating::OxygenGenerator, input);
    let co2_scrubber_rating = part2_compute_rating::<BITS>(Rating::CO2Scrubber, input);
    oxygen_generator_rating * co2_scrubber_rating
}

fn part2_compute_rating<const BITS: usize>(rating: Rating, input: &[u32]) -> u32 {
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

#[cfg(test)]
mod test {
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
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
    "};

    #[test]
    fn part_1_example() {
        assert_eq!(super::solutions::part_1(EXAMPLE), 198);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(super::solutions::part_2(EXAMPLE), 230);
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        Ok(())
    }
}