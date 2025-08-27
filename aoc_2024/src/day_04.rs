use aoc_runner_macros::{aoc, generator, solver, solution};
use glam::IVec2;
use std::collections::HashMap;

fn parse(input: &str) -> HashMap<IVec2, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| (IVec2::new(x as i32, y as i32), char))
        })
        .collect::<HashMap<IVec2, char>>()
}

const POSS_LOCATIONS1: [[IVec2; 3]; 8] = [
    [IVec2::new(0, 1), IVec2::new(0, 2), IVec2::new(0, 3)],
    [IVec2::new(0, -1), IVec2::new(0, -2), IVec2::new(0, -3)],
    [IVec2::new(1, 0), IVec2::new(2, 0), IVec2::new(3, 0)],
    [IVec2::new(-1, 0), IVec2::new(-2, 0), IVec2::new(-3, 0)],
    [IVec2::new(1, 1), IVec2::new(2, 2), IVec2::new(3, 3)],
    [IVec2::new(-1, -1), IVec2::new(-2, -2), IVec2::new(-3, -3)],
    [IVec2::new(1, -1), IVec2::new(2, -2), IVec2::new(3, -3)],
    [IVec2::new(-1, 1), IVec2::new(-2, 2), IVec2::new(-3, 3)],
];

const POSS_LOCATIONS2: [[IVec2; 2]; 4] = [
    [IVec2::new(1, 1), IVec2::new(-1, -1)],
    [IVec2::new(-1, -1), IVec2::new(1, 1)],
    [IVec2::new(1, -1), IVec2::new(-1, 1)],
    [IVec2::new(-1, 1), IVec2::new(1, -1)],
];

#[aoc(2024, day4)]
pub mod solutions {
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> HashMap<IVec2, char> {
        parse(input)
    }

    #[solver(part1, main)]
    pub fn solve_part_1(word_search: HashMap<IVec2, char>) -> usize {
        let mas = ['M', 'A', 'S'];
        word_search
            .iter()
            .filter(|(_position, value)| **value == 'X')
            .map(|(position, _value)| {
                let count = POSS_LOCATIONS1
                    .iter()
                    .map(|mas_positions| {
                        mas_positions
                            .iter()
                            .map(|offset| word_search.get(&(*position + *offset)))
                            .enumerate()
                            .all(|(index, value)| mas.get(index) == value)
                    })
                    .filter(|b| *b)
                    .count();
                count
            })
            .sum::<usize>()
    }

    #[solver(part2, main)]
    pub fn solve_part_2(word_search: HashMap<IVec2, char>) -> usize {
        let mas = ['M', 'S'];
        word_search
            .iter()
            .filter(|(_position, value)| **value == 'A')
            .filter(|(position, _value)| {
                let count = POSS_LOCATIONS2
                    .iter()
                    .map(|mas_positions| {
                        mas_positions
                            .iter()
                            .map(|offset| word_search.get(&(**position + *offset)))
                            .enumerate()
                            .all(|(index, value)| mas.get(index) == value)
                    })
                    .filter(|b| *b)
                    .count()
                    == 2;
                count
            })
            .count()
    }

    #[solution(part1, main)]
    pub fn part_1(input: &str) -> usize {
        let word_search = input_generator(input);
        solve_part_1(word_search)
    }

    #[solution(part2, main)]
    pub fn part_2(input: &str) -> usize {
        let word_search = input_generator(input);
        solve_part_2(word_search)
    }
}

#[cfg(test)]
mod tests {
    use aoc_runner_macros::aoc_case;

    #[aoc_case(18, 9)]
    const CASE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
}
