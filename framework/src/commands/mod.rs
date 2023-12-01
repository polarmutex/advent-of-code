pub mod all;
pub mod download;
pub mod scaffold;
pub mod submit;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Part {
    Part1,
    Part2,
}
impl Part {
    fn to_number(&self) -> u8 {
        if *self == Part::Part2 {
            2
        } else {
            1
        }
    }
}

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "git")]
#[command(about = "A fictional versioning CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    All {},
    #[command(arg_required_else_help = true)]
    Download {
        #[arg(short = 'd')]
        day: u8,
    },
    Scaffold {
        day: u8,
    },
    Submit {
        day: u8,
        #[arg(value_enum)]
        part: Part,
    },
}
