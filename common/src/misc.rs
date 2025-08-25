use miette::IntoDiagnostic;
use miette::Result;
use std::env;
use std::fs;
use std::path::Path;

fn find_nearest_directory_named(directory_name: &str) -> Result<String> {
    let target_directory = Path::new(directory_name);
    let exe_path = env::current_exe().into_diagnostic()?;
    let mut path = exe_path.as_path();
    while !path.join(target_directory).exists() {
        path = path.parent().unwrap();
    }
    return Ok(path.join(target_directory).to_str().unwrap().to_string());
}

/// Load the input for the given year and day.
/// Removes carriage returns and trims leading and trailing whitespace.
pub fn load(year: u16, day: u32) -> Result<String> {
    load_raw(year, day).map(|x| x.trim().replace('\r', ""))
}

/// Load the input for the given year and day.
pub fn load_raw(year: u16, day: u32) -> Result<String> {
    let data_dir_path = find_nearest_directory_named("data")?;
    let file = format!("{data_dir_path}/{year}/{:02}.txt", day);
    fs::read_to_string(file).into_diagnostic()
}

pub fn human_time(time: u128) -> String {
    const TIME_UNITS: &[&str] = &["ns", "μs", "ms", "s"];

    let mut time = time;
    for i in TIME_UNITS {
        if time < 1000 {
            return format!("{}{}", time, i);
        }
        time /= 1000;
    }

    format!("{}{}", time, TIME_UNITS.last().unwrap())
}

/// Simple OCR for Advent of Code ASCII art
pub fn pixel_vector_to_char_strings(pixels: &[char], char_width: usize) -> Vec<String> {
    let mut result = Vec::new();
    let rows = pixels.len() / 40; // Assuming 40 pixels per row
    
    for char_idx in 0..(40 / char_width) {
        let mut char_pattern = Vec::new();
        for row in 0..rows {
            let start_pixel = row * 40 + char_idx * char_width;
            let end_pixel = start_pixel + char_width;
            if end_pixel <= pixels.len() {
                let row_slice: String = pixels[start_pixel..end_pixel].iter().collect();
                char_pattern.push(row_slice);
            }
        }
        result.push(char_pattern.join("\n"));
    }
    result
}

/// OCR mapping for common Advent of Code letter patterns
pub fn ocr(pattern: &str) -> char {
    match pattern {
        // 8x6 patterns
        "####\n#  #\n#  #\n#  #\n#  #\n####" => 'O',
        "###.\n#  #\n###.\n#  #\n#  #\n###." => 'P',
        "###.\n#  #\n###.\n# ..\n# ..\n# .." => 'F',
        ".##.\n#  #\n#...\n# ##\n#  #\n.###" => 'G',
        "#...\n#...\n#...\n#...\n#...\n####" => 'L',
        "###.\n#  #\n#  #\n###.\n#.#.\n#..#" => 'R',
        ".###\n#...\n#...\n.##.\n...#\n###." => 'S',
        "####\n...#\n...#\n...#\n...#\n...#" => '7',
        "###.\n..#.\n..#.\n..#.\n..#.\n..#." => 'T',
        "#  #\n#  #\n#  #\n#  #\n#  #\n.##." => 'U',
        "#  #\n#  #\n#  #\n# ##\n##.#\n#..#" => 'K',
        "#  #\n##.#\n#.##\n#..#\n#..#\n#..#" => 'N',
        ".##.\n#  #\n#  #\n#  #\n#  #\n.##." => 'O',
        "#..#\n#..#\n####\n#..#\n#..#\n#..#" => 'A',
        "###.\n#  #\n###.\n###.\n#  #\n###." => 'B',
        ".###\n#...\n#...\n#...\n#...\n.###" => 'C',
        "###.\n#  #\n#  #\n#  #\n#  #\n###." => 'D',
        "####\n#...\n###.\n#...\n#...\n####" => 'E',
        "#  #\n#  #\n#  #\n#  #\n#  #\n####" => 'U',
        "#  #\n#  #\n.##.\n.##.\n#  #\n#  #" => 'X',
        "#  #\n#  #\n#  #\n.##.\n..#.\n..#." => 'Y',
        "####\n...#\n..#.\n.#..\n#...\n####" => 'Z',
        _ => '?',
    }
}
