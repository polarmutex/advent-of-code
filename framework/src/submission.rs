use ::std::fmt::Display;
use anyhow::Result;
use colored::Colorize;
use std::ops::Mul;

pub struct FinalResult {
    pub final_answer: String,
}

impl Display for FinalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.final_answer.bright_green().to_string())
    }
}

pub trait ToFinalResult {
    fn to_final_answer(self) -> Result<FinalResult, anyhow::Error>;
}

impl ToFinalResult for u32 {
    fn to_final_answer(self) -> Result<FinalResult, anyhow::Error> {
        Ok(FinalResult {
            final_answer: self.to_string(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MulSubmission<T: Clone + Display + Mul<Output = T>>(pub T, pub T);

impl<T: Clone + Display + Mul<Output = T>> ToFinalResult for MulSubmission<T> {
    fn to_final_answer(self) -> Result<FinalResult, anyhow::Error> {
        let result = self.0.clone().mul(self.1.clone());
        let result = result.to_string().bold().white();
        let op = "*".bright_yellow();
        let eq = "=".bright_yellow();
        Ok(FinalResult {
            final_answer: format!("{} {} {} {} {}", self.0, op, self.1, eq, result),
        })
    }
}

impl<T: Clone + Display + Mul<Output = T>> ToFinalResult for Result<MulSubmission<T>> {
    fn to_final_answer(self) -> Result<FinalResult, anyhow::Error> {
        let sub = self.unwrap();
        let result = sub.0.clone().mul(sub.1.clone());
        let result = result.to_string().bold().white();
        let op = "*".bright_yellow();
        let eq = "=".bright_yellow();
        Ok(FinalResult {
            final_answer: format!("{} {} {} {} {}", sub.0, op, sub.1, eq, result),
        })
    }
}

impl<T: Clone + Display + Mul<Output = T>> Display for MulSubmission<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self.0.clone().mul(self.1.clone());
        //write!(f, "{} {} {} = {}", self.0, "x", self.1, result)
        write!(f, "{} {} {} = {}", self.0, "x", self.1, result)
    }
}
