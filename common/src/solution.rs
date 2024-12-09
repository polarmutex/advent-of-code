use crate::Answer;
use miette::miette;

pub struct Solution {
    pub name: &'static str,
    pub day: u8,

    pub part_1: fn(&str) -> miette::Result<Answer>,
    pub part_2: fn(&str) -> miette::Result<Answer>,
}

#[macro_export]
macro_rules! solution {
    ($name:expr, $day:expr) => {
        pub const SOLUTION: $crate::Solution = $crate::Solution {
            name: $name,
            day: $day,

            part_1,
            part_2,
        };
    };
}
