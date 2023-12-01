use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;
use itertools::Itertools;
use nom::character::complete;
use nom::multi::separated_list1;
use nom::IResult as IResultSpecial;
use nom_supreme::final_parser::{Location, RecreateContext};
use nom_supreme::{error::ErrorTree, final_parser::final_parser};

boilerplate!(
    Day,
    1,
    "\
1721
979
366
299
675
1456
",
    "data/01.txt"
);

fn expense_report(input: &str) -> IResultSpecial<&str, Vec<u32>, ErrorTree<&str>> {
    let (input, expense_report) = separated_list1(complete::newline, complete::u32)(input)?;
    let (input, _) = complete::newline(input)?;
    Ok((input, expense_report))
}

fn expense_report_final(input: &str) -> Result<Vec<u32>, ErrorTree<&str>> {
    final_parser(expense_report)(input)
}

impl Solution for Day {
    type Parsed = Vec<u32>;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 514579;
    const ANSWER_1: Self::Answer = 55776;
    const EXAMPLE_ANSWER_2: Self::Answer = 241861950;
    const ANSWER_2: Self::Answer = 223162626;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        dbg!(Day::EXAMPLE_DATA);
        dbg!(input);
        let result = expense_report_final(input);
        if let Err(nom_supreme::error::GenericErrorTree::Stack {
            ref base,
            ref contexts,
        }) = result
        {
            println!("omg here heeh");
            dbg!(&base);
            for context in contexts {
                dbg!(Location::recreate_context("#5", context.0));
            }
        }
        Ok(("", result.unwrap()))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        let result = input
            .iter()
            .permutations(2)
            .map(|v| (v[0], v[1]))
            .find(|perm| match *perm {
                (left, right) => {
                    if left + right == 2020 {
                        true
                    } else {
                        false
                    }
                }
            });
        dbg!(&result);
        if let Some((left, right)) = result {
            left * right
        } else {
            0
        }
    }

    fn part2(input: Self::Parsed) -> Self::Answer {
        let result = input
            .iter()
            .permutations(3)
            .map(|v| (v[0], v[1], v[2]))
            .find(|perm| match *perm {
                (left, middle, right) => {
                    if left + middle + right == 2020 {
                        true
                    } else {
                        false
                    }
                }
            });
        dbg!(&result);
        if let Some((left, middle, right)) = result {
            left * middle * right
        } else {
            0
        }
    }
}
