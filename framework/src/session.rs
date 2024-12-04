use std::{env, fs::read_to_string};

use anyhow::{Context, Result};
use scraper::Html;
use ureq::Request;
use url::Url;

pub struct Session {
    token: String,
}

impl Session {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_owned(),
        }
    }

    pub fn from_file() -> Result<Self> {
        println!("IN FROM FILE");
        let token =
            read_to_string(shellexpand::tilde("~/.config/adventofcode.session").into_owned())?;
        println!("{token}");
        Ok(Self { token })
    }

    pub fn verify(&self, address: &Url) -> Result<Option<SessionVerification>> {
        let body = ureq::get(address.as_str())
            .set("Cookie", &format!("session={}", self.token))
            .call()?
            .into_string()?;

        let document = Html::parse_document(&body);
        let user = match document.select(selector!(".user")).next() {
            Some(user) => user,
            None => return Ok(None),
        };
        let name = user
            .text()
            .next()
            .context("No username found")?
            .trim()
            .to_owned();

        Ok(Some(SessionVerification { name }))
    }
}

pub struct SessionVerification {
    pub name: String,
}

pub trait Authenticated {
    fn authenticated(self, session: &Session) -> Request;
}

impl Authenticated for Request {
    fn authenticated(self, session: &Session) -> Request {
        self.set("Cookie", &format!("session={}", session.token))
    }
}
