use crate::prelude::*;

day!(15, parse => part1::<2_000_000>, part2::<4_000_000>);

struct Sensor {
    location: Coord2d<isize>,
    closest_beacon: Coord2d<isize>,
    distance: isize,
}

impl Sensor {
    pub fn contains(&self, coord: &Coord2d<isize>) -> bool {
        if *coord == self.closest_beacon {
            return false;
        }
        self.location.manhattan_distance(coord) <= self.distance
    }
}

impl std::fmt::Display for Sensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Sensor loc: {} - Closest Beacon: {} - dist: {}",
            self.location, self.closest_beacon, self.distance
        )
    }
}

fn parse(input: &str) -> ParseResult<Vec<Sensor>> {
    let sensors = input
        .lines()
        .map(|line| {
            let (sensor, beacon) = line.split_once(':').unwrap();
            let location = sensor
                .replace("Sensor at x=", "")
                .replace("y=", "")
                .trim()
                .split_once(", ")
                .map(|(x, y)| {
                    Coord2d::from((x.parse::<isize>().expect(""), y.parse::<isize>().expect("")))
                })
                .unwrap();
            let closest_beacon = beacon
                .replace("closest beacon is at x=", "")
                .replace("y=", "")
                .trim()
                .split_once(", ")
                .map(|(x, y)| {
                    Coord2d::from((x.parse::<isize>().expect(""), y.parse::<isize>().expect("")))
                })
                .unwrap();
            let distance = location.manhattan_distance(&closest_beacon);
            Sensor {
                location,
                closest_beacon,
                distance,
            }
        })
        .collect_vec();
    Ok(sensors)
}

fn part1<const Y: isize>(input: &[Sensor]) -> usize {
    let fsensor = &input[0];
    let init_range = fsensor.location.x - fsensor.distance..fsensor.location.x + fsensor.distance;
    let x_bounds = input.iter().fold(init_range, |mut range, sensor| {
        range.start = range.start.min(sensor.location.x - sensor.distance);
        range.end = range.end.max(sensor.location.x + sensor.distance);
        range
    });
    println!("x to try: {} .. {}", x_bounds.start, x_bounds.end);

    (x_bounds.start..=x_bounds.end)
        .filter(|x| {
            input
                .iter()
                .any(|sensor| sensor.contains(&Coord2d::from((*x, Y))))
        })
        .count()
}

fn part2<const N: isize>(input: &[Sensor]) -> usize {
    let point: Coord2d<isize> = input
        .iter()
        .find_map(|s| {
            ((s.location.x - s.distance - 1).max(0)..=s.location.x.min(N))
                .zip(s.location.y..=N)
                .find_map(|pt| {
                    input
                        .iter()
                        .all(|s| !s.contains(&Coord2d::from(pt)))
                        .then(|| Coord2d::from(pt))
                })
        })
        .unwrap();
    (point.x * 4_000_000 + point.y) as usize
}

tests! {
    const EXAMPLE: &str = "\
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
";
    const INPUT: &str = include_str!("data/15.txt");

    simple_tests!(parse, part1::<10>, part1_example_test, EXAMPLE => 26);
    simple_tests!(parse, part1::<2_000_000>, part1_input_test, INPUT => 5142231);
    simple_tests!(parse, part2::<20>, part2_example_test, EXAMPLE => 56000011);
    simple_tests!(parse, part2::<4_000_000>, part2_input_test, INPUT => 10884459367718);
}
