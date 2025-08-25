use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use anyhow as ah;
use anyhow::{anyhow, Ok};
use clap::crate_name;
use directories::ProjectDirs;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
struct CredentialStore {
    session_cookie: String,
}

pub trait CookieStore {
    fn get_session_cookie(&self) -> anyhow::Result<&str>;
    fn set_session_cookie(&mut self, session: &str) -> anyhow::Result<()>;
}

pub struct SessionFileCookieStore {
    pub session_cookie: String,
    session_file: PathBuf,
}

impl SessionFileCookieStore {
    pub fn new() -> ah::Result<SessionFileCookieStore> {
        let Some(proj_dirs) = ProjectDirs::from("com", "xenrelay", crate_name!()) else {
            return Err(anyhow!("Cannot load config directories."));
        };

        let conf_file = proj_dirs.config_local_dir().join("session.txt");

        let conf_f = OpenOptions::new().read(true).open(&conf_file);

        if let Result::Ok(f) = conf_f {
            let session_key = std::io::read_to_string(f)?;
            Ok(SessionFileCookieStore {
                session_cookie: session_key,
                session_file: conf_file,
            })
        } else {
            create_dir_all(proj_dirs.config_local_dir())?;
            Ok(SessionFileCookieStore {
                session_cookie: String::new(),
                session_file: conf_file,
            })
        }
    }
}

impl CookieStore for SessionFileCookieStore {
    fn get_session_cookie(&self) -> ah::Result<&str> {
        Ok(&self.session_cookie)
    }

    fn set_session_cookie(&mut self, session: &str) -> ah::Result<()> {
        self.session_cookie = session.to_owned();

        let mut conf_file = File::create(&self.session_file)?;
        conf_file.write_all(session.as_bytes())?;

        Ok(())
    }
}
