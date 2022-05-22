pub mod day1 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        input
            .lines()
            .map(|x| x.parse().expect("not an integer"))
            .collect()
    }

    pub fn part1(input: &[Int]) -> Int {
        //TODO is there a better way to do this
        for left in input {
            for right in input {
                // skip if processing same number
                if left == right {
                    continue;
                } else if left + right == 2020 {
                    return left * right;
                }
            }
        }

        // else or error
        0
    }

    pub fn part2(input: &[Int]) -> Int {
        for one in input {
            for two in input {
                for three in input {
                    // skip if processing same number
                    if (one == two) || (one == three) || (two == three) {
                        continue;
                    } else if (one + two + three) == 2020 {
                        return one * two * three;
                    }
                }
            }
        }

        // else or error
        0
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "1721
979
366
299
675
1456";

        #[test]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 514579);
        }

        #[test]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 241861950);
        }

        #[test]
        fn test_answers() {
            let input_file = "../data/2020/day1.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 55776);
            assert_eq!(part2(&input), 223162626);
        }
    }
}

/*
pub mod day2 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day2.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day3 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day3.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day4 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day4.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day5 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day5.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day6 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day6.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day7 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day7.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day8 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day8.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day9 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day9.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day10 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day10.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day11 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day11.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day12 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day12.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day13 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day13.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day14 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day14.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day15 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day15.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day16 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day16.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day17 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day17.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day18 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day18.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day19 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day19.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day20 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day20.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}
pub mod day21 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day21.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day22 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day22.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day23 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day23.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day24 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day24.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

pub mod day25 {
    pub type Int = u32;

    pub fn generator(input: &str) -> Vec<Int> {
        std::unimplemented!();
    }

    pub fn part1(input: &[Int]) -> Int {
        std::unimplemented!();
    }

    pub fn part2(input: &[Int]) -> usize {
        std::unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2020/day25.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}
*/
