use std::{
    cmp::min,
    fs::{create_dir_all, write, remove_file},
    io::{BufRead, Write},
    process::Command,
    sync::Arc,
};

use anyhow::{anyhow, Context, Ok};
use chrono::{Datelike, Timelike, Utc};
use chrono_tz::US::Eastern;
use regex::Regex;
use reqwest::{blocking::ClientBuilder, cookie::Jar, Url};
use thiserror::Error;

use crate::{
    cli::{Aoc, Commands},
    codegen::{add_day_to_package, add_package_to_workspace, generate_day_file, populate_year_package},
    iodomain::{
        cargo::{day_from_bin, year_from_package, WorkspaceMeta},
        credentials::{CookieStore, SessionFileCookieStore},
    },
};

const AUTH_MESSAGE: &str = "This command doesn't implement proper authenticaion yet. Use your browser to visit and log in to the AOC website, then copy the value of the 'session' cookie, and paste it here: ";

pub fn login<T: BufRead, U: Write>(readfn: fn() -> T, writefn: fn() -> U, _cli: Aoc) -> anyhow::Result<()> {
    let (mut stdin, mut stdout) = (readfn(), writefn());

    let mut store = SessionFileCookieStore::new()?;
    write!(&mut stdout, "{}", AUTH_MESSAGE)?;
    stdout.flush()?;

    let mut cookie: String = String::new();
    stdin.read_line(&mut cookie)?;

    store.set_session_cookie(cookie.trim())?;

    println!("\nSession cookie stored successfully.");
    Ok(())
}

pub fn input<T: BufRead, U: Write>(readfn: fn() -> T, writefn: fn() -> U, cli: Aoc) -> anyhow::Result<()> {
    println!("Attempting to download input file: {:?}", &cli);
    let store = SessionFileCookieStore::new()?;
    let stored_session = store.get_session_cookie()?;
    if stored_session == "" {
        println!("Could not find session, logging in.");
        login(readfn, writefn, cli.clone())?;
    } else {
        println!("Using existing session.");
    }

    let meta = WorkspaceMeta::load()?;
    let session = store.get_session_cookie()?;

    //URL: https://adventofcode.com/2022/day/22/input

    let jar = Jar::default();
    let cookie = format!("session={}", session);
    let url = "https://adventofcode.com".parse::<Url>().unwrap();
    jar.add_cookie_str(&cookie, &url);
    let store = Arc::new(jar);

    let cb = ClientBuilder::new();
    let client = cb.cookie_provider(store).build()?;

    // Get the current year.
    let mut year = match cli.year {
        Some(y) => y,
        None => {
            let year_filter = Regex::new(r"(\d{4})$").unwrap();
            let current_package = meta
                .current_package()
                .ok_or(anyhow!("Not in a package. Please specify a year or `cd`."))?;
            let matches = year_filter.captures(&current_package.name).unwrap();
            let year_text = matches.get(1).unwrap().as_str();

            year_text.parse::<u16>()?
        }
    };

    // Get the current day.

    let day = match &cli.day {
        Some(d) => *d,
        None => {
            let date_utc = Utc::now();
            let date_est = date_utc.with_timezone(&Eastern);

            if date_est.month() == 12 {
                min(date_est.day() as u8, 25u8)
            } else {
                // Current date is the 25th of the previous year.
                year = year - 1;
                25u8
            }
        }
    };

    let input_url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let response = client.get(input_url).send()?;

    if response.status().is_success() {
        let target_name = meta.get_input_file_for_day(&year, &day);

        // Make sure the target directory exists
        let mut dir_name = target_name.clone();
        dir_name.pop();
        create_dir_all(dir_name)?;
    
        println!("Saving input to {}.", &target_name);
        write(target_name, response.text()?)?;

        // Delete existing binary.
        let year_map = meta.get_year_map();
        let Some(&package) = year_map.get(&year) else {
            return Ok(());
        };
        let day_map = meta.get_day_map(package);
        let Some(&target) = day_map.get(&day) else {
            return Ok(());
        };
        let path = meta.worspace_data.target_directory.join(&target.name);
        let _ = remove_file(path);
    }
    else 
    {
        println!("Server replied with error: {}", response.status());
        println!("Check your setup and try again. This runner does not currently have code to automatically handle this situation.");
        println!("Raw response body:\n\n{}\n", response.text()?);
    }

    Ok(())
}

pub fn prepare<T: BufRead, U: Write>(readfn: fn() -> T, writefn: fn() -> U, _cli: Aoc) -> anyhow::Result<()> {
    // Figure out which day(s) we're prepping for
    // - In November, default to Dec 1 of the current year.
    // - In December, default to the current day before 11pm EST, and the next day after 11pm EST.
    // - Otherwise, default to the Dec 25th of the previous year.

    let stamp = Utc::now().with_timezone(&Eastern);

    let year = _cli
        .year
        .and_then(|y| Some(y as u32))
        .or_else(|| match (stamp.month(), stamp.hour()) {
            (11, _) => Some(stamp.year() as u32),
            (12, _) => Some(stamp.year() as u32),
            _ => Some((stamp.year() - 1) as u32),
        })
        .unwrap();

    let day = _cli
        .day
        .and_then(|d| Some(d as u32))
        .or_else(|| match (stamp.month(), stamp.hour()) {
            (11, _) => Some(1u32),
            (12, 23) => Some((stamp.day() + 1).clamp(1, 25)),
            (12, _) => Some(stamp.day()),
            _ => Some(25),
        })
        .unwrap();

    // If the year doesn't exist yet, generate a workspace member for it (new folder, edit workspace Cargo.toml, create package Cargo.toml, create .gitignore)
    let meta = WorkspaceMeta::load()?;
    let workspace_root = meta.worspace_data.workspace_root.clone();
    let year_root = workspace_root.join(year.to_string());

    if !year_root.exists() {
        create_dir_all(&year_root.join("src"))?;
        populate_year_package(&year_root, year)?;
    }

    if !meta.get_year_map().contains_key(&(year as u16)) {
        add_package_to_workspace(&workspace_root.join("Cargo.toml"), year)?;
    }

    // If the day doesn't exist yet, generate a binary for it (new file, edit package Cargo.toml)
    let meta = WorkspaceMeta::load()?;
    let day_file = year_root.join("src").join(format!("day{}.rs", day));

    if !day_file.exists() {
        generate_day_file(&day_file, year, day)?;
    }

    let &current_package = meta
        .get_year_map()
        .get(&(year as u16))
        .expect("Could not find year package program just added.");
    let day_map = meta.get_day_map(current_package);

    if !day_map.contains_key(&(day as u8)) {
        add_day_to_package(day, year, &day_file, &year_root.join("Cargo.toml"), &year_root)?;
    }

    // Download the input file if it might be available.
    let stamp = Utc::now().with_timezone(&Eastern);
    if stamp.day() == day && stamp.year() == year as i32 {
        let input_file = meta.get_input_file_for_day(&(year as u16), &(day as u8));
        if !input_file.exists() {
            let input_args = Aoc {
                verbose: 0,
                day: Some(day as u8),
                year: Some(year as u16),
                command: Some(Commands::Input),
            };
            let res = input(readfn, writefn, input_args);
            if let Err(e) = res {
                println!("Error while downloading input: {}", e);
            }
        }
    }

    // Run an update & build cycle

    Ok(())
}

#[derive(Error, Debug)]
enum RunError {
    #[error("No targets found. Are there binaries in your Cargo.toml named similar to `day15`?")]
    NoTargetsFound,
    #[error("Could not pick out a default year. Are you currently in a year-specific crate's folder?")]
    NoYearsFound,
    #[error("Could not find year specified. Is that year a crate in your workspace?")]
    YearNotFound,
}

pub fn run<T: BufRead, U: Write>(readfn: fn() -> T, writefn: fn() -> U, cli: Aoc, cmd: &str) -> anyhow::Result<()> {
    // Get some data together
    let data = WorkspaceMeta::load()
        .context("Failed to load data for the current cargo workspace. Are you in a crate or workspace?")?;

    // for pack_id in data.worspace_data.workspace_members.iter() {
    //     println!("  Pack ID: {}", pack_id);
    //     if let Some(pack) = data.package_map().get(&pack_id) {
    //         println!("    Manifest path: {}", pack.manifest_path);
    //         for t in pack.targets.iter() {
    //             println!("    Target: {} ({})", t.name, t.src_path);
    //         }
    //     }
    // }

    match cmd {
        "run" => {}
        "test" => {}
        _ => {
            return Err(anyhow!("Invalid command given to `run`. This should not happen."));
        }
    }

    // Figure out which year we're in
    let pack = match cli.year {
        None => data.current_package().ok_or(RunError::NoYearsFound),
        Some(y) => data.get_year_map().get(&y).map(|&p| p).ok_or(RunError::YearNotFound),
    }?;

    // Figure out the selected day
    let Some(&ref target) = (match cli.day {
        None => data.get_target_for_latest_day(pack),
        Some(d) => data.get_day_map(pack).get(&d).map(|&p| p),
    }) else {
        return Err(RunError::NoTargetsFound.into());
    };

    // Try to get the input for the problem if we don't have it.
    let day_num = day_from_bin(target)?;
    let year_num = year_from_package(pack)?;

    let input_file = data.get_input_file_for_day(&year_num, &day_num);
    if !input_file.exists() {
        println!("Creating input file: {}", input_file);
        let input_args = Aoc {
            verbose: 0,
            day: Some(day_num),
            year: Some(year_num),
            command: Some(Commands::Input),
        };
        let res = input(readfn, writefn, input_args);
        if let Err(e) = res {
            println!("Error while downloading input: {}", e);
        }
    } else {
        println!("File exists: {}", input_file);
    }

    // And now, to run the target!
    println!("Running solutions for {}", target.name);

    let mut child = Command::new("cargo")
        .arg(cmd)
        .arg("--release")
        .arg("--bin")
        .arg(&target.name)
        .current_dir(pack.manifest_path.parent().unwrap())
        .spawn()?;

    child.wait()?;

    Ok(())
}

pub fn benchmark<T: BufRead, U: Write>(_readfn: fn() -> T, _writefn: fn() -> U, _cli: Aoc) -> anyhow::Result<()> {
    todo!()
}
