use ahash::AHashMap;
use aoc_runner_macros::{aoc, generator, solver, solution};

#[derive(Debug, Clone)]
pub enum Command {
    CD(String),
    LS,
}

impl std::str::FromStr for Command {
    type Err = miette::Error;
    fn from_str(input: &str) -> Result<Command, Self::Err> {
        let mut iter = input.split(' ');
        match iter.next().unwrap() {
            "cd" => Ok(Command::CD(String::from(iter.next().unwrap()))),
            "ls" => Ok(Command::LS),
            _ => miette::bail!("Could not match command"),
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cmd: ")?;
        match self {
            Command::LS => write!(f, "ls"),
            Command::CD(dir) => write!(f, "cd {}", dir),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FileType {
    File(String, u64),
    Dir(String),
}
impl std::str::FromStr for FileType {
    type Err = miette::Error;
    fn from_str(input: &str) -> Result<FileType, Self::Err> {
        let (left, right) = input.split_once(' ').unwrap();

        if left == "dir" {
            Ok(FileType::Dir(String::from(right)))
        } else if left.parse::<u64>().is_ok() {
            Ok(FileType::File(
                String::from(right),
                left.parse::<u64>().unwrap(),
            ))
        } else {
            miette::bail!("Could not match filetype")
        }
    }
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cmd: ")?;
        match self {
            FileType::Dir(dir) => write!(f, "dir {}", dir),
            FileType::File(file, size) => write!(f, "{} {}", file, size),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TerminalOutput {
    command: Command,
    output: Vec<FileType>,
}

impl std::str::FromStr for TerminalOutput {
    type Err = miette::Error;
    fn from_str(input: &str) -> Result<TerminalOutput, Self::Err> {
        let lines: Vec<String> = input.lines().map(String::from).collect();
        let command = lines[0]
            .trim()
            .parse::<Command>()
            .expect("parse command enum");

        let mut output = vec![];
        for line in lines.iter().skip(1) {
            output.push(line.parse::<FileType>().unwrap());
        }

        Ok(TerminalOutput { command, output })
    }
}

impl std::fmt::Display for TerminalOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "cmd:\t{}", self.command)?;
        for file in &self.output {
            writeln!(f, "\t{}", file)?;
        }
        Ok(())
    }
}

fn calculate_dir_sizes(input: &[TerminalOutput]) -> AHashMap<String, u64> {
    let mut cwd: Vec<String> = vec![];
    let mut size_map: AHashMap<String, u64> = AHashMap::new();
    for term in input {
        match &term.command {
            Command::CD(dir) => {
                if cwd.is_empty() && dir == "/" {
                    cwd.push("/".to_string());
                } else if dir == ".." {
                    cwd.pop();
                } else {
                    cwd.push(dir.clone())
                }
            }
            Command::LS => {
                for out in &term.output {
                    match out {
                        FileType::File(_file, size) => {
                            for i in 0..cwd.len() {
                                let key = cwd[0..=i].join("/").to_string();
                                size_map
                                    .entry(key)
                                    .and_modify(|v| *v += *size)
                                    .or_insert(*size);
                            }
                        }
                        FileType::Dir(_dir) => {
                            continue;
                        }
                    }
                }
            }
        }
    }
    size_map
}

type Input = Vec<TerminalOutput>;

fn parse(input: &str) -> nom::IResult<&str, Input> {
    let term_outputs = input
        .split('$')
        .skip(1)
        .map(|cmd| {
            cmd.parse::<TerminalOutput>()
                .expect("Can parse terminal cmd")
        })
        .collect();
    Ok(("", term_outputs))
}

#[aoc(2022, day7)]
pub mod solutions {
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let (_, data) = parse(input).unwrap();
        data
    }

    #[solver(part1, gen)]
    pub fn solve_part1(input: &Input) -> u64 {
        let size_map = calculate_dir_sizes(input);
        size_map.values().filter(|v| **v <= 100_000).sum::<u64>()
    }

    #[solver(part2, gen)]
    pub fn solve_part2(input: &Input) -> u64 {
        let size_map = calculate_dir_sizes(input);

        let current_size = size_map.get("/").expect("root to be map");
        let current_freespace = 70_000_000 - current_size;
        let space_needed = 30_000_000 - current_freespace;

        *size_map
            .values()
            .filter(|v| **v > space_needed)
            .min()
            .unwrap()
    }

    #[solution(part1, gen)]
    pub fn part_1(input: &str) -> u64 {
        let data = input_generator(input);
        solve_part1(&data)
    }

    #[solution(part2, gen)]
    pub fn part_2(input: &str) -> u64 {
        let data = input_generator(input);
        solve_part2(&data)
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    "};

    #[test]
    fn part_1_example() {
        assert_eq!(super::solutions::part_1(EXAMPLE), 95437);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(super::solutions::part_2(EXAMPLE), 24933642);
    }
}
