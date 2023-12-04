use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
// use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::multi::fold_many1;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use std::collections::BTreeMap;
use std::collections::HashSet;

boilerplate!(
    Day,
    4,
    "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
",
    "data/04.txt"
);

#[derive(Clone, Debug)]
struct Card {
    // id: u32,
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}
impl Card {
    fn num_matching(&self) -> u32 {
        self.winning_numbers.intersection(&self.my_numbers).count() as u32
    }
    fn score(&self) -> u32 {
        let num_winning = self.num_matching();
        match num_winning.checked_sub(1) {
            Some(num) => 2u32.pow(num),
            None => 0,
        }
    }
}

fn card_number_parser(input: &str) -> IResult<HashSet<u32>> {
    let (input, numbers) = fold_many1(
        terminated(complete::u32, space0),
        HashSet::new,
        |mut res, item| {
            res.insert(item);
            res
        },
    )(input)?;
    Ok((input, numbers))
}

fn card_parser(input: &str) -> IResult<Card> {
    let (input, _) = delimited(
        tuple((tag("Card"), space1)),
        digit1,
        tuple((tag(":"), space1)),
    )(input)?;
    let (input, (winning_numbers, my_numbers)) = separated_pair(
        card_number_parser,
        tuple((tag("|"), space1)),
        card_number_parser,
    )(input)?;
    Ok((
        input,
        Card {
            winning_numbers,
            my_numbers,
        },
    ))
}

impl Solution for Day {
    type Parsed = Vec<Card>;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    fn parse(data: &str) -> IResult<Self::Parsed> {
        separated_list1(line_ending, card_parser)(data)
    }

    fn part1(data: Self::Parsed) -> Self::Answer {
        data.iter().map(|c| c.score()).sum()
    }

    fn part2(data: Self::Parsed) -> Self::Answer {
        // let num_matching = data.iter().map(|c| c.num_matching()).collect_vec();

        let total_scratchcards = (0..data.len())
            .map(|i| (i, 1))
            .collect::<BTreeMap<usize, u32>>();

        let temp = data
            .iter()
            .enumerate()
            .fold(total_scratchcards, |mut res, (index, card)| {
                let num_matches = card.num_matching();
                for i in (index + 1)..(index + 1 + num_matches as usize) {
                    let current_index_cards = *res.get(&index).unwrap();
                    res.entry(i).and_modify(|v| {
                        *v += current_index_cards;
                    });
                }
                // dbg!(res.clone());
                res
            });
        temp.values().sum()
    }
}

tests! {
     const EXAMPLE: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    add_test!(part1_example, test_part1_example, EXAMPLE => 13);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 27845);
    add_test!(part2_example, test_part2_example, EXAMPLE => 30);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 9496801);
}
