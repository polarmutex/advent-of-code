use core::fmt;
use aoc_runner_macros::{aoc, generator, solver, solution};
use nom::IResult;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self},
    sequence::separated_pair,
};
use std::{iter::Sum, ops::Add};

type Input = Vec<Snailfish>;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Snailfish {
    Number(u32),
    Fish((Box<Snailfish>, Box<Snailfish>)),
}
use Snailfish::*;
impl fmt::Display for Snailfish {
    // This trait requires `fmt` with this exact
    // signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number(n) => write!(f, "{}", n),
            Fish((ref a, ref b)) => {
                write!(f, "[{},{}]", a, b)
            }
        }
    }
}
impl Add for Snailfish {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::Fish((Box::new(self), Box::new(other)))
    }
}
impl Sum<Snailfish> for Snailfish {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Snailfish>,
    {
        let final_fish = iter.reduce(|acc, fish| {
            println!("{} + {}", &acc, &fish);
            let result = (acc + fish).reduce_all_the_way();
            println!("result {}", &result);
            result
        });
        match final_fish {
            Some(f) => f,
            None => {
                dbg!(":askdfl;jasfkljas");
                Number(0)
            }
        }
    }
}
#[derive(Debug)]
enum Operation {
    Explode((Option<u32>, Option<u32>)),
    Split,
    Stop,
}
impl Snailfish {
    /// magnitude only works on reduced snailfish
    fn magnitude(&self) -> usize {
        match self {
            Fish((box Number(a), box Number(b))) => {
                let left: usize = 3_usize * (*a as usize);
                let right: usize = 2_usize * (*b as usize);
                println!("{} + {} = {}", left, right, left + right);
                left + right
            }
            Fish((fish_a, fish_b)) => {
                let left = fish_a.magnitude();
                let right = fish_b.magnitude();
                println!("mag {} + mag {} = {}", left, right, 3 * left + 2 * right);
                3 * left + 2 * right
            }
            Number(n) => *n as usize,
        }
    }

    fn reduce_all_the_way(&self) -> Self {
        println!("Starting Fish {}", &self);
        let mut fish = self.clone();
        loop {
            let new_fish = fish.reduce();
            println!("new fish {}", &new_fish);

            if new_fish == fish {
                break;
            } else {
                fish = new_fish;
            }
        }
        fish
    }
    fn reduce(&self) -> Self {
        let (fish, applied_operation) = self.step(0);
        match applied_operation {
            Some(Operation::Explode(_)) => fish.unwrap(),
            None => {
                let fish = self.try_split();
                fish
            }
            panic_op => {
                dbg!(panic_op);
                panic!("shouldn't be possible to have alt operations")
            }
        }
    }
    fn try_split(&self) -> Self {
        match self {
            Number(num) => {
                // dbg!(num);
                Number(*num);
                if *num >= 10 {
                    let split_num = num / 2;
                    Fish((
                        Box::new(Number(split_num)),
                        Box::new(Number(split_num + *num % 2)),
                    ))
                } else {
                    Number(*num)
                }
            }
            Fish((fish_a, fish_b)) => {
                let new_fish_a = fish_a.try_split();
                if new_fish_a != **fish_a {
                    // split happened
                    Fish((Box::new(new_fish_a), fish_b.clone()))
                } else {
                    Fish((fish_a.clone(), Box::new(fish_b.try_split())))
                }
            }
        }
    }
    fn step(&self, level: usize) -> (Option<Snailfish>, Option<Operation>) {
        // dbg!(self);
        match self {
            Number(num) => {
                // dbg!(num);
                (Some(Number(*num)), None)
            }
            Fish((box Number(a), box Number(b))) => {
                // dbg!(a,b);
                if level >= 4 {
                    (None, Some(Operation::Explode((Some(*a), Some(*b)))))
                } else {
                    (
                        Some(Fish((Box::new(Number(*a)), Box::new(Number(*b))))),
                        None,
                    )
                }
            }
            Fish(fishes) => {
                // dbg!(fishes);
                let operation = fishes.0.step(level + 1);
                // dbg!(&operation);
                let (new_left_fish, op) = match operation {
                    (new_fish, Some(Operation::Explode((left, right)))) => {
                        match &fishes.1 {
                            box Number(num) => {
                                // dbg!(num);

                                if let Some(op_num) = right {
                                    // dbg!(op_num, num,
                                    // &new_fish);

                                    let new_num = op_num + num;

                                    (
                                        Fish((
                                            match new_fish {
                                                Some(fish) => Box::new(fish),
                                                None => Box::new(Number(0)),
                                            },
                                            Box::new(Number(new_num)),
                                        )),
                                        Some(Operation::Explode((left, None))),
                                    )
                                } else {
                                    (
                                        Fish((Box::new(new_fish.unwrap()), Box::new(Number(*num)))),
                                        Some(Operation::Explode((left, None))),
                                    )
                                }

                                // let new_op =
                            }
                            box Fish((box Number(num), fishy)) => {
                                // dbg!(num);

                                // TODO: merge top number
                                // dbg!(fishy.clone());
                                if let Some(op_num) = right {
                                    // dbg!(op_num, num,
                                    // &new_fish);

                                    let new_num = op_num + num;

                                    (
                                        Fish((
                                            match new_fish {
                                                Some(fish) => Box::new(fish),
                                                None => Box::new(Number(0)),
                                            },
                                            Box::new(Fish((
                                                Box::new(Number(new_num)),
                                                fishy.clone(),
                                            ))),
                                        )),
                                        Some(Operation::Explode((left, None))),
                                    )
                                } else {
                                    (
                                        Fish((
                                            Box::new(new_fish.unwrap()),
                                            Box::new(Fish((Box::new(Number(*num)), fishy.clone()))),
                                        )),
                                        Some(Operation::Explode((left, None))),
                                    )
                                }
                            }
                            fish => match right {
                                Some(value) => {
                                    let fish_fish = munge_leftmost(&fish, value);
                                    (
                                        Fish((
                                            Box::new(new_fish.unwrap()),
                                            Box::new(fish_fish.clone()),
                                        )),
                                        Some(Operation::Explode((left, None))),
                                    )
                                }
                                None => (
                                    Fish((Box::new(new_fish.unwrap()), fish.clone())),
                                    Some(Operation::Explode((left, right))),
                                ),
                            },
                        }
                    }
                    (new_fish, Some(Operation::Split)) => {
                        todo!("split");
                    }
                    (new_fish, Some(Operation::Stop)) => (new_fish.unwrap(), Some(Operation::Stop)),
                    (fish, None) => {
                        // dbg!(&fish);
                        (fish.unwrap(), None)
                    }
                };
                // dbg!(&new_fish);
                match op {
                    Some(o) => (Some(new_left_fish), Some(o)),
                    None => {
                        // same as above, for right hand
                        // side
                        let operation = fishes.1.step(level + 1);
                        // dbg!(&operation);

                        match operation {
                            (new_right_fish, Some(Operation::Explode((left, right)))) => {
                                match &fishes.0 {
                                    box Number(num) => {
                                        // dbg!(num);
                                        if let Some(op_num) = left {
                                            // dbg!(
                                            //     op_num,
                                            // num
                                            // );
                                            let new_num = op_num + num;
                                            (
                                                Some(Fish((
                                                    Box::new(Number(new_num)),
                                                    match new_right_fish {
                                                        Some(fish) => Box::new(fish),
                                                        None => Box::new(Number(0)),
                                                    },
                                                ))),
                                                Some(Operation::Explode((None, right))),
                                            )
                                        } else {
                                            // dbg!(&num,
                                            // &new_right_fish);
                                            //
                                            (
                                                Some(Fish((
                                                    Box::new(Number(*num)),
                                                    Box::new(new_right_fish.unwrap()),
                                                ))),
                                                Some(Operation::Explode((None, right))),
                                            )
                                        }

                                        // let new_op =
                                    }
                                    box Fish((fishy, box Number(num))) => {
                                        println!("{} -- {}", &fishy, num);

                                        // dbg!(num);
                                        if let Some(op_num) = left {
                                            // dbg!(
                                            //     op_num,
                                            // num
                                            // );
                                            let new_num = op_num + num;
                                            (
                                                Some(Fish((
                                                    Box::new(Fish((
                                                        fishy.clone(),
                                                        Box::new(Number(new_num)),
                                                    ))),
                                                    match new_right_fish {
                                                        Some(fish) => Box::new(fish),
                                                        None => Box::new(Number(0)),
                                                    },
                                                ))),
                                                Some(Operation::Explode((None, right))),
                                            )
                                        } else {
                                            // dbg!(&num,
                                            // &new_right_fish);
                                            //
                                            (
                                                Some(Fish((
                                                    Box::new(Fish((
                                                        fishy.clone(),
                                                        Box::new(Number(*num)),
                                                    ))),
                                                    Box::new(new_right_fish.unwrap()),
                                                ))),
                                                Some(Operation::Explode((None, right))),
                                            )
                                        }

                                        // let new_op =
                                    }
                                    fish => {
                                        match left {
                                            Some(value) => {
                                                let fish_fish = munge_rightmost(&fish, value);
                                                (
                                                    Some(Fish((
                                                        Box::new(fish_fish.clone()),
                                                        Box::new(new_right_fish.unwrap()),
                                                    ))),
                                                    Some(Operation::Explode((None, right))),
                                                )
                                            }
                                            None => (
                                                Some(Fish((
                                                    fish.clone(),
                                                    Box::new(new_right_fish.unwrap()),
                                                ))),
                                                Some(Operation::Explode((left, right))),
                                            ),
                                        }
                                        // dbg!(&op);
                                    }
                                }
                            }
                            (new_fish, Some(Operation::Split)) => {
                                todo!("split");
                            }
                            (new_fish, Some(Operation::Stop)) => (new_fish, Some(Operation::Stop)),
                            (fish, None) => (fish, None),
                        }
                    }
                }
            }
        }
    }
    fn explode(&self) {}
    fn split() {}
}
fn munge_rightmost(fish: &Snailfish, value: u32) -> Snailfish {
    match fish {
        Number(n) => panic!("helpmeee"),
        Fish((a, b)) => Fish((
            a.clone(),
            match b {
                box Number(n) => Box::new(Number(n + value)),
                box fishy => Box::new(munge_rightmost(&fishy, value)),
            },
        )),
    }
}
fn munge_leftmost(fish: &Snailfish, value: u32) -> Snailfish {
    match fish {
        Number(n) => panic!("helpmeee"),
        Fish((a, b)) => Fish((
            match a {
                box Number(n) => Box::new(Number(n + value)),
                box fishy => Box::new(munge_leftmost(&fishy, value)),
            },
            b.clone(),
        )),
    }
}

fn snailfish(input: &str) -> IResult<&str, Snailfish> {
    let has_snailfish: IResult<&str, &str> = tag("[")(input);

    let (input, fish_number) = match has_snailfish {
        Ok((input, _)) => {
            let (input, fish) = separated_pair(snailfish, tag(","), snailfish)(input)?;
            let (input, _) = tag("]")(input)?;
            (input, Fish((Box::new(fish.0), Box::new(fish.1))))
        }
        Err(_) => {
            let (input, num) = complete::u32(input)?;
            (input, Number(num))
        }
    };

    Ok((input, fish_number))
}

#[aoc(2021, day18)]
pub mod solutions {
    use super::*;

    fn parse(input: &str) -> nom::IResult<&str, Input> {
        let snailfishes = input
            .lines()
            .map(|line| snailfish(line).unwrap().1)
            .collect_vec();
        Ok(("", snailfishes))
    }

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> u64 {
        let fish: Snailfish = input.iter().map(|sf| sf.clone()).sum();
        fish.magnitude() as u64
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> u64 {
        let max = input
            .iter()
            .cartesian_product(input.clone())
            .map(|(a, b)| (a.clone() + b.clone()).reduce_all_the_way().magnitude())
            .max();
        max.unwrap() as u64
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> u64 {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> u64 {
        let data = input_generator(input);
        solve_part2(&data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";

    #[test]
    fn test_part_1() {
        assert_eq!(solutions::part_1(EXAMPLE), 4140);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solutions::part_2(EXAMPLE), 3993);
    }
}
