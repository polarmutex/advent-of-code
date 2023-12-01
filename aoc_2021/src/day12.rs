use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
};
use std::collections::BTreeMap;

boilerplate!(
    Day,
    12,
    "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW",
    "data/12.txt"
);

fn step(node_id: String, path: Vec<String>, allowed_ways: &BTreeMap<String, Vec<String>>) -> usize {
    if node_id == "end" {
        1
    } else {
        let next_nodes = allowed_ways.get(&node_id).unwrap();
        next_nodes
            .iter()
            .map(|node| {
                if node.chars().all(|c| c.is_lowercase()) && path.contains(&node) {
                    0
                } else {
                    let mut new_path = path.clone();
                    new_path.push(node.clone());
                    step(node.clone(), new_path, &allowed_ways)
                }
            })
            .sum()
    }
}

fn step_2(
    node_id: String,
    path: Vec<String>,
    allowed_ways: &BTreeMap<String, Vec<String>>,
) -> usize {
    if node_id == "end" {
        1
    } else {
        let next_nodes = allowed_ways.get(&node_id).unwrap();
        next_nodes
            .iter()
            .map(|node| {
                let count_of_all_small_cave_visits = path
                    .iter()
                    .filter(|path_node| path_node.chars().all(|c| c.is_lowercase()))
                    .fold(BTreeMap::new(), |mut acc, item| {
                        acc.entry(item).and_modify(|v| *v += 1).or_insert(1);
                        acc
                    });
                let have_visited_small_cave_twice =
                    count_of_all_small_cave_visits.iter().any(|(_, &v)| v == 2);
                if node.chars().all(|c| c.is_lowercase())
                    && path.contains(&node)
                    && have_visited_small_cave_twice
                {
                    0
                } else {
                    let mut new_path = path.clone();
                    new_path.push(node.clone());
                    step_2(node.clone(), new_path, &allowed_ways)
                }
            })
            .sum()
    }
}

impl Solution for Day {
    type Parsed = BTreeMap<String, Vec<String>>;
    type Answer = usize;
    const EXAMPLE_ANSWER_1: Self::Answer = 10;
    const ANSWER_1: Self::Answer = 3369;
    const EXAMPLE_ANSWER_2: Self::Answer = 3509;
    const ANSWER_2: Self::Answer = 85883;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let (input, nodes) =
            separated_list1(newline, separated_pair(alpha1, tag("-"), alpha1))(input)?;
        let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (a, b) in nodes {
            if a == "end" {
                map.entry(a.into()).or_insert(vec![]);
            } else if b == "start" {
            } else {
                map.entry(a.into())
                    .and_modify(|v| {
                        v.push(b.into());
                    })
                    .or_insert(vec![b.into()]);
            }
            if b == "end" {
                map.entry(b.into()).or_insert(vec![]);
            } else if a == "start" {
            } else {
                map.entry(b.into())
                    .and_modify(|v| {
                        v.push(a.into());
                    })
                    .or_insert(vec![a.into()]);
            }
        }

        Ok((input, map))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        step("start".into(), vec!["start".into()], &input)
    }

    fn part2(input: Self::Parsed) -> Self::Answer {
        step_2("start".into(), vec!["start".into()], &input)
    }
}

const EXAMPLE_SM: &str = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end
";
const EXAMPLE_MD: &str = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";

// add_test!(part1, part1_examplesm_test, EXAMPLE_SM => 10);
// add_test!(part1, part1_examplemd_test, EXAMPLE_MD => 19);
// add_test!(part1, part1_examplelg_test, EXAMPLE_LG => 226);
// add_test!(part1, part1_input_test, INPUT => 3369);
// add_test!(part2, part2_examplesm_test, EXAMPLE_SM => 36);
// add_test!(part2, part2_examplemd_test, EXAMPLE_MD => 103);
// add_test!(part2, part2_examplelg_test, EXAMPLE_LG => 3509);
// add_test!(part2, part2_input_test, INPUT => 85883);
