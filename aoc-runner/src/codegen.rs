use std::fs::write;

use anyhow::Result;
use cargo_metadata::camino::Utf8Path;
use liquid::ParserBuilder;
use toml_edit::{value, Array, DocumentMut, Table};

const YEAR_CARGO_TEMPLATE: &str = include_str!("templates/year-Cargo.toml");
const GITIGNORE: &str = include_str!("templates/year-gitignore");
pub fn populate_year_package(year_root: &Utf8Path, year_num: u32) -> Result<()> {
    // Cargo.toml
    let parser = ParserBuilder::with_stdlib().build()?;
    let cargo_template = parser.parse(YEAR_CARGO_TEMPLATE)?;

    let cargo_args = liquid::object!({"year": year_num});
    let cargo_rendered = cargo_template.render(&cargo_args)?;
    write(year_root.join("Cargo.toml"), cargo_rendered.as_bytes())?;

    // Gitignore
    write(year_root.join(".gitignore"), GITIGNORE.as_bytes())?;

    Ok(())
}

pub fn add_package_to_workspace(workspace_toml: &Utf8Path, year_num: u32) -> Result<()> {
    // Edit the workspace's Cargo.toml to include the new year package.
    let mut doc = std::fs::read_to_string(workspace_toml)?.parse::<DocumentMut>()?;

    let members: &mut Array = doc["workspace"]["members"]
        .as_array_mut()
        .expect("Cargo doc format not recognized");

    members.push(&year_num.to_string());

    std::fs::write(workspace_toml, doc.to_string().as_bytes())?;

    Ok(())
}

const DAY_RS: &str = include_str!("templates/dayX.rs");
pub fn generate_day_file(day_file: &Utf8Path, year_num: u32, day_num: u32) -> Result<()> {
    // Generate the day file -- eg, day4.rs
    let parser = ParserBuilder::with_stdlib().build()?;
    let rs_template = parser.parse(DAY_RS)?;

    let rs_args = liquid::object!({"year": year_num, "day": day_num});
    let rs_rendered = rs_template.render(&rs_args)?;

    write(day_file, rs_rendered.as_bytes())?;

    Ok(())
}

pub fn add_day_to_package(
    day_num: u32,
    year_num: u32,
    day_file: &Utf8Path,
    year_cargo: &Utf8Path,
    year_path: &Utf8Path,
) -> Result<()> {
    // Add the day file to the year-package's Cargo.toml
    let mut doc = std::fs::read_to_string(year_cargo)?.parse::<DocumentMut>()?;
    let bintable = doc["bin"]
        .as_array_of_tables_mut()
        .expect("Per-year cargo doc should have already had a [[bin]] table in it.");

    let mut new_table = Table::new();
    new_table["name"] = value(format!("{}-day{}", year_num, day_num));

    let partial_path = day_file.as_str().replace(year_path.as_str(), "");
    let partial_path = partial_path.trim_start_matches("/");

    new_table["path"] = value(partial_path);

    bintable.push(new_table);

    write(year_cargo, doc.to_string().as_bytes())?;

    Ok(())
}
