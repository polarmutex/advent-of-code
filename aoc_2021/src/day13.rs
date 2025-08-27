


use aoc_runner_macros::{aoc, generator, solver, solution};

use ndarray::{s, Array2, Axis, Zip};
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, newline, u32},
    multi::separated_list1,
    sequence::separated_pair,
    IResult
};

type Input = (Array2<Mark>, Vec<Fold>);

#[derive(Debug, Clone)]
pub enum Mark {
    Marked,
    UnMarked,
}

#[derive(Debug, Clone)]
pub enum Fold {
    Row(u32),
    Column(u32),
}

#[aoc(2021, day13)]
pub mod solutions {
    use super::*;
impl Fold {
    fn apply_to(&self, dots: &Array2<Mark>) -> Array2<Mark> {
        dbg!(&self);
        let (fold_axis, idx) = match self {
            Fold::Row(idx) => (Axis(0), idx),
            Fold::Column(idx) => (Axis(1), idx),
        };

        let folding_axis_length = dots.len_of(fold_axis);
        // dbg!(folding_axis_length);
        let skip_amount = folding_axis_length % 2;
        let (range_a, range_b) = (
            0..*idx as usize,
            (*idx as usize + skip_amount)..folding_axis_length,
        );
        // dbg!(&range_a, &range_b);

        let matrix_a = dots.slice(match self {
            Fold::Row(_) => s!(range_a, ..),
            Fold::Column(_) => s!(.., range_a),
        });
        let mut matrix_b = dots.slice(match self {
            Fold::Row(_) => s!(range_b, ..),
            Fold::Column(_) => s!(.., range_b),
        });
        matrix_b.invert_axis(fold_axis);

        // union matrix a + b
        let smol = Zip::from(matrix_a)
            .and(matrix_b)
            .map_collect(|a, b| match (a, b) {
                (Mark::Marked, _) | (_, Mark::Marked) => Mark::Marked,
                (Mark::UnMarked, Mark::UnMarked) => Mark::UnMarked,
            });
        // dbg!(smol.shape());
        smol
    }
}

fn dots(input: &str) -> IResult<&str, Array2<Mark>> {
    let (input, outputs) = separated_list1(newline, separated_pair(u32, tag(","), u32))(input)?;

    let max_x = outputs.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_y = outputs.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    let nrows = max_y + 1;
    let ncols = max_x + 1;

    let data = vec![Mark::UnMarked; (nrows * ncols) as usize];

    let mut dots = Array2::from_shape_vec((nrows as usize, ncols as usize), data).unwrap();
    for (x, y) in outputs.iter() {
        // dbg!(x, y);
        let point = dots.get_mut((*y as usize, *x as usize)).unwrap();
        // dbg!(point);
        *point = Mark::Marked;
    }

    Ok((input, dots))
}

fn fold(input: &str) -> IResult<&str, Fold> {
    let (input, _) = tag("fold along ")(input)?;
    let (input, (axis, num)) = separated_pair(anychar, tag("="), u32)(input)?;
    Ok((
        input,
        match axis {
            'y' => Fold::Row(num),
            'x' => Fold::Column(num),
            _ => panic!("not x or y for fold"),
        },
    ))
}

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (input, parsed_dots) = dots(input).unwrap();
        let (input, _) = newline::<&str, nom::error::Error<&str>>(input).unwrap();
        let (input, _) = newline::<&str, nom::error::Error<&str>>(input).unwrap();
        let (_, parsed_folds) = separated_list1(newline, fold)(input).unwrap();

        (parsed_dots, parsed_folds)
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> u32 {
        let mut it = input.1.iter();
        let operation = it.next().unwrap();
        let smol_matrix = operation.apply_to(&input.0);
        let count: u32 = smol_matrix
            .iter()
            .map(|point| match point {
                Mark::Marked => 1,
                Mark::UnMarked => 0,
            })
            .sum();
        count
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> u32 {
        let smol_matrix =
            input
                .1
                .iter()
                .enumerate()
                .fold(input.0.clone(), |dots, (_i, operation)| {
                    let smol = operation.apply_to(&dots);
                    smol
                });

        println!("-----FINAL-----");
        for row in smol_matrix.rows() {
            println!(
                "{}",
                row.iter()
                    .map(|point| match point {
                        Mark::Marked => "█",
                        Mark::UnMarked => " ",
                    })
                    .collect::<String>()
            );
        }
        0 // PZEHRAER
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> u32 {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> u32 {
        let data = input_generator(input);
        solve_part2(&data)
    }
}

#[cfg(test)]
mod tests {


    const EXAMPLE: &str = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

    #[test]
    fn test_part_1() {
        let _input = super::solutions::input_generator(EXAMPLE);
    }

    #[test]
    fn test_part_2() {
        let _input = super::solutions::input_generator(EXAMPLE);
    }
}
