use std::{
    fs::{File, OpenOptions},
    io::Write,
    process,
};

const MODULE_TEMPLATE: &str = r#"use framework::boilerplate;
use framework::IResult;
use framework::SolutionData;

boilerplate!(
    Day,
    $DAYNUM,
    "\
",
    "data/$DAY.txt"
);

impl Solution for Day {
    type Parsed = Vec<u32>;
    type Answer = u32;
    const EXAMPLE_ANSWER_1: Self::Answer = 0;
    const ANSWER_1: Self::Answer = 0;
    const EXAMPLE_ANSWER_2: Self::Answer = 0;
    const ANSWER_2: Self::Answer = 0;

    fn parse(input: &str) -> IResult<Self::Parsed> {
        Ok(("", vec![]))
    }

    fn part1(input: Self::Parsed) -> Self::Answer {
        todo!();
    }

    fn part2(input: Self::Parsed) -> Self::Answer {
        todo!();
    }
}
"#;

fn safe_create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

pub fn handle(year: i32, day: u8) {
    let day_padded = format!("{day:02}");
    let module_path = format!("aoc_{year}/src/day{day_padded}.rs");

    let mut file = match safe_create_file(&module_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create module file: {e}");
            process::exit(1);
        }
    };

    match file.write_all(
        MODULE_TEMPLATE
            .replace("$DAYNUM", &day.to_string())
            .replace("$DAY", &day_padded)
            .as_bytes(),
    ) {
        Ok(()) => {
            println!("Created module file \"{}\"", &module_path);
        }
        Err(e) => {
            eprintln!("Failed to write module contents: {e}");
            process::exit(1);
        }
    }
}
