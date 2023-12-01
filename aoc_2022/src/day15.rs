use framework::boilerplate;
use framework::vec::Coord2d;
use framework::IResult;
use framework::SolutionData;
use itertools::iproduct;
use itertools::Itertools;
use std::collections::HashSet;

boilerplate!(
    Day,
    15,
    "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
",
    "data/15.txt"
);

#[derive(Clone, Debug)]
struct Sensor {
    loc: Coord2d,
    closest_beacon: Coord2d,
    distance: i32,
}

impl Sensor {
    pub fn contains(&self, coord: &Coord2d) -> bool {
        //if *coord == self.closest_beacon {
        //    return false;
        //}
        self.loc.manhattan_distance(coord) <= self.distance
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

impl Solution for Day {
    type Parsed = Vec<Sensor>;
    type Answer = usize;
    const EXAMPLE_ANSWER_1: Self::Answer = 26;
    const ANSWER_1: Self::Answer = 5142231;
    const EXAMPLE_ANSWER_2: Self::Answer = 56000011;
    const ANSWER_2: Self::Answer = 10884459367718;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let sensors = input
            .lines()
            .map(|line| {
                let (sensor, beacon) = line.split_once(':').unwrap();
                let loc = sensor
                    .replace("Sensor at x=", "")
                    .replace("y=", "")
                    .trim()
                    .split_once(", ")
                    .map(|(x, y)| {
                        Coord2d::from_coords(
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
                        Coord2d::from_coords(
                            x.parse::<i32>().expect(""),
                            y.parse::<i32>().expect(""),
                        )
                    })
                    .unwrap();
                let distance = loc.manhattan_distance(&closest_beacon);
                Sensor {
                    loc,
                    closest_beacon,
                    distance,
                }
            })
            .collect_vec();
        Ok(("", sensors))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        solve_part1::<2_000_000>(&input)
    }

    fn part2(input: Self::Parsed) -> Self::Answer {
        solve_part2::<4_000_000>(&input)
    }
}

fn solve_part1<const Y: isize>(input: &[Sensor]) -> usize {
    let fsensor = &input[0];
    let init_range = fsensor.loc.x - fsensor.distance..fsensor.loc.x + fsensor.distance;
    let x_bounds = input.iter().fold(init_range, |mut range, sensor| {
        range.start = range.start.min(sensor.loc.x - sensor.distance);
        range.end = range.end.max(sensor.loc.x + sensor.distance);
        range
    });
    println!("x to try: {} .. {}", x_bounds.start, x_bounds.end);

    (x_bounds.start..=x_bounds.end)
        .map(|x| Coord2d::from_coords(x, Y as i32))
        .map(|pos| {
            input
                .iter()
                .map(move |s| {
                    s.loc.manhattan_distance(&pos) <= s.distance && pos != s.closest_beacon
                })
                .any(|f| f) as usize
        })
        .sum()
}

fn solve_part2<const N: i32>(input: &[Sensor]) -> usize {
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
        .map(|(a, b)| Coord2d::from_coords((b - a) / 2, (a + b) / 2))
        .filter(|p| 0 <= p.x && p.x <= N && 0 <= p.y && p.y <= N)
        .filter(|p| input.iter().all(|sen| !sen.contains(p)))
        .collect_vec();

    assert!(possible_pts.len() == 1);
    possible_pts[0].x as usize * 4_000_000_usize + possible_pts[0].y as usize
}
