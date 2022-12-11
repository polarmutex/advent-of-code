use itertools::iproduct;
pub use itertools::Itertools;
use std::collections::HashMap;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         HashMap::from([
         $( ($key, $val), )*
         ])
    }}
}

pub fn ocr(letter: &String) -> char {
    let letter_map = hashmap![
        String::from(A_STR) => 'A',
        String::from(B_STR) => 'B',
        String::from(C_STR) => 'C',
        String::from(E_STR) => 'E',
        String::from(F_STR) => 'F',
        String::from(G_STR) => 'G',
        String::from(H_STR) => 'H',
        String::from(I_STR) => 'I',
        String::from(J_STR) => 'J',
        String::from(K_STR) => 'K',
        String::from(L_STR) => 'L',
        String::from(O_STR) => 'O',
        String::from(P_STR) => 'P',
        String::from(R_STR) => 'R',
        String::from(S_STR) => 'S',
        String::from(U_STR) => 'U',
        String::from(Y_STR) => 'Y',
        String::from(Z_STR) => 'Z'
    ];
    println!("input: {}", letter);
    println!("    K: {}", K_STR);
    *letter_map.get(letter).unwrap_or(&'-')
}

//const LETTER_MAP: HashMap<String, char> = hashmap![
const A_STR: &str = "\
.##.\
#..#\
#..#\
####\
#..#\
#..#";

const B_STR: &str = "\
###.\
#..#\
###.\
#..#\
#..#\
###.";

const C_STR: &str = "\
.##.\
#..#\
#...\
#...\
#..#\
.##.";

const E_STR: &str = "\
####\
#...\
###.\
#...\
#...\
####";

const F_STR: &str = "\
####\
#...\
###.\
#...\
#...\
#...";

const G_STR: &str = "\
.##.\
#..#\
#...\
#.##\
#..#\
.###";

const H_STR: &str = "\
#..#\
#..#\
####\
#..#\
#..#\
#..#";

const I_STR: &str = "\
.###\
..#.\
..#.\
..#.\
..#.\
.###";

const J_STR: &str = "\
..##\
...#\
...#\
...#\
#..#\
.##.";

const K_STR: &str = "\
#..#\
#.#.\
##..\
#.#.\
#.#.\
#..#";

const L_STR: &str = "\
#...\
#...\
#...\
#...\
#...\
####";

const O_STR: &str = "\
.##.\
#..#\
#..#\
#..#\
#..#\
.##.";

const P_STR: &str = "\
###.\
#..#\
#..#\
###.\
#...\
#...";

const R_STR: &str = "\
###.\
#..#\
#..#\
###.\
#.#.\
#..#";

const S_STR: &str = "\
.###\
#...\
#...\
.##.\
...#\
###.";

const U_STR: &str = "\
#..#\
#..#\
#..#\
#..#\
#..#\
.##.";

const Y_STR: &str = "\
#...\
#...\
.#.#\
..#.\
..#.\
..#.";

const Z_STR: &str = "\
####\
...#\
..#.\
.#..\
#...\
####";

pub fn pixel_vector_to_char_strings(pixels: &[char], num: u32) -> Vec<String> {
    // assumming 4 wide chars  with a space between
    // 6 row chars too
    assert!(num * 5 * 6 == pixels.len() as u32);
    let char_x_ranges: Vec<std::ops::Range<u32>> =
        vec![0..4, 5..9, 10..14, 15..19, 20..24, 25..29, 30..34, 35..39];
    let mut ret = vec![];
    for char_num in 0..num {
        let char_str = pixels
            .iter()
            .enumerate()
            .filter(|(i, _)| {
                iproduct!(char_x_ranges[char_num as usize].clone(), 0..6)
                    .map(|(x, y)| y * 40 + x)
                    .collect_vec()
                    .contains(&(*i as u32))
            })
            .map(|(_, v)| *v)
            .collect::<String>();
        ret.push(char_str);
    }
    ret
}
/* print single pixel code
    for (j, val) in char1.iter().enumerate() {
        print!("{}", val);
        if (j + 1) % 4 == 0 {
            println!();
        }
    }
*/
