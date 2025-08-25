use common::{solution, Answer};
use itertools::Itertools;
use itertools::Position;
use nom::character::complete::alphanumeric1;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::Parser;
use nom_supreme::tag::complete::tag;

solution!("Camel Cards", 7);

type Input = Vec<Hand>;

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
fn parse_hand(input: &str) -> nom::IResult<&str, Hand> {
    let (input, (cards, bid)) = separated_pair(alphanumeric1, tag(" "), digit1).parse(input)?;
    Ok((
        input,
        Hand {
            cards: String::from(cards),
            bid: bid.parse::<u32>().expect("a number"),
        },
    ))
}

#[tracing::instrument(skip(data))]
fn parse(data: &str) -> nom::IResult<&str, Input> {
    separated_list1(line_ending, parse_hand).parse(data)
}

#[tracing::instrument(skip(input))]
fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let sorted = data
        .into_iter()
        .sorted_by_key(|h| (h.hand_type(false) as u8, h.card_value(false)));
    dbg!(&sorted);
    let result = sorted
        .enumerate()
        .map(|(i, h)| (i as u32 + 1) * h.bid)
        .sum::<u32>();
        
    Ok(result.into())
}

#[tracing::instrument(skip(input))]
fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let sorted = data
        .into_iter()
        .sorted_by_key(|h| (h.hand_type(true) as u8, h.card_value(true)));
    dbg!(&sorted);
    let result = sorted
        .enumerate()
        .map(|(i, h)| (i as u32 + 1) * h.bid)
        .sum::<u32>();
        
    Ok(result.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 6440.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 5905.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2023, 7)?;
        assert_eq!(super::part_1(input.as_str())?, 248453531.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2023, 7)?;
        assert_eq!(super::part_2(input.as_str())?, 248781813.into());
        Ok(())
    }
}
