use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
use itertools::Itertools;
use nom::bytes::complete::take_until;
use nom::character::complete;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::multi::fold_many1;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::Parser;
use nom_supreme::tag::complete::tag;
use nom_supreme::ParserExt;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::ops::Range;

boilerplate!(
    Day,
    4,
    "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
",
    "data/05.txt"
);

#[derive(Clone, Debug)]
struct Map {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}

impl Map {
    fn convert(&self, val: u64) -> u64 {
        let valid_mapping = self.mappings.iter().find(|(src, dst)| src.contains(&val));
        if let Some((src, dst)) = valid_mapping {
            dst.start + (val - src.start)
        } else {
            val
        }
    }
}

#[derive(Clone, Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

fn line(input: &str) -> IResult<(Range<u64>, Range<u64>)> {
    let (input, (destination, source, num)) = tuple((
        complete::u64,
        complete::u64.preceded_by(tag(" ")),
        complete::u64.preceded_by(tag(" ")),
    ))(input)?;

    Ok((
        input,
        (source..(source + num), destination..(destination + num)),
    ))
}

fn parse_mappings(input: &str) -> IResult<Map> {
    take_until("map:")
        .precedes(tag("map:"))
        .precedes(many1(line_ending.precedes(line)).map(|mappings| Map { mappings }))
        .parse(input)
}

impl Solution for Day {
    type Parsed = Almanac;
    type Answer = u64;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    fn parse(data: &str) -> IResult<Self::Parsed> {
        let (input, seeds) = tag("seeds: ")
            .precedes(separated_list1(space1, complete::u64))
            .parse(data)?;
        let (input, maps) = many1(parse_mappings)(input)?;
        Ok(("", Almanac { seeds, maps }))
    }

    fn part1(data: Self::Parsed) -> Self::Answer {
        let locations = data
            .seeds
            .iter()
            .map(|seed| data.maps.iter().fold(*seed, |seed, map| map.convert(seed)))
            .collect_vec();
        *locations.iter().min().expect("should have min")
    }

    fn part2(data: Self::Parsed) -> Self::Answer {
        let seeds = data
            .seeds
            .chunks(2)
            .flat_map(|chunk| {
                let mut arr = vec![];
                for i in 0..chunk[1] {
                    arr.push(chunk[0] + i)
                }
                arr
            })
            .collect::<Vec<u64>>();
        // dbg!(seeds.clone());
        let locations = seeds
            .iter()
            .map(|seed| data.maps.iter().fold(*seed, |seed, map| map.convert(seed)))
            .collect_vec();
        *locations.iter().min().expect("should have min")
    }
}

tests! {
     const EXAMPLE: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    add_test!(part1_example, test_part1_example, EXAMPLE => 35);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 111627841);
    add_test!(part2_example, test_part2_example, EXAMPLE => 46);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 69323688);
}
