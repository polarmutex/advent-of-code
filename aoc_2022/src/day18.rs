use framework::boilerplate;
use framework::vec::Coord3d;
use framework::IResult;
use framework::SolutionData;
use std::collections::HashSet;

boilerplate!(
    Day,
    18,
    "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
",
    "data/18.txt"
);

fn surface_area(cubes: &HashSet<Coord3d>) -> usize {
    let mut surface_area = 0;
    for cube in cubes {
        for adj_pt in cube.plus_adjacent() {
            if !cubes.contains(&adj_pt) {
                surface_area += 1;
            }
        }
    }
    surface_area
}

impl Solution for Day {
    type Parsed = HashSet<Coord3d>;
    type Answer = usize;
    const EXAMPLE_ANSWER_1: Self::Answer = 64;
    const ANSWER_1: Self::Answer = 3346;
    const EXAMPLE_ANSWER_2: Self::Answer = 58;
    const ANSWER_2: Self::Answer = 1980;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let points: HashSet<Coord3d> = input
            .lines()
            .map(|line| line.parse::<Coord3d>().expect("valid 3d point"))
            .collect();

        Ok(("", points))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        surface_area(&input)
    }

    fn part2(input: Self::Parsed) -> Self::Answer {
        let mut cubes = input.clone();

        let init_surface_area = surface_area(&cubes);

        // find fill range for new cube
        let (mut x_min, mut x_max) = input
            .iter()
            .map(|cube| cube.x)
            .fold((1000, 0), |(x_min, x_max), x| (x_min.min(x), x_max.max(x)));
        let (mut y_min, mut y_max) = input
            .iter()
            .map(|cube| cube.y)
            .fold((1000, 0), |(min, max), y| (min.min(y), max.max(y)));
        let (mut z_min, mut z_max) = input
            .iter()
            .map(|cube| cube.z)
            .fold((1000, 0), |(min, max), z| (min.min(z), max.max(z)));

        // guarentee we have open space all around
        x_min -= 2;
        x_max += 2;
        y_min -= 2;
        y_max += 2;
        z_min -= 2;
        z_max += 2;

        // fill in outershell 1 cube deep in each direction
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                cubes.insert(Coord3d::from_coords((x, y, z_min)));
                cubes.insert(Coord3d::from_coords((x, y, z_max)));
            }
        }
        for y in y_min..=y_max {
            for z in z_min..=z_max {
                cubes.insert(Coord3d::from_coords((x_min, y, z)));
                cubes.insert(Coord3d::from_coords((x_max, y, z)));
            }
        }
        for x in x_min..=x_max {
            for z in z_min..=z_max {
                cubes.insert(Coord3d::from_coords((x, y_min, z)));
                cubes.insert(Coord3d::from_coords((x, y_max, z)));
            }
        }

        // Now fill in from outer shell to external
        let start = Coord3d::from_coords((x_min + 1, y_min + 1, z_min + 1));
        let mut queue = vec![start];
        while let Some(cube) = queue.pop() {
            if cubes.insert(cube) {
                for adjacent_cube in cube.plus_adjacent() {
                    if !cubes.contains(&adjacent_cube) {
                        queue.push(adjacent_cube);
                    }
                }
            }
        }

        let x_len = (x_max - x_min + 1) as usize;
        let y_len = (y_max - y_min + 1) as usize;
        let z_len = (z_max - z_min + 1) as usize;

        let outer_shell_surface_area: usize = 2 * (x_len * y_len + y_len * z_len + x_len * z_len);
        let internal_surface_area = surface_area(&cubes) - outer_shell_surface_area;
        init_surface_area - internal_surface_area
    }
}
