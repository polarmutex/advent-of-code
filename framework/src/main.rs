use crate::session::Session;
use args::{Args, Commands};
use clap::Parser;
use common::{get_year_solutions, Solution};
use miette::Result;
mod args;
#[macro_use]
mod misc;
mod commands;
mod formatter;
mod session;
mod years;

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Initialize year registrations
    years::init_years();

    let session = match &args.token {
        Some(token) => Ok(Session::new(token)),
        None => Session::from_file(),
    };

    match &args.subcommand {
        Commands::Verify => commands::verify::verify(&session?, &args.address)?,
        Commands::Timer(e) => commands::timer::timer(e)?,
        Commands::Init(e) => commands::init::init(&session?, e, &args)?,
        Commands::Run(cmd) => commands::run::run(cmd)?,
        // Commands::RunAll(cmd) => commands::run_all::run(cmd)?,
        Commands::List(cmd) => commands::list::list(cmd)?,
        Commands::Submit(e) => commands::submit::submit(&session?, e, &args)?,
    }

    Ok(())
}

fn get_year(year: u16) -> &'static [Solution] {
    get_year_solutions(year)
}
