use std::{
    fmt::{self, Display, Write},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
pub enum Part {
    One,
    Two,
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Part, Self::Err> {
        match s {
            "1" => Ok(Part::One),
            "2" => Ok(Part::Two),
            _ => Err("part must be `1` or `2`".to_owned()),
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(match self {
            Part::One => '1',
            Part::Two => '2',
        })
    }
}
