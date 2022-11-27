use crate::prelude::*;
use ahash::AHashSet;

day!(4, parse => part1, part2);

type Numbers = Vec<u32>;
type BingoBoard = Vec<Vec<u32>>;

#[derive(Debug, Clone)]
struct Input {
    numbers: Vec<u32>,
    bingo_boards: Vec<BingoBoard>,
}

fn input_parser() -> impl Parser<char, (Numbers, Vec<BingoBoard>), Error = Simple<char>> {
    let number = c::text::int(10).map(|s: String| s.parse().unwrap());
    let called_numbers = number.separated_by(just(','));

    let bingo_board_line = just(' ')
        .ignore_then(number)
        .or(number)
        .separated_by(just(' '))
        .exactly(5);

    let bingo_board = (c::text::newline().ignore_then(bingo_board_line))
        .repeated()
        .exactly(5);

    let bingo_boards = bingo_board.separated_by(c::text::newline());

    called_numbers
        .then_ignore(c::text::newline())
        .then(bingo_boards)
}

fn parse(input: &str) -> ParseResult<Input> {
    let (numbers, bingo_boards) = input_parser().parse(input).unwrap();
    Ok(Input {
        numbers,
        bingo_boards,
    })
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

fn part1(input: &Input) -> Result<u32> {
    let mut seen_digits = AHashSet::with_capacity(input.numbers.len());
    for &num in &input.numbers {
        seen_digits.insert(num);
        let board = match input
            .bingo_boards
            .iter()
            .filter(|board| is_bingo(board, &seen_digits))
            .next()
        {
            Some(x) => x,
            None => continue,
        };
        let unmarked_sum = get_unmarked_sum(board, &seen_digits);
        return Ok(num * unmarked_sum);
    }
    Err(anyhow!("no solution"))
}

fn part2(input: &Input) -> Result<u32> {
    let mut seen_digits = AHashSet::with_capacity(input.numbers.len());
    let mut remaining_boards = input.bingo_boards.clone();
    for &num in &input.numbers {
        seen_digits.insert(num);
        if remaining_boards.len() == 1 {
            if is_bingo(&remaining_boards[0], &seen_digits) {
                let unmarked_sum = get_unmarked_sum(&remaining_boards[0], &seen_digits);
                return Ok(num * unmarked_sum);
            }
        }
        remaining_boards.retain(|board| !is_bingo(board, &seen_digits));
    }
    Err(anyhow!("no solution"))
}

tests! {
    const EXAMPLE: &'static str = "\
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
 2  0 12  3  7";

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 4512);
    input_tests!(2021, 4, parse, part1, part1_input_test, 46920);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 1924);
    input_tests!(2021, 4, parse, part2, part2_input_test, 12635);
}
