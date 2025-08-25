use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
};

use ah::Ok;
use anyhow as ah;
use cargo_metadata::{camino::Utf8PathBuf, Metadata, Package, PackageId, Target};
use regex::Regex;

fn curr_pack_raw<'b>(meta: &'b Metadata, curr_dir: &Path) -> Option<&'b Package> {
    meta.workspace_packages()
        .into_iter()
        .filter(|&p| curr_dir.starts_with(p.manifest_path.parent().unwrap()))
        .next()
}

fn day_map_raw<'c>(curr_package: &'c Package) -> HashMap<u8, &'c Target> {
    let day_filter: Regex = Regex::new(r"d(?:ay)?(\d{1,2})$").unwrap();
    let mut day_map: HashMap<u8, &Target> = HashMap::default();

    for target in curr_package.targets.iter() {
        let Some(captures) = day_filter.captures(&target.name) else {
            continue;
        };
        let Some(m) = captures.get(1) else {
            println!("Matched without finding a capture group: {}", target.name);
            continue;
        };
        let day_num: Result<u8, std::num::ParseIntError> = m.as_str().parse();
        match day_num {
            Result::Err(e) => {
                println!(
                    "Failed to parse num: {}, '{}' ({})",
                    target.name,
                    m.as_str(),
                    e.to_string()
                )
            }
            Result::Ok(dn) => {
                day_map.insert(dn, target);
            }
        }
    }

    day_map
}

pub fn year_from_package(pack: &Package) -> anyhow::Result<u16> {
    let year_filter = Regex::new(r"(\d{4})$").unwrap();
    let matches = year_filter
        .captures(&pack.name)
        .ok_or(ah::anyhow!("Could not find year in crate name {}", pack.name))?;
    let year_text = matches.get(1).unwrap().as_str();

    let year = year_text.parse::<u16>()?;

    Ok(year)
}

pub fn day_from_bin(bin: &Target) -> ah::Result<u8> {
    let day_filter: Regex = Regex::new(r"d(?:ay)?(\d{1,2})$").unwrap();
    let matches = day_filter
        .captures(&bin.name)
        .ok_or(ah::anyhow!("Could not find day part in binary name: {}", &bin.name))?;
    let day_text = matches.get(1).unwrap().as_str();
    let day_num = day_text.parse::<u8>()?;

    Ok(day_num)
}

pub struct WorkspaceMeta {
    pub current_directory: PathBuf,
    pub worspace_data: Metadata,
}

impl WorkspaceMeta {
    pub fn load() -> ah::Result<WorkspaceMeta> {
        let cmd = cargo_metadata::MetadataCommand::new();
        let meta = cmd.exec()?;
        let curr_dir = env::current_dir()?;
        // println!("Current directory: {}", env::current_dir()?.display());
        // println!("Workspace root: {}", meta.workspace_root);

        // TODO: Add lazy-loaded package map, current package, and day map.

        Ok(WorkspaceMeta {
            current_directory: curr_dir,
            worspace_data: meta,
        })
    }

    pub fn package_map(&self) -> HashMap<&PackageId, &Package> {
        HashMap::from_iter(self.worspace_data.packages.iter().map(|p| (&p.id, p)))
    }

    pub fn current_package(&self) -> Option<&Package> {
        // Keeping this weird setup for possible lazy loading shenanigans in the future.
        curr_pack_raw(&self.worspace_data, &self.current_directory)
    }

    pub fn get_target_for_latest_day<'a>(&'a self, curr_package: &'a Package) -> Option<&'a Target> {
        self.get_day_map(curr_package)
            .iter()
            .max_by_key(|i| i.0)
            .map(|(_, &t)| t)
    }

    pub fn get_day_map<'a>(&'a self, curr_package: &'a Package) -> HashMap<u8, &'a Target> {
        day_map_raw(curr_package)
    }

    pub fn get_year_map<'a>(&'a self) -> HashMap<u16, &'a Package> {
        let year_filter = Regex::new(r"(\d{4})$").unwrap();
        let mut year_map: HashMap<u16, &Package> = HashMap::new();

        for pack in self.worspace_data.workspace_packages() {
            let captures = year_filter.captures(&pack.name);
            let c = match captures {
                Some(cs) => cs.get(1).unwrap(),
                None => {
                    continue;
                }
            };

            let year: u16 = c.as_str().parse().unwrap();
            year_map.insert(year, pack);
        }

        year_map
    }

    pub fn get_input_folder_location(&self) -> Utf8PathBuf {
        let mut dir = self.worspace_data.workspace_root.clone();
        dir.push("input");

        dir
    }

    pub fn get_input_file_for_day(&self, year: &u16, day: &u8) -> Utf8PathBuf {
        let mut input_loc = self.get_input_folder_location();
        input_loc.push(year.to_string());
        input_loc.push(format!("{}.txt", day));

        input_loc
    }
}
