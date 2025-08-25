use common::{solution, Answer};
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

solution!("Scratchcards", 4);

type Input = Vec<Card>;

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

fn card_number_parser(input: &str) -> nom::IResult<&str, HashSet<u32>> {
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

fn card_parser(input: &str) -> nom::IResult<&str, Card> {
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

fn parse(data: &str) -> nom::IResult<&str, Input> {
    separated_list1(line_ending, card_parser)(data)
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let result = data.iter().map(|c| c.score()).sum::<u32>();
    
    Ok(result.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
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
            res
        });
    let result = temp.values().sum::<u32>();
    
    Ok(result.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 13.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 30.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2023, 4)?;
        assert_eq!(super::part_1(input.as_str())?, 27845.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2023, 4)?;
        assert_eq!(super::part_2(input.as_str())?, 9496801.into());
        Ok(())
    }
}
