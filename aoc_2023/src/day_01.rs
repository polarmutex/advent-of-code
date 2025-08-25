use common::{solution, Answer};
use nom::character::complete::alphanumeric1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list1;

solution!("Trebuchet?!", 1);

type Input = Vec<String>;

fn parse(data: &str) -> nom::IResult<&str, Input> {
    let line_parser = map(alphanumeric1, String::from);
    separated_list1(line_ending, line_parser)(data)
}

fn part_1(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let result = data.iter()
        .map(|l| process_calibration_line(l, false))
        .sum::<u32>();
        
    Ok(result.into())
}

fn part_2(input: &str) -> miette::Result<Answer> {
    let (_, data) = parse(input).map_err(|e| miette::miette!("Parse error: {}", e))?;
    
    let result = data.iter()
        .map(|l| process_calibration_line(l, true))
        .sum::<u32>();
        
    Ok(result.into())
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

#[cfg(test)]
mod test {
    use common::load_raw;
    use indoc::indoc;

    const EXAMPLE1: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    const EXAMPLE2: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn part_1_example() -> miette::Result<()> {
        assert_eq!(super::part_1(EXAMPLE1)?, 142.into());
        Ok(())
    }

    #[test]
    fn part_2_example() -> miette::Result<()> {
        assert_eq!(super::part_2(EXAMPLE2)?, 281.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_1() -> miette::Result<()> {
        let input = load_raw(2023, 1)?;
        assert_eq!(super::part_1(input.as_str())?, 55538.into());
        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2() -> miette::Result<()> {
        let input = load_raw(2023, 1)?;
        assert_eq!(super::part_2(input.as_str())?, 54875.into());
        Ok(())
    }

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
        assert_eq!(expected, super::process_calibration_line(&line, true))
    }
}