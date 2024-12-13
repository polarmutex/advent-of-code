use common::{solution, Answer};
use glam::IVec2;
use miette::Result;
use std::collections::HashMap;

solution!("Ceres Search", 4);

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

fn part_1(input: &str) -> Result<Answer> {
    let word_search = parse(input);
    let mas = ['M', 'A', 'S'];
    Ok(word_search
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
            // info!(?position, ?value, count);
            count
        })
        .sum::<usize>()
        .into())
}

const POSS_LOCATIONS2: [[IVec2; 2]; 4] = [
    [IVec2::new(1, 1), IVec2::new(-1, -1)],
    [IVec2::new(-1, -1), IVec2::new(1, 1)],
    [IVec2::new(1, -1), IVec2::new(-1, 1)],
    [IVec2::new(-1, 1), IVec2::new(1, -1)],
];

fn part_2(input: &str) -> Result<Answer> {
    let word_search = parse(input);
    let mas = ['M', 'S'];
    Ok(word_search
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
            // info!(?position, ?value, count);
            count
        })
        .count()
        .into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const CASE: &str = indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "};

    #[test]
    // #[test_log::test]
    fn part_1_case() -> miette::Result<()> {
        assert_eq!(super::part_1(CASE)?, 18.into());
        Ok(())
    }

    #[test]
    // #[test_log::test]
    fn part_2_case() -> miette::Result<()> {
        assert_eq!(super::part_2(CASE)?, 9.into());
        Ok(())
    }

    #[test]
    // #[test_log::test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2024, 4)?;
        assert_eq!(super::part_1(input.as_str())?, 2447.into());
        Ok(())
    }

    #[test]
    // #[test_log::test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2024, 4)?;
        assert_eq!(super::part_2(input.as_str())?, 1868.into());
        Ok(())
    }
}
