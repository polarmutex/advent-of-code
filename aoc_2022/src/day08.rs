use ahash::AHashMap;
use ahash::AHashSet;
use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;

boilerplate!(
    Day,
    8,
    "\
30373
25512
65332
33549
35390
",
    "data/08.txt"
);

#[derive(Debug, Clone)]
struct Input {
    tree_grid: Vec<Vec<u8>>,
}

impl Solution for Day {
    type Parsed = Input;
    type Answer = usize;
    const EXAMPLE_ANSWER_1: Self::Answer = 21;
    const ANSWER_1: Self::Answer = 1669;
    const EXAMPLE_ANSWER_2: Self::Answer = 8;
    const ANSWER_2: Self::Answer = 331344;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let num_rows = input.lines().count();
        let num_cols = input.lines().next().unwrap().chars().count();
        let mut tree_grid = vec![vec![0; num_cols]; num_rows];
        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, val)| {
                println!("{} {} {}", x, y, val);
                tree_grid[y][x] = val.to_string().parse::<u8>().expect("u8 number")
            })
        });
        Ok(("", Input { tree_grid }))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        let map = input.tree_grid.clone();
        print_grid(&map);

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
                    println!("{} {} ", x, y);
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

    fn part2(input: Self::Parsed) -> Self::Answer {
        let map = input.tree_grid.clone();
        print_grid(&map);

        let x_len = map[0].len();
        let y_len = map.len();

        let mut vis = AHashMap::<(usize, usize), usize>::new();

        // range goes to last num - 1
        for y in 0..y_len {
            for x in 0..x_len {
                println!("{} {} ", x, y);
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
}

fn print_grid(i: &Vec<Vec<u8>>) {
    println!("len y: {} x: {}", i.len(), i[0].len());
    for y in 0..i.len() {
        for x in 0..i[0].len() {
            print!("{} ", i[y][x]);
        }
        println!();
        println!();
    }
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
