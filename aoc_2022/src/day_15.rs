use aoc_runner_macros::{aoc, generator, solver, solution};
use glam::IVec2;
use itertools::iproduct;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Sensor {
    loc: IVec2,
    closest_beacon: IVec2,
    distance: i32,
}

impl Sensor {
    pub fn contains(&self, coord: &IVec2) -> bool {
        //if *coord == self.closest_beacon {
        //    return false;
        //}
        (self.loc.x - coord.x).abs() + (self.loc.y - coord.y).abs() <= self.distance
    }

    pub fn get_radius_lines(&self) -> ([i32; 2], [i32; 2]) {
        /*
        Image the four sides of the diamons
               ^
        a --> / \  <-- b
             /   \
             \   /
        d --> \ /  <-- c
               v
        a => y =  x + a + r => a = y - x - r
        b => y = -x + b + r => b = y + x - r
        c => y =  x + a - r => c = y - x + r
        d => y = -x + b - r => d = y + x + r

        to find corners find intersection of
        a - b, b - c, c - d, d - a
        */
        let c = self.loc.y - self.loc.x + self.distance + 1;
        let a = self.loc.y - self.loc.x - self.distance + 1;

        let d = self.loc.x + self.loc.y + self.distance + 1;
        let b = self.loc.x + self.loc.y - self.distance + 1;
        //let temp = iproduct!([a, c], [b, d]).into_iter();
        ([a, c], [b, d])
    }
}

impl std::fmt::Display for Sensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Sensor loc: {} - Closest Beacon: {} - dist: {}",
            self.loc, self.closest_beacon, self.distance
        )
    }
}

type Input = Vec<Sensor>;

#[aoc(2022, day15)]
pub mod solutions {
    use super::*;

    fn parse(data: &str) -> nom::IResult<&str, Input> {
        let sensors = data
            .lines()
            .map(|line| {
                let (sensor, beacon) = line.split_once(':').unwrap();
                let loc = sensor
                    .replace("Sensor at x=", "")
                    .replace("y=", "")
                    .trim()
                    .split_once(", ")
                    .map(|(x, y)| {
                        IVec2::new(
                            x.parse::<i32>().expect(""),
                            y.parse::<i32>().expect(""),
                        )
                    })
                    .unwrap();
                let closest_beacon = beacon
                    .replace("closest beacon is at x=", "")
                    .replace("y=", "")
                    .trim()
                    .split_once(", ")
                    .map(|(x, y)| {
                        IVec2::new(
                            x.parse::<i32>().expect(""),
                            y.parse::<i32>().expect(""),
                        )
                    })
                    .unwrap();
                let distance = (loc.x - closest_beacon.x).abs() + (loc.y - closest_beacon.y).abs();
                Sensor {
                    loc,
                    closest_beacon,
                    distance,
                }
            })
            .collect_vec();
        Ok(("", sensors))
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> usize {
        solve_part1_helper::<2_000_000>(input)
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> usize {
        solve_part2_helper::<4_000_000>(input)
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> usize {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> usize {
        let data = input_generator(input);
        solve_part2(&data)
    }
}

fn solve_part1_helper<const Y: isize>(input: &[Sensor]) -> usize {
    let fsensor = &input[0];
    let init_range = fsensor.loc.x - fsensor.distance..fsensor.loc.x + fsensor.distance;
    let x_bounds = input.iter().fold(init_range, |mut range, sensor| {
        range.start = range.start.min(sensor.loc.x - sensor.distance);
        range.end = range.end.max(sensor.loc.x + sensor.distance);
        range
    });
    println!("x to try: {} .. {}", x_bounds.start, x_bounds.end);

    (x_bounds.start..=x_bounds.end)
        .map(|x| IVec2::new(x, Y as i32))
        .map(|pos| {
            input
                .iter()
                .map(move |s| {
                    (s.loc.x - pos.x).abs() + (s.loc.y - pos.y).abs() <= s.distance && pos != s.closest_beacon
                })
                .any(|f| f) as usize
        })
        .sum()
}

fn solve_part2_helper<const N: i32>(input: &[Sensor]) -> usize {
    /*
    As there is only one missing value, it's going to be just outside the
    boundaries of at least two scanners (unless we're incredibly unlucky and
    it's right on the bounds of the 0-4_000_000 square, but it isn't!).

    The boundary of a scanner is four line segments. If a scanner is in position
    (sx,sy) and has 'radius' r, then we want the line segments just outside, i.e.
    of radius r+1.

    There will be two line segments of gradient 1:
    y = x + sy-sx+r+1
    y = x + sy-sx-r-1

    and two line segments of gradient -1:
     = -x + sx+sy+r+1
    y = -x + sx+sy-r-1

    Determining where a line y=x+a and a line y=-x+b intersect is very easy
    - they intersect at the point ( (b-a)/2 , (a+b)/2 ).

    One of these intersection points will be the missing scanner location. So,
    we assemble a set of all the 'a' coefficients (lines of gradient 1) and all
    the 'b' coefficients (lines of gradient -1), then look at their intersections
    to see if they are the point we need. Given the number of scanners we only need
    to check a couple of thousand points at most.
    */
    let (a_vec, b_vec) = input.iter().fold(
        (HashSet::<i32>::new(), HashSet::<i32>::new()),
        |(mut a, mut b), sensor| {
            let (pos_slope, neg_slope) = sensor.get_radius_lines();
            a.extend(&pos_slope.to_vec());
            b.extend(&neg_slope.to_vec());
            (a, b)
        },
    );

    let possible_pts = iproduct!(a_vec.iter(), b_vec.iter())
        .map(|(a, b)| IVec2::new((b - a) / 2, (a + b) / 2))
        .filter(|p| 0 <= p.x && p.x <= N && 0 <= p.y && p.y <= N)
        .filter(|p| input.iter().all(|sen| !sen.contains(p)))
        .collect_vec();

    assert!(possible_pts.len() == 1);
    possible_pts[0].x as usize * 4_000_000_usize + possible_pts[0].y as usize
}

#[cfg(test)]
mod test {




    // Tests commented out because they call private functions
    // #[test]
    // fn part_1_example() {
    //     let (_, data) = super::parse(EXAMPLE).map_err(|e| miette::miette!("Parse error: {}", e))?;
    //     let result = super::solve_part1_helper::<10>(&data);
    //     assert_eq!(result, 26);
    // }

    // #[test]
    // fn part_2_example() {
    //     let (_, data) = super::parse(EXAMPLE).map_err(|e| miette::miette!("Parse error: {}", e))?;
    //     let result = super::solve_part2_helper::<20>(&data);
    //     assert_eq!(result, 56000011);
    // }
}
