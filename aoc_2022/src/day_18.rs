use common::{solution, Answer};

use glam::IVec3;

use std::collections::HashSet;

solution!("Boiling Boulders", 18);

type Input = HashSet<IVec3>;

fn surface_area(cubes: &HashSet<IVec3>) -> usize {
    let mut surface_area = 0;
    for cube in cubes {
        for adj_pt in get_adjacent(*cube) {
            if !cubes.contains(&adj_pt) {
                surface_area += 1;
            }
        }
    }
    surface_area
}

fn get_adjacent(pos: IVec3) -> Vec<IVec3> {
    vec![
        pos + IVec3::new(1, 0, 0),
        pos + IVec3::new(-1, 0, 0),
        pos + IVec3::new(0, 1, 0),
        pos + IVec3::new(0, -1, 0),
        pos + IVec3::new(0, 0, 1),
        pos + IVec3::new(0, 0, -1),
    ]
}

fn parse(data: &str) -> nom::IResult<&str, Input> {
    let points: HashSet<IVec3> = data
        .lines()
        .map(|line| {
            let coords: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
            IVec3::new(coords[0], coords[1], coords[2])
        })
        .collect();

    Ok(("", points))
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    let result = surface_area(&data);
    Ok(result.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    let mut cubes = data.clone();

    let init_surface_area = surface_area(&cubes);

    // find fill range for new cube
    let (mut x_min, mut x_max) = data
        .iter()
        .map(|cube| cube.x)
        .fold((1000, 0), |(x_min, x_max), x| (x_min.min(x), x_max.max(x)));
    let (mut y_min, mut y_max) = data
        .iter()
        .map(|cube| cube.y)
        .fold((1000, 0), |(min, max), y| (min.min(y), max.max(y)));
    let (mut z_min, mut z_max) = data
        .iter()
        .map(|cube| cube.z)
        .fold((1000, 0), |(min, max), z| (min.min(z), max.max(z)));

    // guarantee we have open space all around
    x_min -= 2;
    x_max += 2;
    y_min -= 2;
    y_max += 2;
    z_min -= 2;
    z_max += 2;

    // fill in outershell 1 cube deep in each direction
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            cubes.insert(IVec3::new(x, y, z_min));
            cubes.insert(IVec3::new(x, y, z_max));
        }
    }
    for y in y_min..=y_max {
        for z in z_min..=z_max {
            cubes.insert(IVec3::new(x_min, y, z));
            cubes.insert(IVec3::new(x_max, y, z));
        }
    }
    for x in x_min..=x_max {
        for z in z_min..=z_max {
            cubes.insert(IVec3::new(x, y_min, z));
            cubes.insert(IVec3::new(x, y_max, z));
        }
    }

    // Now fill in from outer shell to external
    let start = IVec3::new(x_min + 1, y_min + 1, z_min + 1);
    let mut queue = vec![start];
    while let Some(cube) = queue.pop() {
        if cubes.insert(cube) {
            for adjacent_cube in get_adjacent(cube) {
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
    let result = init_surface_area - internal_surface_area;
    Ok(result.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;

    const EXAMPLE: &str = "2,2,2
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
2,3,5";

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 64.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 58.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2022, 18)?;
        assert_eq!(super::part_1(input.as_str())?, 3346.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2022, 18)?;
        assert_eq!(super::part_2(input.as_str())?, 1980.into());
        Ok(())
    }
}
