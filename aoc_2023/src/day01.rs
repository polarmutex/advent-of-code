use framework::boilerplate;
use framework::tests;
use framework::IResult;
use framework::SolutionData;
// use itertools::Itertools;
use nom::character::complete::alphanumeric1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list1;

boilerplate!(
    Day,
    1,
    "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
",
    "data/01.txt"
);

impl Solution for Day {
    type Parsed = Vec<String>;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 142;
    const ANSWER_1: Self::Answer = 55538;
    const EXAMPLE_ANSWER_2: Self::Answer = 1;
    const ANSWER_2: Self::Answer = 1;

    fn parse(data: &str) -> IResult<Self::Parsed> {
        let line_parser = map(alphanumeric1, |s| String::from(s));
        separated_list1(line_ending, line_parser)(data)
    }

    fn part1(data: Self::Parsed) -> Self::Answer {
        data.iter()
            .map(|l| process_calibration_line(l, false))
            .sum::<u32>()
    }

    fn part2(data: Self::Parsed) -> Self::Answer {
        data.iter()
            .map(|l| process_calibration_line(l, true))
            .sum::<u32>()
    }
}

fn process_calibration_line(line: &String, convert_num_str: bool) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let sub_line = &line[index..];
        let result = if convert_num_str && sub_line.starts_with("one") {
            Some(1)
        } else if convert_num_str && sub_line.starts_with("two") {
            Some(2)
        } else if convert_num_str && sub_line.starts_with("three") {
            Some(3)
        } else if convert_num_str && sub_line.starts_with("four") {
            Some(4)
        } else if convert_num_str && sub_line.starts_with("five") {
            Some(5)
        } else if convert_num_str && sub_line.starts_with("six") {
            Some(6)
        } else if convert_num_str && sub_line.starts_with("seven") {
            Some(7)
        } else if convert_num_str && sub_line.starts_with("eight") {
            Some(8)
        } else if convert_num_str && sub_line.starts_with("nine") {
            Some(9)
        } else {
            sub_line.chars().next().unwrap().to_digit(10)
        };
        result
    });
    let first = it.next().expect("should be a number");
    match it.last() {
        Some(last) => first * 10 + last,
        None => first * 10 + first,
    }
}

tests! {
     const EXAMPLE1: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
     const EXAMPLE2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixtee
";

    add_test!(part1_example, test_part1_example, EXAMPLE1 => 142);
    add_test!(part1, test_part1_input, Day::INPUT_DATA => 55538);

    use rstest::rstest;

    #[rstest]
    #[case("two1nine".to_string(), 29)]
    #[case("eightwothree".to_string(), 83)]
    #[case("abcone2threexyz".to_string(), 13)]
    #[case("xtwone3four".to_string(), 24)]
    #[case("4nineeightseven2".to_string(), 42)]
    #[case("zoneight234".to_string(), 14)]
    #[case("7pqrstsixteen".to_string(), 76)]
    fn line_test(
        #[case] line: String,
        #[case] expected: u32,
    ) {
        assert_eq!(expected, process_calibration_line(&line, true))
    }

    add_test!(part2_example, test_part2_example, EXAMPLE2 => 281);
    add_test!(part2, test_part2_input, Day::INPUT_DATA => 54875);
}
