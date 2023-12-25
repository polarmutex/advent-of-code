use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
use glam::I64Vec3;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    Parser,
};
use z3::ast::{Ast, Int};
// use nom_supreme::ParserExt;
// use tracing::info;

boilerplate!(
    Day,
    24,
    "\
",
    "data/24.txt"
);

#[derive(Clone, Debug)]
struct Hail {
    start: I64Vec3,
    direction: I64Vec3,
}

fn intersection(a: &Hail, b: &Hail) -> Option<(f64, f64)> {
    let x1 = a.start.x as f64;
    let y1 = a.start.y as f64;
    let dx1 = a.direction.x as f64;
    let dy1 = a.direction.y as f64;

    let x2 = b.start.x as f64;
    let y2 = b.start.y as f64;
    let dx2 = b.direction.x as f64;
    let dy2 = b.direction.y as f64;

    let m1 = dy1 / dx1;
    let m2 = dy2 / dx2;
    if (m2 - m1).abs() < f64::EPSILON {
        return None;
    }
    let x = (m1 * x1 - m2 * x2 + y2 - y1) / (m1 - m2);
    let y = (m1 * m2 * (x2 - x1) + m2 * y1 - m1 * y2) / (m2 - m1);
    Some((x, y))
}

fn parse_coords(input: &str) -> IResult<I64Vec3> {
    let (input, x) = complete::i64.parse(input)?;
    let (input, _) = terminated(tag(","), space1).parse(input)?;
    let (input, y) = complete::i64(input)?;
    let (input, _) = terminated(tag(","), space1).parse(input)?;
    let (input, z) = complete::i64(input)?;

    Ok((input, I64Vec3::new(x, y, z)))
}

impl Solution for Day {
    type Parsed = Vec<Hail>;
    type Answer = u64;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    #[tracing::instrument(skip(input))]
    fn parse(input: &str) -> IResult<Self::Parsed> {
        separated_list1(
            line_ending,
            separated_pair(
                parse_coords,
                delimited(space1, tag("@"), space1),
                parse_coords,
            )
            .map(|(start, direction)| Hail { start, direction }),
        )
        .parse(input)
    }

    #[tracing::instrument(skip(input))]
    fn part1(input: Self::Parsed) -> Self::Answer {
        dbg!(&input);
        let range = 200000000000000.0..=400000000000000.0;
        input
            .iter()
            .tuple_combinations()
            .filter(|(a, b)| {
                let Some((x, y)) = intersection(a, b) else {
                    return false;
                };
                if a.direction.x.signum() as f64 != (x - a.start.x as f64).signum()
                    || b.direction.x.signum() as f64 != (x - b.start.x as f64).signum()
                {
                    return false;
                }
                range.contains(&x) && range.contains(&y)
            })
            .count() as u64
    }

    #[tracing::instrument(skip(input))]
    fn part2(input: Self::Parsed) -> Self::Answer {
        let ctx = z3::Context::new(&z3::Config::new());
        let s = z3::Solver::new(&ctx);
        let [fx, fy, fz, fdx, fdy, fdz] =
            ["fx", "fy", "fz", "fdx", "fdy", "fdz"].map(|v| Int::new_const(&ctx, v));

        let zero = Int::from_i64(&ctx, 0);
        for (i, hail) in input.iter().enumerate() {
            let x = hail.start.x;
            let y = hail.start.y;
            let z = hail.start.z;
            let dx = hail.direction.x;
            let dy = hail.direction.y;
            let dz = hail.direction.z;
            let [x, y, z, dx, dy, dz] = [x, y, z, dx, dy, dz].map(|v| Int::from_i64(&ctx, v as _));
            let t = Int::new_const(&ctx, format!("t{i}"));
            s.assert(&t.ge(&zero));
            s.assert(&((&x + &dx * &t)._eq(&(&fx + &fdx * &t))));
            s.assert(&((&y + &dy * &t)._eq(&(&fy + &fdy * &t))));
            s.assert(&((&z + &dz * &t)._eq(&(&fz + &fdz * &t))));
        }
        assert_eq!(s.check(), z3::SatResult::Sat);
        let model = s.get_model().unwrap();
        let res = model.eval(&(&fx + &fy + &fz), true).unwrap();
        dbg!(res.as_i64().unwrap());
        res.as_i64().unwrap() as u64
    }
}

tests! {
     const EXAMPLE: &str = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

    // add_test!(part1_example, test_part1_example, EXAMPLE => 2);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 11098);
    // add_test!(part2_example, test_part2_example, EXAMPLE => 47);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 920630818300104);
}
