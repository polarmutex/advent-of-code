use common::{solution, Answer};

solution!("Lanternfish", 6);

type Input = Vec<u64>;

fn parse(input: &str) -> nom::IResult<&str, Input> {
    let fish: Result<Vec<u64>, _> = input
        .split(',')
        .map(|num| num.parse::<u64>())
        .collect();
    
    match fish {
        Ok(f) => Ok(("", f)),
        Err(_) => Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::MapRes))),
    }
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, fish) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    Ok(sim_fish(80, &fish).into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, fish) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    Ok(sim_fish(256, &fish).into())
}

fn sim_fish(days: u32, input: &[u64]) -> u64 {
    let mut laternfish_timer_state: Vec<u64> = vec![0; 10];

    // initial
    for fish in input.iter() {
        let fish = *fish as usize;
        laternfish_timer_state[fish] += 1;
    }

    for _day in 0..days {
        let new_fish = laternfish_timer_state[0];
        laternfish_timer_state[0] = 0;
        for fish_state in 1..9 {
            laternfish_timer_state[fish_state - 1] += laternfish_timer_state[fish_state];
            laternfish_timer_state[fish_state] = 0;
        }
        laternfish_timer_state[6] += new_fish;
        laternfish_timer_state[8] = new_fish;
    }
    laternfish_timer_state.iter().sum()
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const EXAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 5934.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 26984457539u64.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2021, 6)?;
        assert_eq!(super::part_1(input.as_str())?, 388739.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2021, 6)?;
        assert_eq!(super::part_2(input.as_str())?, 1741362314973u64.into());
        Ok(())
    }
}
