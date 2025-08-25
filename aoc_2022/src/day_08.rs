use ahash::AHashMap;
use ahash::AHashSet;
use common::{solution, Answer};

solution!("Treetop Tree House", 8);

#[derive(Debug, Clone)]
struct TreeInput {
    tree_grid: Vec<Vec<u8>>,
}

type Input = TreeInput;

fn parse(input: &str) -> nom::IResult<&str, Input> {
        let num_rows = input.lines().count();
        let num_cols = input.lines().next().unwrap().chars().count();
        let mut tree_grid = vec![vec![0; num_cols]; num_rows];
        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, val)| {
                tree_grid[y][x] = val.to_string().parse::<u8>().expect("u8 number")
            })
        });
        Ok(("", TreeInput { tree_grid }))
    }

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    fn part1_impl(input: TreeInput) -> usize {
        let map = input.tree_grid.clone();

        let x_len = map[0].len();
        let x_max = x_len - 1;
        let y_len = map.len();
        let y_max = y_len - 1;

        let mut vis = AHashSet::<(usize, usize)>::new();

        // range goes to last num - 1
        for y in 0..y_len {
            for x in 0..x_len {
                if (x == 0) || (x == x_max) || (y == 0) || (y == y_max) {
                    vis.insert((x, y));
                } else {
                    let top = map[0..y].iter().map(|val| val[x]).collect::<Vec<_>>();
                    let bottom = map[y + 1..y_len]
                        .iter()
                        .map(|val| val[x])
                        .collect::<Vec<_>>();
                    let left = map[y][0..x].iter().collect::<Vec<_>>();
                    let right = map[y][x + 1..x_len].iter().collect::<Vec<_>>();

                    if top.iter().all(|val| *val < map[y][x])
                        || bottom.iter().all(|val| *val < map[y][x])
                        || right.iter().all(|val| **val < map[y][x])
                        || left.iter().all(|val| **val < map[y][x])
                    {
                        vis.insert((x, y));
                    }
                }
            }
        }
        vis.len()
    }

    Ok(part1_impl(data).into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    fn part2_impl(input: TreeInput) -> usize {
        let map = input.tree_grid.clone();

        let x_len = map[0].len();
        let y_len = map.len();

        let mut vis = AHashMap::<(usize, usize), usize>::new();

        // range goes to last num - 1
        for y in 0..y_len {
            for x in 0..x_len {
                let top = map[0..y].iter().rev().map(|val| val[x]).collect::<Vec<_>>();
                let bottom = map[y + 1..y_len]
                    .iter()
                    .map(|val| val[x])
                    .collect::<Vec<_>>();
                let left = map[y][0..x].iter().copied().rev().collect::<Vec<_>>();
                let right = map[y][x + 1..x_len].to_vec();

                let score = viewing_distance(map[y][x], &top)
                    * viewing_distance(map[y][x], &bottom)
                    * viewing_distance(map[y][x], &left)
                    * viewing_distance(map[y][x], &right);
                vis.insert((x, y), score);
            }
        }
        *vis.values().max().unwrap()
    }
    
    Ok(part2_impl(data).into())
}

fn viewing_distance(height: u8, view: &[u8]) -> usize {
    let mut score = 0;
    for val in view.iter() {
        if *val < height {
            score += 1;
        } else {
            score += 1;
            return score;
        }
    }
    score
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 21.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 8.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2022, 8)?;
        assert_eq!(super::part_1(input.as_str())?, 1669.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2022, 8)?;
        assert_eq!(super::part_2(input.as_str())?, 331344.into());
        Ok(())
    }
}
