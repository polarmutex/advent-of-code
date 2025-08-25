use common::{solution, Answer};
use itertools::Itertools;
use nom::bytes::complete::take_until;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::Parser;
use nom_supreme::tag::complete::tag;
use nom_supreme::ParserExt;
use std::ops::Range;
use tracing::info;

solution!("If You Give A Seed A Fertilizer", 5);

type Input = Almanac;

#[derive(Debug, Clone)]
struct MappedRange {
    before: Option<Range<u64>>,
    overlap: Option<Range<u64>>,
    after: Option<Range<u64>>,
}

#[derive(Clone, Debug)]
struct Map {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}

impl Map {
    fn convert(&self, val: u64) -> u64 {
        let valid_mapping = self.mappings.iter().find(|(src, _)| src.contains(&val));
        if let Some((src, dst)) = valid_mapping {
            dst.start + (val - src.start)
        } else {
            val
        }
    }
    fn convert_ranges(&self, val: Range<u64>) -> Vec<Range<u64>> {
        let mut overlaps = vec![];
        let mut old = vec![val];
        let mut old_next = vec![];
        for mapping in &self.mappings {
            for range in old.drain(..) {
                let mapped = self.convert_range(range, mapping);
                overlaps.extend(mapped.overlap);
                old_next.extend(mapped.before);
                old_next.extend(mapped.after);
            }
            std::mem::swap(&mut old, &mut old_next);
        }
        overlaps.into_iter().chain(old).collect()
    }
    fn convert_range(&self, val: Range<u64>, mapping: &(Range<u64>, Range<u64>)) -> MappedRange {
        dbg!(mapping);
        let (src, dst) = mapping;
        let opt_range = |start, end| Some(start..end).filter(|r| !r.is_empty());
        let before = opt_range(val.start, src.start.min(val.end));
        let after = opt_range(src.end.max(val.start), val.end);
        let overlap = opt_range(val.start.max(src.start), val.end.min(src.end))
            .map(|r| (r.start + dst.start - src.start)..(r.end + dst.start - src.start));
        MappedRange {
            before,
            overlap,
            after,
        }
    }
}

#[derive(Clone, Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

#[tracing::instrument(skip(input))]
fn line(input: &str) -> nom::IResult<&str, (Range<u64>, Range<u64>)> {
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

fn parse_mappings(input: &str) -> nom::IResult<&str, Map> {
    take_until("map:")
        .precedes(tag("map:"))
        .precedes(many1(line_ending.precedes(line)).map(|mappings| Map { mappings }))
        .parse(input)
}

#[tracing::instrument(skip(data))]
fn parse(data: &str) -> nom::IResult<&str, Input> {
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(space1, complete::u64))
        .parse(data)?;
    let (input, maps) = many1(parse_mappings)(input)?;
    info!(?seeds);
    Ok((input, Almanac { seeds, maps }))
}

#[tracing::instrument(skip(input))]
fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let locations = data
        .seeds
        .iter()
        .map(|seed| data.maps.iter().fold(*seed, |seed, map| map.convert(seed)))
        .collect_vec();
    let result = *locations.iter().min().expect("should have min");
    
    Ok(result.into())
}

#[tracing::instrument(skip(input))]
fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let seed_ranges = data.seeds.into_iter().tuples().map(|(start, range)| Range {
        start,
        end: start + range,
    });
    info!(?seed_ranges);
    let result = seed_ranges
        .flat_map(|seed_range| {
            data.maps.iter().fold(vec![seed_range], |acc, map| {
                acc.into_iter()
                    .flat_map(|r| map.convert_ranges(r))
                    .collect()
            })
        })
        .map(|locations_range| locations_range.start)
        .min()
        .expect("to have min");
        
    Ok(result.into())
}

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
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
    "};

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE)?, 35.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE)?, 46.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2023, 5)?;
        assert_eq!(super::part_1(input.as_str())?, 111627841.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2023, 5)?;
        assert_eq!(super::part_2(input.as_str())?, 69323688.into());
        Ok(())
    }
}
