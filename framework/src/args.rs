use std::{borrow::Cow, path::PathBuf, str::FromStr};

use chrono::{Datelike, Utc};
use clap::{Parser, Subcommand};
use common::Part;
use url::Url;

#[derive(Parser)]
#[command(
    name = "advent_of_code",
    author = "Connor Slade <connor@connorcode.com>"
)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Commands,

    /// The session token to use for the request.
    /// If not provided, the token will be read from the environment variable `AOC_TOKEN`.
    #[arg(short, long)]
    pub token: Option<String>,
    /// The address of the Advent of Code server to use.
    #[arg(short, long, default_value = "https://adventofcode.com")]
    pub address: Url,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Verify that the session token provided is still valid
    Verify,
    /// Fetch the puzzle input for a given day and write to a file.
    /// Also creates a base solution file for the given day.
    Init(InitArgs),
}
/// Run a solution to a problem
// Run(RunArgs),
/// Run all solutions in a given year
// RunAll(RunAllArgs),
/// List all solutions for a given year
// List(ListArgs),

#[derive(Parser, Debug)]
pub struct InitArgs {
    /// A formatter that will be used to get the path for the input file.
    #[arg(short = 'l', long, default_value = "data/{{year}}/{{day:pad(2)}}.txt")]
    pub input_location: String,
    /// An inserter is a string that will be inserted into a file at a specific marker.
    /// The argument is formatted as `{location}:{marker}:{template}` or if you want to use multiple markers `{location}:{marker}:{template}:{marker}:{template}:...`.
    /// This argument can be provided multiple times.
    /// Some uses of inserters are automatically importing the new source into a module or adding the day to the list of days.
    #[arg(short, long)]
    pub inserter: Vec<Insertion>,
    /// Cancel adding an inserter to import the newly scaffolded into its years's lib.rs and adding it to the SOLUTIONS array.
    /// Equivalent to `"aoc_{{year}}/src/lib.rs|// [import_marker]|mod day_{{day:pad(2)}};\\n|// [list_marker]|day_{{day:pad(2)}}::SOLUTION,\\n    "`.
    #[arg(long)]
    pub no_default_insertions: bool,
    /// Location formatter of the file importing each solution module.
    #[arg(long, default_value = "aoc_{{year}}/src/lib.rs")]
    pub module_location: String,
    /// A formatter that will be used to get the path for the solution file.
    #[arg(short, long, default_value = "aoc_{{year}}/src/day_{{day:pad(2)}}.rs")]
    pub solution_location: String,
    /// Path to a template file that will be used to create the solution file.
    /// If not provided, a default template will be used.
    #[arg(short = 't', long)]
    pub solution_template: Option<PathBuf>,
    /// Don't create a solution file.
    /// Useful if you want to use this command with a different language or organization.
    #[arg(short, long)]
    pub no_scaffold: bool,
    /// Allows overwriting the existing solution file.
    #[arg(long)]
    pub allow_overwrite: bool,
    /// Automatically open the solution file in your editor.
    /// Only works if you are not using `--no-scaffold`.
    /// Configure the editor with the `--editor` argument.
    #[arg(short, long)]
    pub auto_open: bool,
    /// Command to open a file in your editor.
    #[arg(short, long, default_value = "code {{file}}")]
    pub editor: String,

    /// The day to fetch the input for.
    pub day: u8,
    /// The year to fetch the input for.
    #[arg(default_value_t = current_year())]
    pub year: u16,
}

#[derive(Parser)]
pub struct RunArgs {
    /// The day to run
    pub day: u8,
    /// The part to run, a or b
    pub part: Part,
    /// The year to run
    #[arg(default_value_t = current_year())]
    pub year: u16,
    /// The location of the input file, will default to `data/{year:pad(2)}/{day:pad(2)}.txt`
    #[arg(short, long)]
    pub input: Option<PathBuf>,
    /// Wether just the answer should be printed, not the execution time or other information
    #[arg(short, long)]
    pub raw: bool,
}

#[derive(Parser)]
pub struct RunAllArgs {
    /// The year to run
    #[arg(default_value_t = current_year())]
    pub year: u16,
}

#[derive(Parser)]
pub struct ListArgs {
    /// The year to list solutions for
    #[arg(default_value_t = current_year())]
    pub year: u16,
}

pub fn current_year() -> u16 {
    Utc::now().year() as u16
}

#[derive(Debug, Clone)]
pub struct Insertion {
    pub location: Cow<'static, str>,
    // (Marker, Template)
    pub parts: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

impl FromStr for Insertion {
    type Err = &'static str;

    /// `{location}:{marker}:{template}`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('|').collect::<Vec<_>>();

        if parts.len() < 3 {
            return Err("Expected at least 3 parts");
        }

        let location = parts.remove(0).to_owned();
        let parts = parts
            .chunks_exact(2)
            .map(|x| (x[0].to_owned().into(), x[1].to_owned().into()))
            .collect::<Vec<_>>();

        Ok(Self {
            location: Cow::Owned(location),
            parts,
        })
    }
}
