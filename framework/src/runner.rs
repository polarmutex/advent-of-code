pub use crate::error::Error;
pub use crate::inputs::Inputs;
pub use colored::Colorize;
pub use std::io::Write;
pub use std::result::Result;

#[macro_export]
macro_rules! main {
    ($year_num:literal, $($day:ident).*$(.)?) => {
        $(mod $day;)*
        fn main() -> $crate::runner::Result<()> {
            use $crate::runner::*;
            println!(
                "\n🎄 {} {} {} {} 🎄\n",
                "Advent".bright_red().bold(),
                "of".bright_green(),
                "Code".blue().bold(),
                format!("{:>4}", $year_num).bright_magenta().bold()
            );

            let included_days: Vec<u32> = std::env::args()
                .filter_map(|v| v.parse::<u32>().ok())
                .collect();

            for day in included_days {
                println!("{}", day);
            }

            Ok(())
        }
    };
}

#[macro_export]
macro_rules! day {
    ($day_num:literal, $year_num:literal, $parse_fn:ident => $($part_fn:ident),+) => {
        pub struct DayMetadata;
        impl DayMetadata {
            pub fn number() -> u32 {
                $day_num
            }

            pub fn execute(inputs: &mut $crate::runner::Inputs) -> Result<(), $crate::error::Error> {
                use $crate::runner::*;
                const OUTPUT_WIDTH: usize = 40;

                let input = inputs.get($year_num, $day_num)?;
                let parsed = $parse_fn(&input)?;

                $({
                    let part_name = stringify!($part_fn);
                    let remaining_space = OUTPUT_WIDTH.checked_sub(part_name.len() + 1).unwrap_or(0);
                    println!(" {} {} ", "::".magenta(), part_name.bright_yellow());
                    _ = std::io::stdout().flush();

                    let result = $part_fn(&parsed)?;
                    let str_len = result.value().len() - result.control_count();
                    let remaining_space = remaining_space.checked_sub(str_len).unwrap_or(0);
                    for _ in 0..remaining_space {
                        print!(" ");
                    }
                    print!("{}", result.value());
                    _ = std::io::stdout().flush();
                })+

                Ok(())
            }
        }
    };
}
