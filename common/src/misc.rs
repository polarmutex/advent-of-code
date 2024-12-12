use miette::IntoDiagnostic;
use miette::Result;
use std::env;
use std::fs;
use std::path::Path;

fn find_nearest_directory_named(directory_name: &str) -> Result<String> {
    let target_directory = Path::new(directory_name);
    let exe_path = env::current_exe().into_diagnostic()?;
    let mut path = exe_path.as_path();
    while !path.join(target_directory).exists() {
        path = path.parent().unwrap();
    }
    return Ok(path.join(target_directory).to_str().unwrap().to_string());
}

/// Load the input for the given year and day.
/// Removes carriage returns and trims leading and trailing whitespace.
pub fn load(year: u16, day: u32) -> Result<String> {
    load_raw(year, day).map(|x| x.trim().replace('\r', ""))
}

/// Load the input for the given year and day.
pub fn load_raw(year: u16, day: u32) -> Result<String> {
    let data_dir_path = find_nearest_directory_named("data")?;
    let file = format!("{data_dir_path}/{year}/{:02}.txt", day);
    fs::read_to_string(file).into_diagnostic()
}

pub fn human_time(time: u128) -> String {
    const TIME_UNITS: &[&str] = &["ns", "μs", "ms", "s"];

    let mut time = time;
    for i in TIME_UNITS {
        if time < 1000 {
            return format!("{}{}", time, i);
        }
        time /= 1000;
    }

    format!("{}{}", time, TIME_UNITS.last().unwrap())
}
