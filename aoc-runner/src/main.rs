// Only used for config files (at the moment)
//use std::path::PathBuf;
use std::io;
use std::io::BufRead;
use std::io::Write;

use aoc_runner::cli::*;
use aoc_runner::commands as cmds;
use clap::Parser;

fn stdin_wrapper() -> impl BufRead {
    io::stdin().lock()
}

fn stdout_wrapper() -> impl Write {
    io::stdout().lock()
}

fn main() -> anyhow::Result<()> {
    let Cli::Aoc(cli) = Cli::parse();

    match &cli.command {
        Some(Commands::Login) => cmds::login(stdin_wrapper, stdout_wrapper, cli),
        Some(Commands::Input) => cmds::input(stdin_wrapper, stdout_wrapper, cli),
        Some(Commands::Prep) => cmds::prepare(stdin_wrapper, stdout_wrapper, cli),
        Some(Commands::Run) => cmds::run(stdin_wrapper, stdout_wrapper, cli, "run"),
        Some(Commands::Test) => cmds::run(stdin_wrapper, stdout_wrapper, cli, "test"),
        Some(Commands::Bench) => cmds::benchmark(stdin_wrapper, stdout_wrapper, cli),
        None => cmds::run(stdin_wrapper, stdout_wrapper, cli, "run"),
    }
}
