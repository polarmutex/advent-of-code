use crate::day::Part;
use anyhow::Result;
use std::fs::read_to_string;
use std::time::Duration;

#[derive(Debug, Default)]
pub struct Submit {
    session_token: Option<String>,
}

pub enum SubmitResult {
    TooQuick,
    Wrong,
    Right,
    AlreadyDone,
    Error,
}

impl Submit {
    pub fn new() -> Submit {
        Default::default()
    }

    fn get_session_token(&mut self) -> Result<&str> {
        if self.session_token.is_none() {
            self.session_token = Some(read_to_string("./session_token.txt")?);
        }
        Ok(self.session_token.as_ref().unwrap())
    }

    pub fn submit(
        &mut self,
        year: u32,
        day: u32,
        part: Part,
        answer: &str,
    ) -> Result<SubmitResult> {
        let session_token = self.get_session_token()?;
        let cookie = format!("session={session_token}");

        let response = ureq::post(&format!("https://adventofcode.com/{year}/day/{day}/answer"))
            .set("cookie", cookie.trim())
            .timeout(Duration::from_secs(5))
            .send_form(&[
                (
                    "level",
                    match part {
                        Part::Part1 => "1",
                        Part::Part2 => "2",
                    },
                ),
                ("answer", answer),
            ])?
            .into_string()?;

        if response.contains("You gave an answer too recently") {
            Ok(SubmitResult::TooQuick)
        } else if response.contains("That's not the right answer") {
            Ok(SubmitResult::Wrong)
        } else if response.contains("That's the right answer!") {
            Ok(SubmitResult::Right)
        } else if response.contains("You don't seem to be solving") {
            Ok(SubmitResult::AlreadyDone)
        } else {
            Ok(SubmitResult::Error)
        }
    }
}
