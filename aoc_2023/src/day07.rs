use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
use itertools::Itertools;
use itertools::Position;
use nom::character::complete::alphanumeric1;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::Parser;
use nom_supreme::tag::complete::tag;

boilerplate!(
    Day,
    7,
    "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
",
    "data/07.txt"
);

#[derive(Clone, Debug)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Clone, Debug)]
struct Hand {
    cards: String,
    bid: u32,
}

impl Hand {
    fn hand_type(&self, use_jokers: bool) -> HandType {
        let counts_str = if !use_jokers {
            self.cards
                .chars()
                .counts()
                .into_iter()
                .sorted_by(|a, b| a.1.cmp(&b.1))
                .rev()
                .map(|(_, v)| v.to_string())
                .join("")
        } else {
            let num_jokers = self.cards.chars().counts().get(&'J').map_or(0, |v| *v);
            if num_jokers == 5 {
                String::from("5")
            } else {
                self.cards
                    .chars()
                    .counts()
                    .into_iter()
                    .sorted_by(|a, b| a.1.cmp(&b.1))
                    .rev()
                    .filter_map(|(k, v)| (k != 'J').then_some(v))
                    .with_position()
                    .map(|pos| match pos {
                        Position::First(num) | Position::Only(num) => num + num_jokers,
                        Position::Middle(num) | Position::Last(num) => num,
                    })
                    .map(|v| v.to_string())
                    .join("")
            }
        };
        dbg!(&self.cards);
        dbg!(&counts_str);

        match counts_str.as_str() {
            "5" => HandType::FiveOfAKind,
            "41" => HandType::FourOfAKind,
            "32" => HandType::FullHouse,
            "311" => HandType::ThreeOfAKind,
            "221" => HandType::TwoPair,
            "2111" => HandType::OnePair,
            "11111" => HandType::HighCard,
            value => panic!("{value} is unknown hand type"),
        }
    }

    fn card_value(&self, use_jokers: bool) -> Vec<u8> {
        self.cards
            .chars()
            .map(|c| {
                let c = if use_jokers {
                    if c == 'J' {
                        '1'
                    } else {
                        c
                    }
                } else {
                    c
                };
                match c {
                    'A' => 14u8,
                    'K' => 13u8,
                    'Q' => 12u8,
                    'J' => 11u8,
                    'T' => 10u8,
                    value => value.to_digit(10).expect("valid digit") as u8,
                }
            })
            .collect_vec()
    }
}

#[tracing::instrument(skip(input))]
fn parse_hand(input: &str) -> IResult<Hand> {
    let (input, (cards, bid)) = separated_pair(alphanumeric1, tag(" "), digit1).parse(input)?;
    Ok((
        input,
        Hand {
            cards: String::from(cards),
            bid: bid.parse::<u32>().expect("a number"),
        },
    ))
}

impl Solution for Day {
    type Parsed = Vec<Hand>;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    #[tracing::instrument(skip(data))]
    fn parse(data: &str) -> IResult<Self::Parsed> {
        separated_list1(line_ending, parse_hand).parse(data)
    }

    #[tracing::instrument(skip(data))]
    fn part1(data: Self::Parsed) -> Self::Answer {
        let sorted = data
            .into_iter()
            .sorted_by_key(|h| (h.hand_type(false) as u8, h.card_value(false)));
        dbg!(&sorted);
        sorted
            .enumerate()
            .map(|(i, h)| (i as u32 + 1) * h.bid)
            .sum()
    }

    #[tracing::instrument(skip(data))]
    fn part2(data: Self::Parsed) -> Self::Answer {
        let sorted = data
            .into_iter()
            .sorted_by_key(|h| (h.hand_type(true) as u8, h.card_value(true)));
        dbg!(&sorted);
        sorted
            .enumerate()
            .map(|(i, h)| (i as u32 + 1) * h.bid)
            .sum()
    }
}

tests! {
     const EXAMPLE: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    add_test!(part1_example, test_part1_example, EXAMPLE => 6440);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 248453531);
    add_test!(part2_example, test_part2_example, EXAMPLE => 5905);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 248781813);
}
