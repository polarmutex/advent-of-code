use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;
use itertools::Itertools;

boilerplate!(
    Day,
    7,
    "\
16,1,2,0,4,2,7,1,2,14",
    "data/07.txt"
);

impl Solution for Day {
    type Parsed = Vec<u32>;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 37;
    const ANSWER_1: Self::Answer = 335330;
    const EXAMPLE_ANSWER_2: Self::Answer = 168;
    const ANSWER_2: Self::Answer = 92439766;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let crabs = input
            .split(',')
            .map(|val| val.parse::<u32>().unwrap())
            .collect_vec();
        Ok(("", crabs))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        let (low, high) = input.iter().fold((1_000, 0), |mut bounds, crab| {
            if bounds.0 > *crab {
                bounds.0 = *crab
            }
            if bounds.1 < *crab {
                bounds.1 = *crab
            }
            bounds
        });

        (low..high)
            .map(|pos| input.iter().map(|crab| crab.abs_diff(pos)).sum::<u32>())
            .min()
            .unwrap()
    }

    fn part2(input: Self::Parsed) -> Self::Answer {
        let (low, high) = input.iter().fold((1_000, 0), |mut bounds, crab| {
            if bounds.0 > *crab {
                bounds.0 = *crab
            }
            if bounds.1 < *crab {
                bounds.1 = *crab
            }
            bounds
        });

        (low..high)
            .map(|pos| {
                input
                    .iter()
                    .map(|crab| {
                        let d = crab.abs_diff(pos);
                        d * (d + 1) / 2
                    })
                    .sum::<u32>()
            })
            .min()
            .unwrap()
    }
}
