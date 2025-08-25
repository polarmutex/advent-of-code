use common::{solution, Answer};
use nom::IResult;
use std::iter::{self};

use ndarray::{concatenate, Array2, Axis};
use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    sequence::separated_pair,
};

solution!("Trench Map", 20);

fn newlines(input: &str) -> IResult<&str, ()> {
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    Ok((input, ()))
}
fn image(input: &str) -> IResult<&str, Array2<char>> {
    let (input, raw_input) = separated_list1(newline, many1(one_of("#.")))(input)?;

    let nrows = raw_input.len();
    let ncols = raw_input[0].len();

    let image = Array2::from_shape_vec(
        (nrows, ncols),
        raw_input.into_iter().flatten().collect::<Vec<char>>(),
    )
    .unwrap();

    Ok((input, image))
}

fn new_pad_char(algo: &Vec<char>, c: char) -> char {
    let char_string = iter::repeat(c)
        .take(6)
        .map(|c| match c {
            '#' => "1",
            '.' => "0",
            _ => panic!("asfkglj"),
        })
        .collect::<String>();
    let num = usize::from_str_radix(&char_string, 2).expect("a valid parse");
    *algo.get(num).expect("a valid index")
}

fn pad_array(original: &Array2<char>, zero_char: char) -> Array2<char> {
    let pad_axis_1 = Array2::from_elem((original.len_of(Axis(0)), 1), zero_char);

    let padded_axis_1 = concatenate(
        Axis(1),
        &[
            // pad_axis_1.view(),
            pad_axis_1.view(),
            pad_axis_1.view(),
            original.view(),
            // pad_axis_1.view(),
            pad_axis_1.view(),
            pad_axis_1.view(),
        ],
    )
    .unwrap();

    let pad_axis_0 = Array2::from_elem((1, padded_axis_1.len_of(Axis(1))), zero_char);

    let padded_axis_0 = concatenate(
        Axis(0),
        &[
            // pad_axis_0.view(),
            pad_axis_0.view(),
            pad_axis_0.view(),
            padded_axis_1.view(),
            // pad_axis_0.view(),
            pad_axis_0.view(),
            pad_axis_0.view(),
        ],
    )
    .unwrap();
    padded_axis_0
}

fn process(image: &Array2<char>, algo: &Vec<char>, pad_char: char) -> Array2<char> {
    let padded_image = pad_array(image, pad_char);
    let processed_image = padded_image
        .windows((3, 3))
        .into_iter()
        .map(|elems| {
            let string_num = elems
                .iter()
                .map(|item| match item {
                    '#' => "1",
                    '.' => "0",
                    _ => panic!("input wasn't correct"),
                })
                .collect::<String>();
            let num = usize::from_str_radix(&string_num, 2).expect("a valid parse");
            algo.get(num).expect("a valid index")
        })
        .cloned()
        .collect::<Vec<char>>();
    Array2::from_shape_vec(
        (
            padded_image.len_of(Axis(0)) - 2,
            padded_image.len_of(Axis(1)) - 2,
        ),
        processed_image,
    )
    .unwrap()
}

fn big_pad_array(original: &Array2<char>, num_iters: usize) -> Array2<char> {
    let pad_axis_1 = Array2::from_elem((original.len_of(Axis(0)), num_iters + 1), '.');

    let padded_axis_1 = concatenate(
        Axis(1),
        &[pad_axis_1.view(), original.view(), pad_axis_1.view()],
    )
    .unwrap();

    let pad_axis_0 = Array2::from_elem((num_iters + 1, padded_axis_1.len_of(Axis(1))), '.');

    let padded_axis_0 = concatenate(
        Axis(0),
        &[pad_axis_0.view(), padded_axis_1.view(), pad_axis_0.view()],
    )
    .unwrap();
    padded_axis_0
}

fn big_process(image: &Array2<char>, algo: &Vec<char>) -> Array2<char> {
    let processed_image = image
        .windows((3, 3))
        .into_iter()
        .map(|elems| {
            let string_num = elems
                .iter()
                .map(|item| match item {
                    '#' => "1",
                    '.' => "0",
                    _ => panic!("input wasn't correct"),
                })
                .collect::<String>();
            let num = usize::from_str_radix(&string_num, 2).expect("a valid parse");
            algo.get(num).expect("a valid index")
        })
        .cloned()
        .collect::<Vec<char>>();

    Array2::from_shape_vec(
        (image.len_of(Axis(0)) - 2, image.len_of(Axis(1)) - 2),
        processed_image,
    )
    .unwrap()
}

fn parse(input: &str) -> IResult<&str, (Vec<char>, Array2<char>)> {
        let (input, (algo, image)) = separated_pair(many1(one_of(".#")), newlines, image)(input)?;
    Ok((input, (algo, image)))
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, (algo, image)) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
        let new_image = process(&image, &algo, '.');
        let pad_char = new_pad_char(&algo, '.');
        let new_image = process(&new_image, &algo, pad_char);
        let count = new_image
            .iter()
            .filter(|v| match v {
                '#' => true,
                _ => false,
            })
            .count();
        Ok((count as u64).into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, (algo, image)) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
        let grid = big_pad_array(&image, 50);

        let mut new_image = grid.clone();

        for _ in 0..50 {
            new_image = big_process(&new_image, &algo);
            new_image = big_pad_array(&new_image, 0);
        }

        let count = new_image
            .iter()
            .filter(|v| match v {
                '#' => true,
                _ => false,
            })
            .count();
        Ok((count as u64).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE).unwrap(), Answer::Number(35));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE).unwrap(), Answer::Number(3351));
    }
}
