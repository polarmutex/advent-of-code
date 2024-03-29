use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;
use itertools::Itertools;
use std::cmp::Ordering;
use std::str::FromStr;

boilerplate!(
    Day,
    13,
    "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
",
    "data/13.txt"
);

#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
    Num(u32),
    List(Vec<Packet>),
}
impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Num(num) => write!(f, "{}", num),
            Packet::List(pkts) => write!(f, "[{}]", pkts.iter().join(",")),
        }
    }
}

fn parse_packet(input: &str) -> Packet {
    if &input[0..1] == "[" {
        let mut stack = 0_i32;
        let list: Vec<Packet> = input[1..input.len() - 1]
            .split(|c| {
                if c == '[' {
                    stack += 1
                } else if c == ']' {
                    stack -= 1
                }
                c == ',' && stack == 0
            })
            .filter_map(|s| (!s.is_empty()).then(|| parse_packet(s)))
            .collect();
        Packet::List(list)
    } else {
        Packet::Num(input.parse().unwrap())
    }
}

impl FromStr for Packet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let packet = parse_packet(s);
        Ok(packet)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // Num vs Num
            (Packet::Num(self_num), Packet::Num(other_num)) => {
                println!("Compare {} vs {}", self_num, other_num);
                self_num.cmp(other_num)
            }
            // List vs List
            (Packet::List(self_pkts), Packet::List(other_pkts)) => {
                println!(
                    "Comparing [{}] vs [{}]",
                    self_pkts.iter().join(","),
                    other_pkts.iter().join(",")
                );
                self_pkts.cmp(other_pkts)
            }
            // Num vs List
            (Packet::Num(self_num), Packet::List(other_pkts)) => {
                println!(
                    "Comparing {} vs [{}]",
                    self_num,
                    other_pkts.iter().join(",")
                );
                Packet::List(vec![Packet::Num(*self_num)]).cmp(other)
            }
            // List vs Num
            (Packet::List(self_pkts), Packet::Num(other_num)) => {
                println!(
                    "Comparing [{}] vs {}",
                    self_pkts.iter().join(","),
                    other_num,
                );
                self.cmp(&Packet::List(vec![Packet::Num(*other_num)]))
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

impl Solution for Day {
    type Parsed = Vec<PacketPair>;
    type Answer = usize;
    const EXAMPLE_ANSWER_1: Self::Answer = 13;
    const ANSWER_1: Self::Answer = 5529;
    const EXAMPLE_ANSWER_2: Self::Answer = 140;
    const ANSWER_2: Self::Answer = 27690;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        let packet_pair: Vec<PacketPair> = input
            .trim()
            .split("\n\n")
            .map(|pair| {
                let (left, right) = pair.split_once('\n').expect("two packets");
                let (left, right) = (
                    left.parse::<Packet>().expect("valid left packet"),
                    right.parse::<Packet>().expect("valid right packet"),
                );
                PacketPair { left, right }
            })
            .collect_vec();
        Ok(("", packet_pair))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        let mut index_sum = 0_u32;
        for (i, pair) in input.iter().enumerate() {
            println!("{}", pair.left);
            println!("{}", pair.right);
            let res = pair.left.cmp(&pair.right);
            match res {
                Ordering::Equal => println!("{} - left = right", i + 1),
                Ordering::Less => {
                    println!("{} - left < right", i + 1);
                    index_sum += (i as u32) + 1
                }
                Ordering::Greater => {
                    println!("{} - left > right", i + 1);
                }
            }
        }
        index_sum as usize
    }

    fn part2(input: Self::Parsed) -> Self::Answer {
        let mut packets = input
            .iter()
            .flat_map(|pair| [pair.left.clone(), pair.right.clone()])
            .collect_vec();
        let divider_packet_1 = "[[2]]".parse::<Packet>().expect("");
        let divider_packet_2 = "[[6]]".parse::<Packet>().expect("");
        packets.push(divider_packet_1.clone());
        packets.push(divider_packet_2.clone());
        packets.sort();

        (packets.binary_search(&divider_packet_1).unwrap() + 1)
            * (packets.binary_search(&divider_packet_2).unwrap() + 1)
    }
}
