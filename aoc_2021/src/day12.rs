use crate::prelude::*;

day!(12, parse => part1, part2);

fn parse(_input: &str) -> ParseResult<Vec<u8>> {
    let i = vec![];
    Ok(i)
}

fn part1(_input: &[u8]) -> u32 {
    todo!()
}

fn part2(_input: &[u8]) -> u32 {
    todo!()
}

tests! {
    const SIMPLE_EXAMPLE: &str = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

    const SLIGHTLY_LG_EXAMPLE: &str = "\
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
    const EXAMPLE: &str = "\
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
start-RW
";
    const INPUT: &str = include_str!("data/12.txt");

    simple_tests!(parse, part1, part1_simple_example_test, SIMPLE_EXAMPLE => 10);
    simple_tests!(parse, part1, part1_slightly_lg_example_test, SLIGHTLY_LG_EXAMPLE => 19);
    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 226);
    simple_tests!(parse, part1, part1_input_test, INPUT => 3369);
    simple_tests!(parse, part2, part2_simple_example_test, SIMPLE_EXAMPLE => 36);
    simple_tests!(parse, part2, part2_slightly_lg_example_test, SLIGHTLY_LG_EXAMPLE => 103);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 3509);
    simple_tests!(parse, part2, part2_input_test, INPUT => 85883);
}
