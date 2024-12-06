use std::{borrow::Cow, fs, path::PathBuf, time::Instant};

use anyhow::{bail, Context, Result};
use common::{human_time, Part};

use crate::{args::RunArgs, get_year};

pub fn run(cmd: &RunArgs) -> Result<()> {
    let solutions = get_year(cmd.year);
    let solution = solutions
        .iter()
        .find(|x| x.day == cmd.day)
        .with_context(|| format!("No solution for day {} in year {}", cmd.day, cmd.year))?;

    println!(
        "[*] Running: {} ({})",
        solution.name,
        cmd.part.to_string().to_uppercase()
    );

    let path = match &cmd.input {
        Some(path) => Cow::Borrowed(path),
        None => {
            let path = PathBuf::from(format!("data/{}/{:02}.txt", cmd.year, cmd.day));
            if !path.exists() {
                bail!("Default input file does not exist. Use --input to specify a path.");
            }
            Cow::Owned(path)
        }
    };

    let input = fs::read_to_string(&*path)?.trim().replace('\r', "");

    let start = Instant::now();
    let out = match cmd.part {
        Part::One => (solution.part_1)(&input),
        Part::Two => (solution.part_2)(&input),
    };

    if cmd.raw {
        println!("{out}");
    } else {
        let time = start.elapsed().as_nanos();
        println!("[+] OUT: {} ({})", out, human_time(time));
    }

    Ok(())
}
