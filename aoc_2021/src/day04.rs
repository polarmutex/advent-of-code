use ahash::AHashSet;
use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;
use itertools::Itertools;

boilerplate!(
    Day,
    4,
    "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
 2  0 12  3  7",
    "data/04.txt"
);

type BingoBoard = Vec<Vec<u32>>;

#[derive(Debug, Clone)]
struct Input {
    numbers: Vec<u32>,
    bingo_boards: Vec<BingoBoard>,
}

impl std::str::FromStr for Input {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Input, Self::Err> {
        let sections: Vec<&str> = input.trim().split("\n\n").collect();

        for section in sections[1..].iter() {
            println!("{}\n\n", section);
            assert!(section.split('\n').collect_vec().len() == 5)
        }

        let numbers: Vec<u32> = sections[0]
            .split(',')
            .map(|num| num.parse::<u32>().expect(""))
            .collect();
        let bingo_boards: Vec<BingoBoard> = sections[1..]
            .iter()
            //.map(|board| let bingo_board = vec![vec![]]; {board.lines().enumerate().map(|(iy, row)| row.split_whitespace().enumerate(|(ix, val)| val ); val)
            .map(|board| {
                let mut bingo_board = vec![vec![0; 5]; 5];
                board.lines().enumerate().for_each(|(iy, row)| {
                    row.split_whitespace().enumerate().for_each(|(ix, val)| {
                        bingo_board[ix][iy] =
                            val.parse::<u32>().expect("bingo board contains numbers");
                    })
                });
                bingo_board
            })
            .collect();
        let out = Input {
            numbers,
            bingo_boards,
        };
        Ok(out)
    }
}

fn is_bingo(board: &BingoBoard, seen_digits: &AHashSet<u32>) -> bool {
    let check_row = (0..5).any(|y| {
        (0..5)
            .map(|x| board[x][y])
            .all(|num| seen_digits.contains(&num))
    });
    let check_col = (0..5).any(|x| {
        (0..5)
            .map(|y| board[x][y])
            .all(|num| seen_digits.contains(&num))
    });
    check_row || check_col
}

fn get_unmarked_sum(board: &BingoBoard, seen_digits: &AHashSet<u32>) -> u32 {
    board
        .iter()
        .flat_map(|column| column.iter())
        .filter(|nr| !seen_digits.contains(nr))
        .sum()
}

impl Solution for Day {
    type Parsed = Input;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 4512;
    const ANSWER_1: Self::Answer = 46920;
    const EXAMPLE_ANSWER_2: Self::Answer = 1924;
    const ANSWER_2: Self::Answer = 12635;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let i: Input = input.parse::<Input>().expect("valid input");
        Ok(("", i))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        let mut seen_digits = AHashSet::with_capacity(input.numbers.len());
        for &num in &input.numbers {
            seen_digits.insert(num);

            let board = match input
                .bingo_boards
                .iter()
                .find(|board| is_bingo(board, &seen_digits))
            {
                Some(x) => x,
                None => continue,
            };
            let unmarked_sum = get_unmarked_sum(board, &seen_digits);
            return unmarked_sum * num;
        }
        0
    }

    fn part2(input: Self::Parsed) -> Self::Answer {
        let mut seen_digits = AHashSet::with_capacity(input.numbers.len());
        let mut remaining_boards = input.bingo_boards.clone();
        for &num in &input.numbers {
            seen_digits.insert(num);
            if remaining_boards.len() == 1 && is_bingo(&remaining_boards[0], &seen_digits) {
                let unmarked_sum = get_unmarked_sum(&remaining_boards[0], &seen_digits);
                return unmarked_sum * num;
            }
            remaining_boards.retain(|board| !is_bingo(board, &seen_digits));
        }
        0
    }
}
