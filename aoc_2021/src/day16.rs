use crate::prelude::*;

day!(16, parse => part1, part2);

fn parse(_input: &str) -> ParseResult<Vec<u8>> {
    let i = vec![];
    Ok(i)
}

fn part1(_input: &[u8]) -> u32 {
    todo!()
}

fn part2(_input: &[u8]) -> u64 {
    todo!()
}

tests! {
    const EXAMPLE1: &str = "\
8A004A801A8002F
";
    const EXAMPLE2: &str = "\
620080001611562C8802118E34
";
    const EXAMPLE3: &str = "\
C0015000016115A2E0802F182340
";
    const EXAMPLE4: &str = "\
A0016C880162017C3686B18A3D4780
";
    const EXAMPLE5: &str = "\
C200B40A82
";
    const EXAMPLE6: &str = "\
04005AC33890
";
    const EXAMPLE7: &str = "\
880086C3E88112
";
    const EXAMPLE8: &str = "\
CE00C43D881120
";
    const EXAMPLE9: &str = "\
D8005AC2A8F0
";
    const EXAMPLE10: &str = "\
F600BC2D8F
";
    const EXAMPLE11: &str = "\
9C005AC2F8F0
";
    const EXAMPLE12: &str = "\
9C0141080250320F1802104A08
";
    const INPUT: &str = include_str!("data/16.txt");

    simple_tests!(parse, part1, part1_example1_test, EXAMPLE1 => 16);
    simple_tests!(parse, part1, part1_example2_test, EXAMPLE2 => 12);
    simple_tests!(parse, part1, part1_example3_test, EXAMPLE3 => 23);
    simple_tests!(parse, part1, part1_example4_test, EXAMPLE4 => 31);
    simple_tests!(parse, part1, part1_input_test, INPUT => 925);
    simple_tests!(parse, part2, part2_example5_test, EXAMPLE5 => 3);
    simple_tests!(parse, part2, part2_example6_test, EXAMPLE6 => 54);
    simple_tests!(parse, part2, part2_example7_test, EXAMPLE7 => 7);
    simple_tests!(parse, part2, part2_example8_test, EXAMPLE8 => 9);
    simple_tests!(parse, part2, part2_example9_test, EXAMPLE9 => 1);
    simple_tests!(parse, part2, part2_example10_test, EXAMPLE10 => 0);
    simple_tests!(parse, part2, part2_example11_test, EXAMPLE11 => 0);
    simple_tests!(parse, part2, part2_example12_test, EXAMPLE12 => 1);
    simple_tests!(parse, part2, part2_input_test, INPUT => 342997120375);
}
