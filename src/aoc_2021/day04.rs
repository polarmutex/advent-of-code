use advent_of_code_traits::{days::Day4, ParseInput, Solution};

use crate::AdventOfCode2021;

#[derive(Debug, Clone)]
pub struct GridBoard {
    grid: [[(i32, bool); 5]; 5],
}

impl GridBoard {
    fn new(grid: [[i32; 5]; 5]) -> Self {
        Self {
            grid: grid.map(|row| row.map(|n| (n, false))),
        }
    }

    fn is_winning(&self) -> bool {
        let any_row_matched = self
            .grid
            .iter()
            .any(|row| row.iter().all(|(_, marked)| *marked));
        let any_col_matched = (0..5).any(|x| (0..5).all(|y| self.grid[y][x].1));
        any_row_matched || any_col_matched
    }

    fn mark_number(&mut self, n: i32) {
        for row in self.grid.iter_mut() {
            for (v, marked) in row {
                if *v == n {
                    *marked = true;
                }
            }
        }
    }

    fn sum_unmarked(&self) -> i32 {
        self.grid
            .map(|row| {
                row.iter()
                    .filter_map(|(n, marked)| if *marked { None } else { Some(n) })
                    .sum()
            })
            .iter()
            .sum()
    }
}

fn parse_line(s: &str) -> [i32; 5] {
    let mut line = [0; 5];
    let numbers: Vec<i32> = s
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();
    line.copy_from_slice(&numbers);
    line
}

impl ParseInput<Day4> for AdventOfCode2021 {
    type Parsed = (Vec<i32>, Vec<GridBoard>);

    fn parse_input(input: &str) -> Self::Parsed {
        let mut lines = input.lines();
        let numbers = lines
            .next()
            .unwrap()
            .split(',')
            .filter_map(|n| n.parse().ok())
            .collect();

        let mut boards = vec![];
        while let Some(_) = lines.next() {
            let mut grid = [[0; 5]; 5];
            for i in 0..5 {
                grid[i] = parse_line(lines.next().unwrap());
            }
            let board = GridBoard::new(grid);
            boards.push(board);
        }

        (numbers, boards)
    }
}

impl Solution<Day4> for AdventOfCode2021 {
    type Part1Output = i32;
    type Part2Output = i32;

    fn part1(input: &(Vec<i32>, Vec<GridBoard>)) -> Self::Part1Output {
        let (numbers, boards) = input;
        let mut boards = boards.clone();
        for n in numbers {
            for board in boards.iter_mut() {
                board.mark_number(*n);
                if board.is_winning() {
                    return board.sum_unmarked() * n;
                }
            }
        }
        unreachable!("No answer for part1")
    }

    fn part2(input: &(Vec<i32>, Vec<GridBoard>)) -> Self::Part2Output {
        let (numbers, boards) = input;
        let mut boards = boards.clone();
        let mut winning_boards = Vec::new();
        for n in numbers {
            for (i, board) in boards.iter_mut().enumerate() {
                if let Some(_) = winning_boards.iter().find(|(index, _)| *index == i) {
                    continue;
                }
                board.mark_number(*n);
                if board.is_winning() {
                    winning_boards.push((i, n));
                }
            }
        }
        match winning_boards.pop() {
            Some((i, n)) => boards[i].sum_unmarked() * n,
            None => unreachable!("No answer for part2"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    const EXAMPLE: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;

    #[test]
    fn test_part1() {
        let input = <AdventOfCode2021 as ParseInput<Day4>>::parse_input(EXAMPLE);
        assert_eq!(<AdventOfCode2021 as Solution<Day4>>::part1(&input), 4512);
    }

    #[test]
    fn test_part2() {
        let input = <AdventOfCode2021 as ParseInput<Day4>>::parse_input(EXAMPLE);
        assert_eq!(<AdventOfCode2021 as Solution<Day4>>::part2(&input), 1924);
    }

    #[test]
    fn test_answers() {
        let input_file = "data/2021/day04_github.txt";
        let input_str =
            read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
        let input = <AdventOfCode2021 as ParseInput<Day4>>::parse_input(&input_str);
        assert_eq!(<AdventOfCode2021 as Solution<Day4>>::part1(&input), 46920);
        assert_eq!(<AdventOfCode2021 as Solution<Day4>>::part2(&input), 12635);
    }
}
