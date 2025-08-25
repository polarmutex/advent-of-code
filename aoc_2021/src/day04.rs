use common::{solution, Answer};
use std::collections::HashSet;

solution!("Giant Squid", 4);

type BingoBoard = Vec<Vec<u32>>;

#[derive(Debug, Clone)]
struct GameInput {
    numbers: Vec<u32>,
    bingo_boards: Vec<BingoBoard>,
}

impl std::str::FromStr for GameInput {
    type Err = miette::Error;
    fn from_str(input: &str) -> Result<GameInput, Self::Err> {
        let sections: Vec<&str> = input.trim().split("\n\n").collect();

        if sections.is_empty() {
            return Err(miette::miette!("No sections found in input"));
        }

        let numbers: Result<Vec<u32>, _> = sections[0]
            .split(',')
            .map(|num| num.parse::<u32>())
            .collect();
        
        let numbers = numbers.map_err(|e| miette::miette!("Failed to parse numbers: {}", e))?;

        let bingo_boards: Result<Vec<BingoBoard>, miette::Error> = sections[1..]
            .iter()
            .map(|board| {
                let lines: Vec<&str> = board.lines().collect();
                if lines.len() != 5 {
                    return Err(miette::miette!("Bingo board must have 5 rows"));
                }
                
                let mut bingo_board = vec![vec![0; 5]; 5];
                for (iy, row) in lines.iter().enumerate() {
                    let values: Vec<&str> = row.split_whitespace().collect();
                    if values.len() != 5 {
                        return Err(miette::miette!("Bingo board row must have 5 numbers"));
                    }
                    for (ix, val) in values.iter().enumerate() {
                        bingo_board[ix][iy] = val.parse::<u32>()
                            .map_err(|e| miette::miette!("Invalid number in bingo board: {}", e))?;
                    }
                }
                Ok(bingo_board)
            })
            .collect();
            
        let bingo_boards = bingo_boards?;
        
        Ok(GameInput {
            numbers,
            bingo_boards,
        })
    }
}

type Input = GameInput;

fn parse(input: &str) -> nom::IResult<&str, Input> {
    match input.parse::<GameInput>() {
        Ok(game_input) => Ok(("", game_input)),
        Err(_) => Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::MapRes))),
    }
}

fn is_bingo(board: &BingoBoard, seen_digits: &HashSet<u32>) -> bool {
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

fn get_unmarked_sum(board: &BingoBoard, seen_digits: &HashSet<u32>) -> u32 {
    board
        .iter()
        .flat_map(|column| column.iter())
        .filter(|nr| !seen_digits.contains(nr))
        .sum()
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, game_input) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let mut seen_digits = HashSet::with_capacity(game_input.numbers.len());
    for &num in &game_input.numbers {
        seen_digits.insert(num);

        let board = match game_input
            .bingo_boards
            .iter()
            .find(|board| is_bingo(board, &seen_digits))
        {
            Some(x) => x,
            None => continue,
        };
        let unmarked_sum = get_unmarked_sum(board, &seen_digits);
        return Ok((unmarked_sum * num).into());
    }
    Ok(0.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, game_input) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let mut seen_digits = HashSet::with_capacity(game_input.numbers.len());
    let mut remaining_boards = game_input.bingo_boards.clone();
    for &num in &game_input.numbers {
        seen_digits.insert(num);
        if remaining_boards.len() == 1 && is_bingo(&remaining_boards[0], &seen_digits) {
            let unmarked_sum = get_unmarked_sum(&remaining_boards[0], &seen_digits);
            return Ok((unmarked_sum * num).into());
        }
        remaining_boards.retain(|board| !is_bingo(board, &seen_digits));
    }
    Ok(0.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
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
         2  0 12  3  7
    "};

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 4512.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 1924.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2021, 4)?;
        assert_eq!(super::part_1(input.as_str())?, 46920.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2021, 4)?;
        assert_eq!(super::part_2(input.as_str())?, 12635.into());
        Ok(())
    }
}
