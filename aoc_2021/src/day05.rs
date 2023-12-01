use framework::boilerplate;
use framework::line::Line;
use framework::vec::Coord2d;
use framework::IResult;
use framework::SolutionData;

boilerplate!(
    Day,
    5,
    "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
    "data/05.txt"
);

fn count_overlap_pts<'i, I: Iterator<Item = &'i Line> + Clone + 'i>(lines: I) -> u32 {
    let mut board_size: Coord2d = lines.clone().flat_map(|line| [line.from, line.to]).fold(
        Coord2d::from_coords(0, 0),
        |mut size, coord| {
            size.x = size.x.max(coord.x);
            size.y = size.x.max(coord.y);
            size
        },
    );
    // to account for zero based indexing
    board_size.x += 1;
    board_size.y += 1;
    // Initialize 2d vector
    let mut board = vec![vec![0; board_size.y as usize]; board_size.x as usize];
    for line in lines {
        for point in line {
            let x = point.x as usize;
            let y = point.y as usize;
            board[x][y] += 1;
        }
    }
    let mut count = 0;
    for x in 0..board_size.x {
        for y in 0..board_size.y {
            if board[x as usize][y as usize] > 1 {
                count += 1;
            }
        }
    }
    count
}
impl Solution for Day {
    type Parsed = Vec<Line>;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 5;
    const ANSWER_1: Self::Answer = 7269;
    const EXAMPLE_ANSWER_2: Self::Answer = 12;
    const ANSWER_2: Self::Answer = 21140;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        //Ok(input_parser().parse(input).unwrap())
        let lines: Vec<Line> = input
            .lines()
            .map(|line| {
                line.split_once(" -> ")
                    .map(|(from, to)| Line {
                        from: from
                            .split_once(',')
                            .map(|(x, y)| {
                                Coord2d::from_coords(
                                    x.parse::<i32>().expect(""),
                                    y.parse::<i32>().expect(""),
                                )
                            })
                            .expect(""),
                        to: to
                            .split_once(',')
                            .map(|(x, y)| {
                                Coord2d::from_coords(
                                    x.parse::<i32>().expect(""),
                                    y.parse::<i32>().expect(""),
                                )
                            })
                            .expect(""),
                    })
                    .expect("")
            })
            .collect();
        Ok(("", lines))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        count_overlap_pts(
            input
                .iter()
                .filter(|line| line.is_horizontal() || line.is_vertical()),
        )
    }

    fn part2(input: Self::Parsed) -> Self::Answer {
        count_overlap_pts(input.iter())
    }
}
