use ::std::fmt::Display;
use anyhow::Result;
use colored::Colorize;
use std::ops::Mul;

pub struct FinalResult {
    pub answer: String,
    pub display: String,
}

impl Display for FinalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display.bright_green())
    }
}

pub trait ToFinalResult {
    fn to_final_answer(self) -> Result<FinalResult, anyhow::Error>;
}

/*
  u32
*/
impl ToFinalResult for u32 {
    fn to_final_answer(self) -> Result<FinalResult, anyhow::Error> {
        Ok(FinalResult {
            answer: self.to_string(),
            display: format!("{}", self.to_string().bold().bright_white()),
        })
    }
}

/*
  usize
*/
impl ToFinalResult for usize {
    fn to_final_answer(self) -> Result<FinalResult, anyhow::Error> {
        Ok(FinalResult {
            answer: self.to_string(),
            display: format!("{}", self.to_string().bold().bright_white()),
        })
    }
}

/*
  u64
*/
impl ToFinalResult for u64 {
    fn to_final_answer(self) -> Result<FinalResult, anyhow::Error> {
        Ok(FinalResult {
            answer: self.to_string(),
            display: format!("{}", self.to_string().bold().bright_white()),
        })
    }
}

/*
  String
*/
impl ToFinalResult for String {
    fn to_final_answer(self) -> Result<FinalResult, anyhow::Error> {
        Ok(FinalResult {
            answer: self.to_string(),
            display: format!("{}", self.bold().bright_white()),
        })
    }
}

/*
  MulSubmission
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MulSubmission<T: Clone + Display + Mul<Output = T>>(pub T, pub T);

// ToFinalResult for MulSubmission
impl<T: Clone + Display + Mul<Output = T>> ToFinalResult for MulSubmission<T> {
    fn to_final_answer(self) -> Result<FinalResult, anyhow::Error> {
        let result = self.0.clone().mul(self.1.clone());
        let answer = result.to_string();
        let display = result.to_string().bold().bright_white();
        let op = "x".bright_yellow();
        let eq = "=".bright_yellow();
        Ok(FinalResult {
            answer,
            display: format!("{} {} {} {} {}", self.0, op, self.1, eq, display),
        })
    }
}

// ToFinalResult fro Result<MulSubmission>
impl<T: Clone + Display + Mul<Output = T>> ToFinalResult for Result<MulSubmission<T>> {
    fn to_final_answer(self) -> Result<FinalResult, anyhow::Error> {
        let sub = self.unwrap();
        let result = sub.0.clone().mul(sub.1.clone());
        let answer = result.to_string();
        let display = result.to_string().bold().bright_white();
        let op = "x".bright_yellow();
        let eq = "=".bright_yellow();
        Ok(FinalResult {
            answer,
            display: format!("{} {} {} {} {}", sub.0, op, sub.1, eq, display),
        })
    }
}

// Display for MulSubmission
impl<T: Clone + Display + Mul<Output = T>> Display for MulSubmission<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self.0.clone().mul(self.1.clone());
        let result = result.to_string().bold().white();
        let op = "x".bright_yellow();
        let eq = "=".bright_yellow();
        //write!(f, "{} {} {} = {}", self.0, "x", self.1, result)
        write!(f, "{} {} {} {} {}", self.0, op, self.1, eq, result)
    }
}
