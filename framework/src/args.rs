use std::path::PathBuf;

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
}
/// Run a solution to a problem
// Run(RunArgs),
/// Run all solutions in a given year
// RunAll(RunAllArgs),
/// List all solutions for a given year
// List(ListArgs),

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
