use crate::prelude::*;

day!(4, parse => part1, part2);

#[derive(Debug, Clone, Copy)]
struct ElfTask {
    start: u32,
    end: u32,
}

impl FromStr for ElfTask {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').expect("invalid input for elftask");
        let task: ElfTask = ElfTask {
            start: start.parse()?,
            end: end.parse()?,
        };
        Ok(task)
    }
}

impl ElfTask {
    pub fn fully_contains(&self, other: &ElfTask) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlaps(&self, other: &ElfTask) -> bool {
        (self.start <= other.end) && (other.start <= self.end)
    }
}

#[derive(Debug, Clone, Copy)]
struct ElfCleanupPair {
    left: ElfTask,
    right: ElfTask,
}

impl FromStr for ElfCleanupPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(',').expect("invalid elf cleanup pairs");
        let pair: ElfCleanupPair = ElfCleanupPair {
            left: left.parse()?,
            right: right.parse()?,
        };
        Ok(pair)
    }
}

impl ElfCleanupPair {
    pub fn fully_contains(&self) -> bool {
        self.left.fully_contains(&self.right) || self.right.fully_contains(&self.left)
    }
    pub fn overlaps(&self) -> bool {
        self.left.overlaps(&self.right) || self.right.overlaps(&self.left)
    }
}

fn parse(input: &str) -> ParseResult<Vec<ElfCleanupPair>> {
    input
        .lines()
        .map(|line| line.parse::<ElfCleanupPair>())
        .collect()
}

fn part1(input: &[ElfCleanupPair]) -> usize {
    input.iter().filter(|pair| pair.fully_contains()).count()
}

fn part2(input: &[ElfCleanupPair]) -> usize {
    input.iter().filter(|pair| pair.overlaps()).count()
}

tests! {
    const EXAMPLE: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
    const INPUT: &str = include_str!("../../data/2022/04.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 2);
    simple_tests!(parse, part1, part1_input_test, INPUT => 573);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 4);
    simple_tests!(parse, part2, part2_input_test, INPUT => 867);
}
