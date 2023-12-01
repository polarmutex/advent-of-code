use std::fmt::Display;
use std::process::Command;
use std::process::Output;
use std::process::Stdio;

#[derive(Debug)]
pub enum AocCommandError {
    CommandNotFound,
    CommandNotCallable,
    BadExitStatus(Output),
    IoError,
}

impl Display for AocCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AocCommandError::CommandNotFound => write!(f, "aoc-cli is not present in environment."),
            AocCommandError::CommandNotCallable => write!(f, "aoc-cli could not be called."),
            AocCommandError::BadExitStatus(_) => {
                write!(f, "aoc-cli exited with a non-zero status.")
            }
            AocCommandError::IoError => write!(f, "could not write output files to file system."),
        }
    }
}

pub fn check() -> Result<(), AocCommandError> {
    Command::new("aoc")
        .arg("-V")
        .output()
        .map_err(|_| AocCommandError::CommandNotFound)?;
    Ok(())
}

pub fn download(year: i32, day: u8) -> Result<Output, AocCommandError> {
    let input_path = get_input_path(year, day);

    let args = build_args(
        "download",
        &[
            "--year".into(),
            year.to_string(),
            "--overwrite".into(),
            "--input-only".into(),
            "--input-file".into(),
            input_path.to_string(),
        ],
        day,
    );

    let output = call_aoc_cli(&args)?;
    println!("---");
    println!("🎄 Successfully wrote input to \"{}\".", &input_path);
    Ok(output)
}

pub fn submit(day: u8, part: u8, result: &str) -> Result<Output, AocCommandError> {
    // workaround: the argument order is inverted for submit.
    let mut args = build_args("submit", &[], day);
    args.push(part.to_string());
    args.push(result.to_string());
    call_aoc_cli(&args)
}

fn build_args(command: &str, args: &[String], day: u8) -> Vec<String> {
    let mut cmd_args = args.to_vec();
    cmd_args.append(&mut vec!["--day".into(), day.to_string(), command.into()]);
    cmd_args
}

fn call_aoc_cli(args: &[String]) -> Result<Output, AocCommandError> {
    // println!("Calling >aoc with: {}", args.join(" "));
    let output = Command::new("aoc")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|_| AocCommandError::CommandNotCallable)?;

    if output.status.success() {
        Ok(output)
    } else {
        Err(AocCommandError::BadExitStatus(output))
    }
}

fn get_input_path(year: i32, day: u8) -> String {
    let day_padded = format!("{day:02}");
    format!("aoc_{year}/src/data/{day_padded}.txt")
}
