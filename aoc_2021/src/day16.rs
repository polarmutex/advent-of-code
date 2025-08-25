use common::{solution, Answer};
use nom::IResult;
use nom::IResult as IResultSpecial;
use nom::{
    bits::complete::{tag, take},
    multi::{many0, many1, many_m_n},
    sequence::preceded,
};
use std::str;

solution!("Packet Decoder", 16);

#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
    Literal {
        version: u8,
        type_id: u8,
        value: usize,
    },
    Operator {
        version: u8,
        type_id: u8,
        packets: Vec<Packet>,
    },
}

#[allow(dead_code)]
fn convert_to_binary_from_hex(hex: &str) -> String {
    hex[2..].chars().map(to_binary).collect()
}

#[allow(dead_code)]
fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn operator(input: (&[u8], usize)) -> IResultSpecial<(&[u8], usize), Vec<Packet>> {
    // dbg!(input);
    let (input, length_type_id) = take(1_usize)(input)?;
    // dbg!(length_type_id);
    match length_type_id {
        0 => {
            let (input, length_in_bits): (_, usize) = take(15_usize)(input)?;

            let (first_bits, offset, input) = if length_in_bits % 8 > 0 {
                let (input, first): (_, u8) = take(length_in_bits % 8)(input)?;
                (Some(first), 8 - length_in_bits % 8, input)
            } else {
                (None, 0, input)
            };

            let (input, rest_of_bits): (_, Vec<u8>) =
                many_m_n(length_in_bits / 8, length_in_bits / 8, take(8_usize))(input)?;

            let bits = match first_bits {
                Some(byte) => {
                    let mut bits: Vec<u8> = vec![byte];
                    bits.extend(rest_of_bits);
                    bits
                }
                None => rest_of_bits,
            };

            let (_input, packets) = many1(parse_bits)((&bits, offset)).unwrap();

            Ok((input, packets))
        }
        1 => {
            let (input, num_subpackets) = take(11_usize)(input)?;

            let (input, packets) = many_m_n(num_subpackets, num_subpackets, parse_bits)(input)?;
            Ok((input, packets))
        }
        _ => panic!("invalid length type id"),
    }
}
fn literal(input: (&[u8], usize)) -> IResultSpecial<(&[u8], usize), u8> {
    let (input, bits) = take(4_usize)(input)?;
    Ok((input, bits))
}
fn literals(input: (&[u8], usize)) -> IResultSpecial<(&[u8], usize), (usize, usize)> {
    let (input, bits) = many0(preceded(tag(0b1, 1_usize), literal))(input)?;
    let (input, ending_literal) = preceded(tag(0b0, 1_usize), literal)(input)?;
    let mut bitshift: usize = 0;
    for byte in bits.iter() {
        bitshift = bitshift.checked_shl(4).unwrap() | *byte as usize;
    }
    let value = bitshift.checked_shl(4).unwrap() | ending_literal as usize;
    let num_parsed_bits = bits.len() * 5 + 5;
    Ok((input, (num_parsed_bits % 4, value)))
}

fn parse_bits(input: (&[u8], usize)) -> IResultSpecial<(&[u8], usize), Packet> {
    let (input, version) = take(3_usize)(input)?;
    let (input, type_id) = take(3_usize)(input)?;
    match type_id {
        4 => {
            let (input, (_skip, value)) = literals(input)?;

            Ok((
                input,
                Packet::Literal {
                    version,
                    type_id,
                    value,
                },
            ))
        }
        _ => {
            let (input, packet) = operator(input)?;
            Ok((
                input,
                Packet::Operator {
                    version,
                    type_id,
                    packets: packet,
                },
            ))
        }
    }
}

fn process_packet(packet: &Packet) -> usize {
    match packet {
        Packet::Operator {
            version,
            type_id: _,
            packets,
        } => {
            let sum: usize = packets.iter().map(process_packet).sum();
            (*version as usize) + sum
        }

        Packet::Literal { version, .. } => *version as usize,
    }
}
fn process_packet2(packet: &Packet) -> usize {
    match packet {
        Packet::Operator {
            version: _,
            type_id,
            packets,
        } => {
            let mut packets = packets.iter().map(process_packet2);
            match type_id {
                0 => packets.sum(),
                1 => packets.product(),
                2 => packets.min().unwrap(),
                3 => packets.max().unwrap(),
                5 => {
                    let a = packets.next().unwrap();
                    let b = packets.next().unwrap();
                    let c = packets.next();
                    assert_eq!(c, None);
                    if a > b {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    let a = packets.next().unwrap();
                    let b = packets.next().unwrap();
                    let c = packets.next();
                    assert_eq!(c, None);
                    if a < b {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    let a = packets.next().unwrap();
                    let b = packets.next().unwrap();
                    let c = packets.next();
                    assert_eq!(c, None);
                    if a == b {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("askfjlasf"),
            }
        }
        Packet::Literal { value, .. } => *value as usize,
    }
}

fn parse(input: &str) -> IResult<&str, Packet> {
        let arr = hex::decode(input.replace("\n", "").as_bytes()).unwrap();
        let (_, packet) = parse_bits((&arr, 0)).unwrap();
        Ok(("", packet))
    }

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, input) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
        Ok((process_packet(&input) as u64).into())
    }

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, input) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
        Ok((process_packet2(&input) as u64).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "A0016C880162017C3686B18A3D4780";

    #[test]
    fn test_part_1() {
        let input = parse(EXAMPLE).unwrap().1;
        assert_eq!(part_1(EXAMPLE).unwrap(), Answer::Number(925));
    }

    #[test]
    fn test_part_2() {
        let input = parse(EXAMPLE).unwrap().1;
        assert_eq!(part_2(EXAMPLE).unwrap(), Answer::Number(0)); // Will need actual expected value
    }
}

// const EXAMPLE1: &str = "8A004A801A8002F478";
// const EXAMPLE2: &str = "620080001611562C8802118E34";
// const EXAMPLE3: &str = "C0015000016115A2E0802F182340";
// const EXAMPLE4: &str = "A0016C880162017C3686B18A3D4780";
// const INPUT: &str = include_str!("data/16.txt");
