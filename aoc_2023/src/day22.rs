use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
use glam::IVec3;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult as IBaseResult;
use nom::Parser;
use nom_locate::LocatedSpan;
use std::collections::HashMap;
use std::collections::HashSet;
// use nom_supreme::ParserExt;
// use tracing::info;

boilerplate!(
    Day,
    22,
    "\
",
    "data/22.txt"
);

#[derive(Clone, Debug)]
struct Brick {
    start: IVec3,
    end: IVec3,
}

fn parse_brick_part(input: &str) -> IResult<IVec3> {
    let (input, x) = complete::i32.parse(input)?;
    let (input, _) = complete::char(',').parse(input)?;
    let (input, y) = complete::i32.parse(input)?;
    let (input, _) = complete::char(',').parse(input)?;
    let (input, z) = complete::i32.parse(input)?;
    Ok((input, IVec3::new(x, y, z)))
}

fn parse_brick(input: &str) -> IResult<Brick> {
    let (input, (start, end)) =
        separated_pair(parse_brick_part, complete::char('~'), parse_brick_part)(input)?;

    // let cubes = [start.x..=end.x, start.y..=end.y, start.z..=end.z]
    //     .into_iter()
    //     .multi_cartesian_product()
    //     .map(|cube| IVec3::new(cube[0], cube[1], cube[2]))
    //     .collect();
    Ok((input, Brick { start, end }))
}

impl Solution for Day {
    type Parsed = Vec<Brick>;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    #[tracing::instrument(skip(input))]
    fn parse(input: &str) -> IResult<Self::Parsed> {
        separated_list1(line_ending, parse_brick).parse(input)
    }

    #[tracing::instrument(skip(input))]
    fn part1(mut input: Self::Parsed) -> Self::Answer {
        input.sort_by_key(|b| b.start.z);
        dbg!(&input);
        let mut below = vec![HashSet::new(); input.len()];
        let mut above = vec![HashSet::new(); input.len()];
        let mut space = HashMap::new();
        for (i, mut b) in input.iter_mut().enumerate() {
            let x1 = b.start.x;
            let y1 = b.start.y;
            let mut z1 = b.start.z;
            let x2 = b.end.x;
            let y2 = b.end.y;
            let mut z2 = b.end.z;
            while z1 > 1
                && (x1..=x2)
                    .cartesian_product(y1..=y2)
                    .all(|(x, y)| !space.contains_key(&(x, y, z1 - 1)))
            {
                z2 -= 1;
                z1 -= 1;
            }
            for (x, y) in (x1..=x2).cartesian_product(y1..=y2) {
                for z in z1..=z2 {
                    space.insert((x, y, z), i);
                }
                if let Some(&j) = space.get(&(x, y, z1 - 1)) {
                    above[j].insert(i);
                    below[i].insert(j);
                }
            }
            b.start.x = x1;
            b.start.y = y1;
            b.start.z = z1;
            b.end.x = x2;
            b.end.y = y2;
            b.end.z = z2;
        }

        let mut falling = HashSet::new();
        let (mut p1, mut p2) = (0, 0);
        for b in 0..input.len() {
            falling.clear();
            disintegrate_all(&below, &above, &mut falling, b);
            p1 += (falling.len() == 1) as usize;
            p2 += falling.len() - 1;
        }
        p1 as u32
    }

    #[tracing::instrument(skip(input))]
    fn part2(mut input: Self::Parsed) -> Self::Answer {
        input.sort_by_key(|b| b.start.z);
        dbg!(&input);
        let mut below = vec![HashSet::new(); input.len()];
        let mut above = vec![HashSet::new(); input.len()];
        let mut space = HashMap::new();
        for (i, b) in input.iter().enumerate() {
            let mut z_start = b.start.z;
            let mut z_end = b.end.z;
            while z_start > 1
                && (b.start.x..=b.end.x)
                    .cartesian_product(b.start.y..=b.end.y)
                    .all(|(x, y)| !space.contains_key(&(x, y, z_start - 1)))
            {
                z_start -= 1;
                z_end -= 1;
            }
            for (x, y) in (b.start.x..=b.end.x).cartesian_product(b.start.y..=b.end.y) {
                for z in z_start..=z_end {
                    space.insert((x, y, z), i);
                }
                if let Some(&j) = space.get(&(x, y, z_start - 1)) {
                    above[j].insert(i);
                    below[i].insert(j);
                }
            }
            // b.start.x = x1;
            // b.start.y = y1;
            // b.start.z = z1;
            // b.end.x = x2;
            // b.end.y = y2;
            // b.end.z = z2;
        }

        let mut falling = HashSet::new();
        let (mut p1, mut p2) = (0, 0);
        for b in 0..input.len() {
            falling.clear();
            disintegrate_all(&below, &above, &mut falling, b);
            p1 += (falling.len() == 1) as usize;
            p2 += falling.len() - 1;
        }
        p2 as u32
    }
}

fn disintegrate_all(
    below: &[HashSet<usize>],
    above: &[HashSet<usize>],
    falling: &mut HashSet<usize>,
    i: usize,
) {
    falling.insert(i);
    for &b in &above[i] {
        if below[b].iter().all(|x| falling.contains(x)) {
            disintegrate_all(below, above, falling, b);
        }
    }
}

tests! {
     const EXAMPLE: &str = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

    add_test!(part1_example, test_part1_example, EXAMPLE => 5);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 411);
    add_test!(part2_example, test_part2_example, EXAMPLE => 7);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 47671);
}
