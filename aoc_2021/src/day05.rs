use crate::prelude::*;

day!(5, parse => part1, part2);

fn input_parser() -> impl Parser<char, Vec<Line>, Error = Simple<char>> {
    let number = c::text::int(10).map(|s: String| s.parse().unwrap());
    let point = (number.then_ignore(just(',')).then(number)).map(Vec2::from_coords);
    let arrow = (just(' ').then(just('-')).then(just('>')).then(just(' '))).ignored(); // TODO: replace with 'keyword'
    let vent_line = (point.then_ignore(arrow).then(point)).map(Line::between_points);
    vent_line.separated_by(c::text::newline())
}

fn parse(input: &str) -> ParseResult<Vec<Line>> {
    //Ok(input_parser().parse(input).unwrap())
    let lines: Vec<Line> = input
        .lines()
        .map(|line| {
            line.split_once(" -> ")
                .map(|(from, to)| Line {
                    from: from
                        .split_once(',')
                        .map(|(x, y)| (x.parse::<i32>().expect(""), y.parse::<i32>().expect("")))
                        .map(Vec2::from_coords)
                        .expect(""),
                    to: to
                        .split_once(',')
                        .map(|(x, y)| (x.parse::<i32>().expect(""), y.parse::<i32>().expect("")))
                        .map(Vec2::from_coords)
                        .expect(""),
                })
                .expect("")
        })
        .collect();
    Ok(lines)
}

fn count_overlap_pts<'i, I: Iterator<Item = &'i Line> + Clone + 'i>(lines: I) -> u32 {
    let mut board_size: Vec2<u32> = lines
        .clone()
        .flat_map(|line| [line.from, line.to])
        .fold((0, 0), |(x, y), coord| {
            (x.max(coord.x as u32), y.max(coord.y as u32))
        })
        .into();
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

fn part1(input: &[Line]) -> u32 {
    count_overlap_pts(
        input
            .iter()
            .filter(|line| line.is_horizontal() || line.is_vertical()),
    )
}

fn part2(input: &[Line]) -> u32 {
    count_overlap_pts(input.iter())
}

tests! {
    const EXAMPLE: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    const INPUT: &str = include_str!("../../data/2021/05.txt");

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 5);
    simple_tests!(parse, part1, part1_input_test, INPUT => 7269);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 12);
    simple_tests!(parse, part2, part2_input_test, INPUT => 21140);
}
