//! from https://gitlab.com/mbryant/aoc-2021 to use as sample for future solves
//!
//! This crate contains reasonably efficient solutions for all Advent of Code 2021 puzzles.  See
//! [AOC 2021](https://adventofcode.com/2021) for more information, including puzzle prompts.
//!
//! If you haven't used [Rust](https://www.rust-lang.org) before, these are generated docs from the
//! codebase.  They should cover my thoughts on the problem and solutions, provide an overview, and
//! allow easily browsing through the code via the `[src]` buttons on the right side of the screen.
//! This is the first time I've thought of displaying more than code via Rust docs like this, so
//! I'm curious for feedback.
//!
//! # Initial Goal
//!
//! Execute all puzzles before the JVM can start up (~800ms).
//!
//! Note: This was solidly achieved, as all puzzles run in &lt;100ms on each benchmarked system.
//! On my desktop, they run faster than `python` can cold-start (measured via `time python3 -c
//! "exit()"`)!
//!
//! # Code layout
//!
//! Each day's code is in a different module (linked at the bottom of this page), with three
//! mandatory functions: `generator`, `part1`, and `part2`.  `generator` is passed the input text,
//! parses and computes a useful intermediate representation from it, and a reference to that value
//! is passed to `part1` and `part2`.
//!
//! This allows us to focus on each part individually, as well as track the cost of parsing the
//! input.  However, it means we often end up doing duplicated work between `part1` and `part2`.
//!
//! Solutions are intended to be general, but may require constants to be changed.  For example, if
//! the input is a fixed-size grid, data structures will likely use a constant set to that fixed
//! size, since this enables storing data with less required pointer traversing.
//!
//! Due to the anemic (by modern standards) cache on my desktop machine, I frequently optimize for
//! memory efficiency rather than amount of work done by the CPU.  This may not pay off as well on
//! a system with a faster memory hierarchy.
//!
//! # Benchmarking
//!
//! Solutions have been benchmarked on a few different systems, but the main development was done
//! on an [Intel i7-6700K](`benchmarks::I6700K`).  System information and results can be found
//! under the [`benchmarks`] module.
//!
//! For the full code, including framework and benchmarking code, see [the Gitlab
//! repo](https://gitlab.com/mbryant/aoc-2021).

/// The intro day - not much interesting here.
pub mod day1 {
    /// [`u32`] experimentally appears to be a good balance between cache-efficiency and fast
    /// operations.
    pub type Int = u32;

    /// Parse each line to an integer.
    pub fn generator(input: &str) -> Vec<Int> {
        input
            .lines()
            .map(|x| x.parse().expect("Not an integer"))
            .collect()
    }

    /// Use [`Iterator::fold`] to track the number of increases along with the previous element.
    pub fn part1(input: &[Int]) -> Int {
        let (increases, _) = input
            .iter()
            .fold((0, Int::MIN), |(increases, prev), &curr| {
                (
                    if prev < curr {
                        increases + 1
                    } else {
                        increases
                    },
                    curr,
                )
            });
        increases - 1
    }

    /// Rather than comparing 3-tuples, we can recognize that two of the elements overlap and only
    /// compare the first and last.
    pub fn part2(input: &[Int]) -> usize {
        let increases = input
            .iter()
            .enumerate()
            .skip(3)
            .filter(|(i, &v)| v > input[i - 3])
            .count();
        increases
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "199
200
208
210
200
207
240
269
260
263";

        #[test]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 7);
        }

        #[test]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 5);
        }

        #[test]
        fn test_answers() {
            let input_file = "../data/2021/day1.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1448);
            assert_eq!(part2(&input), 1471);
        }
    }
}

/// More interesting parsing than [day1], but not too different or difficult.
pub mod day2 {
    /// u32 experimentally appears to be a good balance between cache-efficiency and fast
    /// operations.
    pub type Int = u32;

    /// A naive representation for operations - this might be faster to use if we could tell Rust
    /// that our integers will never be a full 32 bits long, but doesn't matter too much.
    pub enum Direction {
        Forward(Int),
        Up(Int),
        Down(Int),
    }

    /// Parsing text is the bottleneck for this day.
    ///
    /// We know the distance will be in [0..9], and the first characters of each command are
    /// unique, so we only need to look closely at two characters of the string.
    /// Additionally, we parse via ASCII rather than using [str::parse].
    pub fn generator(input: &str) -> Vec<Direction> {
        input
            .lines()
            .map(|line| {
                let dist = (line.as_bytes()[line.len() - 1] - b'0') as Int;

                match line.bytes().next() {
                    Some(b'u') => Direction::Up(dist),
                    Some(b'f') => Direction::Forward(dist),
                    Some(b'd') => Direction::Down(dist),
                    _ => unreachable!(),
                }
            })
            .collect()
    }

    /// Trivial [Iterator::fold] solution with no real optimization opportunities
    pub fn part1(input: &[Direction]) -> u32 {
        let (x, y) = input.iter().fold((0, 0), |(x, y), dir| match dir {
            Direction::Forward(dist) => (x + dist, y),
            Direction::Up(dist) => (x, y - dist),
            Direction::Down(dist) => (x, y + dist),
        });

        x * y
    }

    /// Trivial [Iterator::fold] solution with no real optimization opportunities
    pub fn part2(input: &[Direction]) -> u32 {
        let (x, y, _) = input.iter().fold((0, 0, 0), |(x, y, aim), dir| match dir {
            Direction::Forward(dist) => (x + dist, y + aim * dist, aim),
            Direction::Up(dist) => (x, y, aim - dist),
            Direction::Down(dist) => (x, y, aim + dist),
        });

        x * y
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        #[test]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 150);
        }

        #[test]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 900);
        }

        #[test]
        fn test_answers() {
            let input_file = "../data/2021/day2.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1250395);
            assert_eq!(part2(&input), 1451210346);
        }
    }
}

/// Inputs can obviously be stored as integers, rather than lists.
/// The biggest challenge today was to keep the ordering consistent between [day3::generator] and
/// [day3::part1]/[day3::part2].
pub mod day3 {
    /// All inputs are of length 12
    pub const BITS: usize = 12;

    /// An integer type large enough to hold [BITS] bits.
    pub type Int = u32;

    const _: () = assert!(std::mem::size_of::<Int>() * 8 >= BITS);

    /// Build integers from binary representation in the standard fashion.
    pub fn generator(input: &str) -> Vec<Int> {
        input
            .lines()
            .map(|line| {
                line.bytes()
                    .enumerate()
                    .rfold(0, |acc, (i, b)| {
                        if b == b'1' {
                            acc | 1 << (BITS - i - 1)
                        } else {
                            acc
                        }
                    })
                    .try_into()
                    .expect("All integers are BITS long")
            })
            .collect()
    }

    /// For each of the bits, do some fairly naive folding.
    pub fn part1(input: &[Int]) -> usize {
        let gamma: usize = (0..BITS).fold(0, |gamma, i| {
            let ones = input.iter().filter(|&num| (num & (1 << i)) != 0).count();

            if ones > input.len() / 2 {
                gamma | (1 << i)
            } else {
                gamma
            }
        });

        gamma * (((1 << BITS) - 1) & !gamma)
    }

    /// Two sets of very similar looking code, but with nothing terribly interesting.
    pub fn part2(input: &[Int]) -> usize {
        let (mut oxygen, mut co2) = (0, 0);

        let mut current = input.to_vec();
        for i in (0..BITS).rev() {
            let ones = current.iter().filter(|&num| (num & (1 << i)) != 0).count();
            let oxygen_target = (2 * ones >= current.len()) as Int;

            current.retain(|&num| (num >> i) & 1 == oxygen_target);

            if current.len() == 1 {
                oxygen = current[0] as usize;
            }
        }

        let mut current = input.to_vec();
        for i in (0..BITS).rev() {
            let ones = current.iter().filter(|&num| (num & (1 << i)) != 0).count();
            let co2_target = (2 * ones < current.len()) as Int;

            current.retain(|&num| (num >> i) & 1 == co2_target);

            if current.len() == 1 {
                co2 = current[0] as usize;
            }
        }

        oxygen * co2
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 198);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 230);
        }

        #[test]
        fn test_answers() {
            let input_file = "../data/2021/day3.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 845186);
            assert_eq!(part2(&input), 4636702);
        }
    }
}

/// It's bingo time.  The real problems start!
pub mod day4 {
    use arrayvec::ArrayVec;
    use fnv::FnvHashMap;
    use itertools::Itertools;

    /// Experimentally chosen via benchmarking.
    pub type Int = u32;

    /// All boards are 5x5.
    pub const BOARD_SIZE: usize = 5;

    /// A marker representing a previously hit square.
    pub const SENTINEL: Int = Int::MAX;

    /// We use fixed-size arrays for boards, to avoid unnecessary memory accesses.
    pub type Board = [[Int; BOARD_SIZE]; BOARD_SIZE];

    /// A game consists of some input and a set of boards.
    /// We don't need to be fancy with the representation, since
    /// this is mainly a carrier struct for the interesting data.
    #[derive(Debug, Clone)]
    pub struct Game {
        pub input: Vec<Int>,
        pub boards: Vec<Board>,
    }

    /// A fairly annoying day for parsing,  since it took me a while to figure out why
    /// [Itertools::chunks] can't be directly chained with [Iterator::map] and [Iterator::collect].
    ///
    /// Remove the first line, split the remaining into chunks of `BOARD_SIZE + 1` lines each, then
    /// split on any whitespace before converting to [Board]s.
    /// We use some temporary [ArrayVec]s to save on allocations, since otherwise we'd have to rely
    /// on the compiler to eliminate unnecessary [Vec] allocations while creating our fixed size
    /// arrays (which it doesn't).
    pub fn generator(input: &str) -> Game {
        let mut lines = input.lines();

        let drawn = lines
            .next()
            .expect("Must be multiple lines")
            .split(',')
            .map(|x| x.parse().expect("Must draw ints"))
            .collect();

        let boards = lines
            .chunks(BOARD_SIZE + 1)
            .into_iter()
            .map(|lines| {
                lines
                    .skip(1)
                    .take(BOARD_SIZE)
                    .map(|line| {
                        line.split_whitespace()
                            .map(|x| x.parse().expect("Board contains ints"))
                            .collect::<ArrayVec<_, BOARD_SIZE>>()
                            .as_slice()
                            .try_into()
                            .expect("Must be BOARD_SIZE elements")
                    })
                    .collect::<ArrayVec<_, BOARD_SIZE>>()
                    .as_slice()
                    .try_into()
                    .expect("Must be BOARD_SIZE rows")
            })
            .collect();

        Game {
            input: drawn,
            boards,
        }
    }

    /// For each number drawn, walk through all boards and cross it off.
    /// After crossing it off on a board, do a naive check for whether this move completes a bingo.
    ///
    /// This has approaches that theoretically do less work, but the constant factors (likely
    /// memory overhead of branch mispredictions) mean they lose to the naive approach.
    pub fn part1(input: &Game) -> usize {
        let mut game = input.clone();

        let (draw, board) = game
            .input
            .iter()
            .find_map(|&draw| {
                // Apply the draw to all boards
                game.boards
                    .iter_mut()
                    .enumerate()
                    .find_map(|(b, board)| {
                        (0..BOARD_SIZE).find_map(|row| {
                            (0..BOARD_SIZE).find_map(|col| {
                                if board[row][col] == draw {
                                    // Found a hit, check if we have a bingo
                                    board[row][col] = SENTINEL;

                                    if (0..BOARD_SIZE).all(|y| board[row][y] == SENTINEL)
                                        || (0..BOARD_SIZE).all(|x| board[x][col] == SENTINEL)
                                    {
                                        return Some(b);
                                    }
                                }

                                None
                            })
                        })
                    })
                    .map(|winning_board| (draw, winning_board))
            })
            .expect("Must be a winning board");

        let board_sum = game.boards[board]
            .iter()
            .map(|row| {
                row.iter()
                    .copied()
                    .filter_map(|x| (x != SENTINEL).then(|| x as usize))
                    .sum::<usize>()
            })
            .sum::<usize>();

        (draw as usize) * board_sum
    }

    /// Implemented very similarly to [part1], except we track which boards haven't yet scored a
    /// bingo and remove them from the game when they do.
    pub fn part2(input: &Game) -> usize {
        let mut game = input.clone();

        let mut remaining = game
            .boards
            .iter_mut()
            .enumerate()
            .collect::<FnvHashMap<_, _>>();

        let (draw, board) = game
            .input
            .iter()
            .find_map(|&draw| {
                // Assume two boards aren't removed at once, since the hashmap won't work well with
                // that.
                let mut removed = usize::MAX;

                remaining.retain(|&b, board| {
                    // Keep any boards that aren't a bingo.
                    !(0..BOARD_SIZE).any(|row| {
                        (0..BOARD_SIZE).any(|col| {
                            if board[row][col] == draw {
                                // Found a hit, check if we have a bingo
                                board[row][col] = SENTINEL;

                                // Are we a bingo?
                                let bingo = (0..BOARD_SIZE).all(|y| board[row][y] == SENTINEL)
                                    || (0..BOARD_SIZE).all(|x| board[x][col] == SENTINEL);

                                if bingo {
                                    removed = b;
                                }

                                bingo
                            } else {
                                false
                            }
                        })
                    })
                });

                (remaining.is_empty()).then(|| (draw, removed))
            })
            .expect("Must be a winning board");

        let board_sum = game.boards[board]
            .iter()
            .map(|row| {
                row.iter()
                    .copied()
                    .filter_map(|x| (x != SENTINEL).then(|| x as usize))
                    .sum::<usize>()
            })
            .sum::<usize>();

        (draw as usize) * board_sum
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str =
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        #[test]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 4512);
        }

        #[test]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 1924);
        }

        #[test]
        fn test_answers() {
            let input_file = "../data/2021/day4.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 46920);
            assert_eq!(part2(&input), 12635);
        }
    }
}

/// Calculate the number of segment intersections by drawing each segment on a grid and seeing how
/// many overlaps occurred.
/// This feels gross compared to using math, but the implementation worked out to be both cleaner and run faster.
///
/// This is by far the slowest of the early days, since the approach requires writing mostly
/// unpredictably to an array of [day5::BOARD_SIZE]² bytes, and the vast majority of time is spent
/// on cache misses.
pub mod day5 {
    use std::cmp::Ordering;

    /// Negative numbers aren't very ergonomic in Rust when working with unsigned integers, so we
    /// use signed integers even though all values are unsigned.
    pub type Int = i32;

    /// The largest value in our kune segments is under 1000.
    pub const BOARD_SIZE: usize = 1000;

    /// The most naive representation for line segments.
    #[derive(Debug, Copy, Clone)]
    pub struct Line {
        /// (x,y)
        start: (Int, Int),
        /// (x,y)
        end: (Int, Int),
    }

    /// A key insight is that we want all segments to be sorted in the `x` direction, since we can
    /// greatly reduce the number of comparisons by using sorting, and `x` is more cache-efficient
    /// than `y` direction.
    ///
    /// This function is full of repetitive error-"handling" code - this would be easier with a
    /// legitimate parsing library, but it's likely not worth the cost and additional dependency
    /// that would require.
    pub fn generator(input: &str) -> Vec<Line> {
        input
            .lines()
            .map(|line| {
                let mut spaces = line.split_whitespace();

                let start = spaces
                    .next()
                    .expect("Must be a start")
                    .split_once(',')
                    .expect("Must be a comma");
                let end = spaces
                    .last()
                    .expect("Must be a end")
                    .split_once(',')
                    .expect("Must be a comma");

                let segment = Line {
                    start: (
                        start.0.parse().expect("Must be integer x"),
                        start.1.parse().expect("Must be integer y"),
                    ),
                    end: (
                        end.0.parse().expect("Must be integer x"),
                        end.1.parse().expect("Must be integer y"),
                    ),
                };

                // Ensure our segments are always sorted by X for cache efficiency.
                if segment.start.0 <= segment.end.0 {
                    segment
                } else {
                    Line {
                        start: segment.end,
                        end: segment.start,
                    }
                }
            })
            .collect()
    }

    /// This was initially implemented with actual math to calculate the overlaps, but simply
    /// drawing all segments on the board was simpler, faster, and made it easier to share code
    /// with [part2].
    /// It's possible that the math-based approach would beat this in performance with more
    /// optimization work.
    pub fn part1(input: &[Line]) -> usize {
        let mut hit_count = [[0u8; BOARD_SIZE]; BOARD_SIZE];

        for line in input {
            // We're already sorted in the x direction, so only care about y.
            let increment_y = match line.start.1.cmp(&line.end.1) {
                Ordering::Equal => {
                    // Fully horizontal, so special-case this for a ~15% perf boost.
                    (line.start.0..=line.end.0).for_each(|x| {
                        hit_count[line.start.1 as usize][x as usize] += 1;
                    });
                    continue;
                }
                Ordering::Less => 1,
                Ordering::Greater => -1,
            };
            if line.start.0 != line.end.0 {
                // Only vertical and horizontal allowed
                continue;
            }

            let mut y = line.start.1;
            hit_count[y as usize][line.start.0 as usize] += 1;

            while y != line.end.1 {
                y += increment_y;
                hit_count[y as usize][line.start.0 as usize] += 1;
            }
        }

        hit_count
            .into_iter()
            .map(|row| row.into_iter().filter(|&x| x > 1).count())
            .sum()
    }

    /// Almost identical to [part1].
    pub fn part2(input: &[Line]) -> usize {
        let mut hit_count = [[0u8; BOARD_SIZE]; BOARD_SIZE];

        for line in input {
            // We're already sorted in the x direction, so only care about y.
            let increment_y = match line.start.1.cmp(&line.end.1) {
                Ordering::Equal => {
                    // Fully horizontal, so special-case this for a ~15% perf boost.
                    (line.start.0..=line.end.0).for_each(|x| {
                        hit_count[line.start.1 as usize][x as usize] += 1;
                    });
                    continue;
                }
                Ordering::Less => 1,
                Ordering::Greater => -1,
            };
            let increment_x = if line.start.0 == line.end.0 { 0 } else { 1 };

            let (mut x, mut y) = line.start;
            hit_count[y as usize][x as usize] += 1;

            while (x, y) != line.end {
                y += increment_y;
                x += increment_x;
                hit_count[y as usize][x as usize] += 1;
            }
        }

        hit_count
            .into_iter()
            .map(|row| row.into_iter().filter(|&x| x > 1).count())
            .sum()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        #[test]
        fn test_part1() {
            let input = generator(EXAMPLE);
            //assert_eq!(part1(&input), 5);
        }

        #[test]
        fn test_part2() {
            let input = generator(EXAMPLE);
            //assert_eq!(part2(&input), 12);
        }

        #[test]
        fn test_answers() {
            let input_file = "../data/2021/day5.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            //assert_eq!(part1(&input), 7269);
            //assert_eq!(part2(&input), 21140);
        }
    }
}

/// A part2-gotcha day if you don't realize the counts are all that matter.
pub mod day6 {
    /// Fish are born with a counter of 8
    pub const BIRTH_AGE: usize = 8;
    /// Fish can spawn again with a counter of 6
    pub const SPAWN_AGE: usize = 6;

    /// Experimentally faster than u32
    pub type FishCount = usize;

    /// We only track the number of fish with a given age
    pub type Fish = [FishCount; BIRTH_AGE + 1];

    /// Each input is in [0..9), so we can look at bytes rather than parsing integers.
    pub fn generator(input: &str) -> Fish {
        let mut table: Fish = Default::default();

        input
            .trim_end()
            .split(',')
            .map(|x| (x.as_bytes()[0] - b'0') as usize)
            .for_each(|age| table[age] += 1);

        table
    }

    /// For each day, shuffle the counts of all fish down by one, spawning new ones as necessary.
    pub fn fish_count<const DAYS: usize>(mut ages: Fish) -> FishCount {
        for _ in 0..DAYS {
            // Take the zeroes
            let zeroes = std::mem::take(&mut ages[0]);

            // Bump everything else down a day
            for age in 1..ages.len() {
                ages[age - 1] = ages[age];
            }

            ages[SPAWN_AGE] += zeroes;
            ages[BIRTH_AGE] = zeroes
        }

        ages.iter().sum()
    }

    /// [fish_count] for 80 iterations.
    pub fn part1(input: &Fish) -> FishCount {
        fish_count::<80>(*input)
    }

    /// [fish_count] for 256 iterations.
    pub fn part2(input: &Fish) -> FishCount {
        fish_count::<256>(*input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "3,4,3,1,2";

        #[test]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 5934);
        }

        #[test]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 26984457539);
        }

        #[test]
        fn test_answers() {
            let input_file = "../data/2021/day6.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 388739);
            assert_eq!(part2(&input), 1741362314973);
        }
    }
}

/// Use some math that's unjustified but feels approximately correct to find fast answers.
pub mod day7 {
    use itertools::Itertools;

    /// Naive parsing strategy
    pub fn generator(input: &str) -> Vec<isize> {
        input
            .trim_end()
            .split(',')
            .map(|x| x.parse().expect("Crabs are integral"))
            .sorted()
            .collect()
    }

    /// The optimal position for changes of one is the median.
    pub fn part1(input: &[isize]) -> isize {
        let target = input[input.len() / 2];

        input.iter().map(|x| (x - target).abs()).sum()
    }

    /// The optimal position for changes with a squared distance function like this appears to be
    /// near the mean.
    pub fn part2(input: &[isize]) -> usize {
        let furthest = *input.last().expect("Must be multiple crabs");

        let memoized_distances: Vec<usize> = (0..=furthest)
            .scan(0usize, |state, distance| {
                *state += distance as usize;

                Some(*state)
            })
            .collect();

        // Mean appears to be right about the correct location for some reason.
        // Rather than figure out why, let's just check a few nearby assuming the answer is `mean
        // +- rounding`.
        let mean = input.iter().sum::<isize>() / input.len() as isize;

        (mean - 1..=mean + 1)
            .map(|target| {
                input
                    .iter()
                    .map(|crab| memoized_distances[(crab - target).abs() as usize])
                    .sum()
            })
            .min()
            .expect("Must be a crab")
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

        #[test]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 37);
        }

        #[test]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 168);
        }

        #[test]
        fn test_answers() {
            let input_file = "../data/2021/day7.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 335330);
            assert_eq!(part2(&input), 92439766);
        }
    }
}

/// A fun logic problem, with parsing being the most complicated piece.
pub mod day8 {
    use arrayvec::ArrayVec;
    use itertools::Itertools;

    /// We store inputs as a bit vector with [`NUM_LEGS`] different bits.
    pub type Number = u8;

    /// There are 10 signals per input
    pub const NUM_SIGNALS: usize = 10;
    /// There are four outputs per input
    pub const NUM_OUTPUTS: usize = 4;
    /// We're using a seven segment display
    pub const NUM_LEGS: usize = 7;

    /// Store inputs in a binary representation.
    pub struct Input {
        /// Patterns are sorted by length.
        patterns: [Number; NUM_SIGNALS],
        outputs: [Number; NUM_OUTPUTS],
    }

    /// Parse each input into a bit vector, skip the `|` separator, and repeat for the outputs.
    pub fn generator(input: &str) -> Vec<Input> {
        input
            .lines()
            .map(|line| {
                let to_binary =
                    |num: &str| num.bytes().map(|b| 1 << (b - b'a')).fold(0u8, |a, b| a | b);

                let mut line = line.split(' ');

                Input {
                    patterns: line
                        .by_ref()
                        .take(10)
                        .map(to_binary)
                        .sorted_by_key(|x| x.count_ones())
                        .collect::<ArrayVec<_, NUM_SIGNALS>>()
                        .as_slice()
                        .try_into()
                        .expect("Must be NUM_SIGNALS elements"),
                    outputs: line
                        .skip(1)
                        .map(to_binary)
                        .collect::<ArrayVec<_, NUM_OUTPUTS>>()
                        .as_slice()
                        .try_into()
                        .expect("Must be NUM_OUTPUTS elements"),
                }
            })
            .collect()
    }

    /// The target output values have a unique number of legs, so we can simply count the bits in
    /// each output to determine if it's a target value.
    pub fn part1(signals: &[Input]) -> usize {
        signals
            .iter()
            .map(|input| {
                input
                    .outputs
                    .iter()
                    .filter(|output| matches!(output.count_ones(), 2 | 3 | 4 | 7))
                    .count()
            })
            .sum()
    }

    /// Logic through determining which letter corresponds to which leg over the input, then use
    /// this to convert the outputs.
    ///
    /// For example, one, seven, four, and eight have unique numbers of legs.  We can easily
    /// determine what bit represents the top leg of seven by doing `seven - one` an seeing the
    /// leftover bit.  Repeating this process quickly determines the remaining legs.
    pub fn part2(signals: &[Input]) -> usize {
        signals
            .iter()
            .map(|input| {
                //  00
                // 1  2
                // 1  2
                //  33
                // 4  5
                // 4  5
                //  66
                let mut k = [0u8; NUM_LEGS];

                // The easy ones
                let one = input.patterns[0];
                let seven = input.patterns[1];
                let four = input.patterns[2];
                let eight = input.patterns[9];

                let matches_bits = |x: Number, bits: Number| x & bits == bits;

                let three = input.patterns[3..6]
                    .iter()
                    .copied()
                    .find(|&x| matches_bits(x, one))
                    .expect("Must be a three");

                // As many things as we can do with our current info.
                k[0] = seven - one;
                k[1] = (eight - three) & four;
                k[3] = (three & four) - one;
                k[4] = (eight - three) & !k[1];
                k[6] = eight - four - k[0] - k[4];

                // Two and five are the remaining ones with 5 legs, but we only need one of them.
                let two = input.patterns[3..6]
                    .iter()
                    .copied()
                    .find(|&x| x & k[4] != 0)
                    .expect("Must be a two");

                k[2] = one & two;
                k[5] = one & !two;

                input
                    .outputs
                    .iter()
                    .map(|&x| match x {
                        x if x == one => 1,
                        x if x == seven => 7,
                        x if x == four => 4,
                        x if x == two => 2,
                        x if x == three => 3,
                        x if x == (k[0] | k[1] | k[3] | k[5] | k[6]) => 5,
                        x if x == (eight - k[2]) => 6,
                        x if x == (eight - k[3]) => 0,
                        x if x == (eight - k[4]) => 9,
                        x if x == eight => 8,
                        _ => unreachable!(),
                    })
                    .fold(0, |acc, digit| 10 * acc + digit)
            })
            .sum()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
cdfeb fcadb cdfeb cdbaf";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 26);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 61229);
        }

        #[test]
        fn test_answers() {
            let input_file = "../data/2021/day8.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 294);
            assert_eq!(part2(&input), 973292);
        }
    }
}

/// An obvious BFS day that is more efficiently solved with a linear pass and a modified
/// union-find.
pub mod day9 {
    use arrayvec::ArrayVec;
    use fnv::FnvHashMap;
    use itertools::Itertools;

    /// The input map is 100 elements wide
    pub const WIDTH: usize = 100;
    /// The input map is 100 elements tall
    pub const HEIGHT: usize = 100;
    /// The map is a grid of [`WIDTH`] x [`HEIGHT`] containing numbers in `[0,9]`.
    pub type Map = [[u8; WIDTH]; HEIGHT];

    /// Naive parsing
    pub fn generator(input: &str) -> Map {
        input
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|b| b - b'0')
                    .collect::<ArrayVec<_, WIDTH>>()
                    .as_slice()
                    .try_into()
                    .expect("Map must be WIDTH wide")
            })
            .collect::<ArrayVec<_, HEIGHT>>()
            .as_slice()
            .try_into()
            .expect("Map must be HEIGHT tall")
    }

    /// Naive solution of finding all points with strictly higher neighbors.
    pub fn part1(map: &Map) -> usize {
        map.iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, _)| {
                        let point = map[y][x];

                        let lower = [
                            map.get(y - 1).map(|row| row[x]),
                            map.get(y + 1).map(|row| row[x]),
                            map[y].get(x - 1).copied(),
                            map[y].get(x + 1).copied(),
                        ]
                        .into_iter()
                        .flatten()
                        .all(|neighbor| neighbor > point);

                        if lower {
                            (1 + point) as usize
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    /// Generate a grid containing an initial basin for each point, then do an efficient union-find
    /// to merge the initial basins and yield the actual set of basins in O(grid size) time.
    ///
    /// By walking through the grid linearly, we can assign a basin id to each square.  If we're
    /// touching an existing basin member, then we propagate that basin id, and otherwise we
    /// increment our basin counter and mark the square as part of a new basin.  Unfortunately,
    /// this runs into problems with oddly shaped basins (consider a large `V`), as starting from
    /// the top will consider them to be two different basins.
    ///
    /// We can resolve this by detecting when it occurs (when a square has two neighbors with
    /// different basin memberships) and noting that these two can eventually be merged.  After
    /// counting the basin memberships, we're left with a map from each basin to its element count
    /// and a list of basins that should be merged.
    ///
    /// To avoid implementing union-find, we create a total ordering of basin merges while creating
    /// them, allowing us to walk the merge list in a single ordered pass and yielding the final
    /// set of basins.
    pub fn part2(map: &Map) -> usize {
        type BasinId = u16;
        type Basins = [[BasinId; WIDTH]; HEIGHT];

        // Use 0 as a standin for None, since we can't use Option<NonZeroU16> in stable yet.
        const HIGH: BasinId = 0;

        let mut basin_id = HIGH + 1;
        let mut unions = FnvHashMap::default();
        let mut map: Basins = map
            .iter()
            .map(|row| {
                row.iter()
                    .copied()
                    .map(|x| if x == 9 { HIGH } else { basin_id })
                    .collect::<ArrayVec<_, WIDTH>>()
                    .as_slice()
                    .try_into()
                    .expect("Map must be WIDTH wide")
            })
            .collect::<ArrayVec<_, HEIGHT>>()
            .as_slice()
            .try_into()
            .expect("Map must be HEIGHT tall");

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if map[y][x] == HIGH {
                    continue;
                }

                // Check left and up for their contents.
                match (map.get(y - 1).map(|row| row[x]), map[y].get(x - 1).copied()) {
                    // Must be (0, 0)
                    (None, None) => (),
                    // No categorized neighbors
                    (None, Some(HIGH)) | (Some(HIGH), None) | (Some(HIGH), Some(HIGH)) => {
                        basin_id += 1;
                        map[y][x] = basin_id;
                    }
                    // Edge of a basin
                    (Some(HIGH), Some(left)) => map[y][x] = left,
                    (Some(up), Some(HIGH)) => map[y][x] = up,
                    // Nothing above us
                    (None, Some(left)) => map[y][x] = left,
                    // Nothing to the left of us
                    (Some(up), None) => map[y][x] = up,
                    // Neighbors that match
                    (Some(up), Some(left)) if left == up => map[y][x] = left,
                    // Neighbors that don't match
                    (Some(up), Some(left)) => {
                        // Two basins met, so union them together.  We'll map the lower ID to the
                        // lower ID.
                        let higher = std::cmp::max(up, left);
                        let lower = std::cmp::min(up, left);
                        unions.insert(lower, higher);

                        map[y][x] = higher;
                    }
                }
            }
        }

        let mut totals = map
            .into_iter()
            .flat_map(|row| row.into_iter())
            .filter(|&x| x != HIGH)
            .counts();

        for (lower, higher) in unions.into_iter().sorted() {
            if let Some(lower) = totals.remove(&lower) {
                *totals.entry(higher).or_default() += lower;
            }
        }

        totals.values().sorted().rev().take(3).product()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 15);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 1134);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2021/day9.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 524);
            assert_eq!(part2(&input), 1235430);
        }
    }
}

/// Not the standard interview question with paren matching, surprisingly.
pub mod day10 {
    use arrayvec::ArrayVec;
    use itertools::Itertools;

    /// We don't bother parsing lines, so just reference the input directly.
    pub type Line<'a> = &'a [u8];

    /// We assume lines are under 120 chars.
    pub const LONGEST: usize = 120;

    /// Map each line into a reference to the input bytes for it, since we don't need to do much
    /// with the input.
    pub fn generator(input: &str) -> Vec<Line> {
        input.lines().map(|line| line.as_bytes()).collect()
    }

    /// Maintain a stack of the inputs, determining which characters are invalid based on whether
    /// they match the expected value on the top of the stack.
    pub fn part1(lines: &[Line]) -> usize {
        lines
            .iter()
            .filter_map(|&line| {
                // A fixed stack allocation that can be reused across iterations.
                let mut stack = ArrayVec::<u8, LONGEST>::new();

                for &b in line.iter() {
                    match b {
                        // Openers
                        b'(' => stack.push(b')'),
                        b'[' => stack.push(b']'),
                        b'{' => stack.push(b'}'),
                        b'<' => stack.push(b'>'),

                        // Closers
                        b')' | b']' | b'}' | b'>' => {
                            let expected = stack
                                .pop()
                                .expect("Only invalid characters allowed, not missing openers");

                            if b != expected {
                                return Some(match b {
                                    b')' => 3,
                                    b']' => 57,
                                    b'}' => 1197,
                                    b'>' => 25137,
                                    _ => unreachable!(),
                                });
                            }
                        }

                        _ => unreachable!(),
                    }
                }

                None
            })
            .sum()
    }

    /// Maintain a stack of the inputs, filling in the remainder of the input with the stack's
    /// contents once the input terminates.
    pub fn part2(lines: &[Line]) -> usize {
        let autocompletes = lines
            .iter()
            .filter_map(|&line| {
                let mut stack = ArrayVec::<u8, LONGEST>::new();

                for &b in line.iter() {
                    match b {
                        // Openers
                        b'(' => stack.push(b')'),
                        b'[' => stack.push(b']'),
                        b'{' => stack.push(b'}'),
                        b'<' => stack.push(b'>'),

                        // Closers
                        _ => {
                            let expected = stack
                                .pop()
                                .expect("Only invalid characters allowed, not missing openers");

                            if b != expected {
                                return None;
                            }
                        }
                    }
                }

                Some(stack.into_iter().rev().fold(0, |score, next| {
                    score * 5
                        + match next {
                            b')' => 1,
                            b']' => 2,
                            b'}' => 3,
                            b'>' => 4,
                            _ => unreachable!(),
                        }
                }))
            })
            .sorted()
            .collect::<Vec<_>>();

        autocompletes[autocompletes.len() / 2]
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        #[test]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 26397);
        }

        #[test]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 288957);
        }

        #[test]
        fn test_answers() {
            let input_file = "../data/2021/day10.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 411471);
            assert_eq!(part2(&input), 3122628974);
        }
    }
}

/// [`day11::flash`] each octopus, recursing into the neighboring octopuses whenever one flashes.
///
/// It feels like there should be a better way than doing these recursions, but I didn't quickly
/// come up with it.
pub mod day11 {
    use arrayvec::ArrayVec;

    /// The input grid is 10 elements wide
    pub const WIDTH: usize = 10;
    /// The input grid is 10 elements high
    pub const HEIGHT: usize = 10;

    /// Size is small enough that we don't need to worry about the cache, so pick [`u32`] to make
    /// the ALU happier.
    pub type Octopus = u32;
    /// A grid of octopuses
    pub type OctopusGrid = [[Octopus; WIDTH]; HEIGHT];

    /// Naive parsing approach
    pub fn generator(input: &str) -> OctopusGrid {
        input
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|b| (b - b'0') as Octopus)
                    .collect::<ArrayVec<_, WIDTH>>()
                    .as_slice()
                    .try_into()
                    .expect("Grid must be WIDTH wide")
            })
            .collect::<ArrayVec<_, HEIGHT>>()
            .as_slice()
            .try_into()
            .expect("Grid must be HEIGHT tall")
    }

    /// The set of offsets to reach the neighbors of a square.
    pub const NEIGHBORS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    /// Given a coordinate on the grid, attempt to flash by increasing the energy level of the
    /// octopus at that coordinate, then recursing to any other octopuses that should attempt to
    /// flash as a result.
    ///
    /// Returns the number of octopuses that flashed.
    pub fn flash(grid: &mut OctopusGrid, x: isize, y: isize) -> usize {
        // Quick bounds check.
        let element = grid
            .get_mut(y as usize)
            .and_then(|row| row.get_mut(x as usize));

        if let Some(element) = element {
            *element += 1;

            if *element == 10 {
                // We just flashed, so flash everything else.
                return NEIGHBORS
                    .iter()
                    .map(|neighbor| flash(grid, x + neighbor.1, y + neighbor.0))
                    .sum::<usize>()
                    + 1;
            };
        }

        0
    }

    /// [`flash`] for 100 steps.
    pub fn part1(grid: &OctopusGrid) -> usize {
        let mut grid = *grid;
        let mut flashed = 0;

        for _ in 0..100 {
            // Flash the whole grid to start with.
            let local_flashed = (0..HEIGHT)
                .map(|y| {
                    (0..WIDTH)
                        .map(|x| flash(&mut grid, x as isize, y as isize))
                        .sum::<usize>()
                })
                .sum::<usize>();

            // Clean and count the flashed squares.
            for row in grid.iter_mut() {
                for x in row.iter_mut() {
                    if *x > 9 {
                        *x = 0;
                    }
                }
            }

            flashed += local_flashed;
        }

        flashed
    }

    /// [`flash`] until we've reached a step where [`WIDTH`]*[`HEIGHT`] octopuses flashed at once.
    pub fn part2(grid: &OctopusGrid) -> usize {
        let mut grid = *grid;

        for step in 1.. {
            // Flash the whole grid to start with.
            let flashed = (0..HEIGHT)
                .map(|y| {
                    (0..WIDTH)
                        .map(|x| flash(&mut grid, x as isize, y as isize))
                        .sum::<usize>()
                })
                .sum::<usize>();

            if flashed == WIDTH * HEIGHT {
                return step;
            }

            // Clean the flashed squares.
            for row in grid.iter_mut() {
                for x in row.iter_mut() {
                    if *x > 9 {
                        *x = 0;
                    }
                }
            }
        }

        unreachable!()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        #[test]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 1656);
        }

        #[test]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 195);
        }

        #[test]
        fn test_answers() {
            let input_file = "../data/2021/day11.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 1755);
            assert_eq!(part2(&input), 212);
        }
    }
}

/// Depth-first search using a graph implemented as a bit-based adjacency matrix.
pub mod day12 {
    use indexmap::set::IndexSet;

    /// An id representing this node, which also serves as its index into a [`NodeSet`].
    /// This allows us to avoid string comparisons in favor of more efficient integer operations.
    pub type Node = usize;

    /// We represent a set of nodes as a bit vector, and a [`u32`] can fit all `MAX_NODES` into it.
    pub type NodeSet = u32;
    pub const _: () = assert!(std::mem::size_of::<NodeSet>() * 8 >= MAX_NODES);

    /// We represent the graph structure via an adjacency matrix, and also store a list of which
    /// caves are small.
    pub struct Graph {
        nodes: [NodeSet; MAX_NODES],
        small_caves: NodeSet,
    }

    /// We arbitrarily represent the destination as 0
    pub const DEST: usize = 0;
    /// We arbitrarily represent the source as 0
    pub const SRC: usize = 1;
    /// There are at most 32 nodes in the cave system
    pub const MAX_NODES: usize = 32;

    /// Associate each edge in an adjacency matrix, and assign a unique index to each node.
    pub fn generator(input: &str) -> Graph {
        let mut graph = Graph {
            nodes: Default::default(),
            small_caves: (1 << SRC) | (1 << DEST),
        };

        // Represent nodes as usize since string comparisons are expensive.
        let mut nodes = IndexSet::with_capacity(MAX_NODES);

        // We insert the source and dest nodes immediately, since we want to use their IDs as
        // constants.
        nodes.insert("end");
        nodes.insert("start");

        // Parse each edge into a more efficient representation, then to an adjacency matrix.
        for (a, b) in input
            .lines()
            .map(|line| line.split_once('-').expect("Edges have two nodes"))
        {
            // Convert each node to its index.
            let a_index = nodes.insert_full(a);
            let b_index = nodes.insert_full(b);

            // If this is the first time seeing this node, figure out if it's a small cave.
            if a_index.1 && a.chars().all(|c| c.is_ascii_lowercase()) {
                graph.small_caves |= 1 << a_index.0;
            }
            if b_index.1 && b.chars().all(|c| c.is_ascii_lowercase()) {
                graph.small_caves |= 1 << b_index.0;
            }

            graph.nodes[a_index.0] |= 1 << b_index.0;
            graph.nodes[b_index.0] |= 1 << a_index.0;
        }

        graph
    }

    /// Recursively DFS through the graph, counting all of the paths.
    ///
    /// This is slightly complicated by the ability to visit small caves at most twice, requiring
    /// us to track how many small caves have been visited in our recursive calls.
    pub fn counter(graph: &Graph, mut visited: NodeSet, src: Node, small_visited: bool) -> usize {
        if src == DEST {
            // We made it!
            return 1;
        }

        let mut paths = 0;

        // Temporarily add ourselves in.
        visited |= 1 << src;

        let mut neighbors = graph.nodes[src as usize] as i32;
        while neighbors != 0 {
            let neighbor = neighbors.trailing_zeros() as usize;
            let remove_lowest: i32 = neighbors & -neighbors;

            if (visited & (1 << neighbor) != 0) && (graph.small_caves & (1 << neighbor) != 0) {
                // Can't visit the source multiple times.
                if !small_visited && neighbor != SRC {
                    // We're in a small cave, so let's try visiting it twice.
                    paths += counter(graph, visited, neighbor, true);
                }
            } else {
                // Big cave, maybe we've been here, maybe not.
                paths += counter(graph, visited, neighbor, small_visited);
            }

            neighbors ^= remove_lowest;
        }

        paths
    }

    /// Directly calls [`counter`], pretending that we've already visited a small cave.
    pub fn part1(graph: &Graph) -> usize {
        counter(graph, 0, SRC, true)
    }

    /// Directly calls [`counter`].
    pub fn part2(graph: &Graph) -> usize {
        counter(graph, 0, SRC, false)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

        #[test]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 226);
        }

        #[test]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 3509);
        }

        #[test]
        fn test_answers() {
            let input_file = "../data/2021/day12.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 3369);
            assert_eq!(part2(&input), 85883);
        }
    }
}

/// Trivial code to fold each coordinate repeatedly.
pub mod day13 {
    use fnv::FnvHashSet;

    /// The largest coordinate is well under 64k, so we use a u16 to fit [`Fold`] into 4 bytes
    /// (after the tag and padding).
    pub type Dim = u16;

    /// Represents a coordinate on the input paper.
    pub type Coord = (Dim, Dim);

    /// The two dimensions we can fold in.
    #[derive(Clone, Copy)]
    pub enum Fold {
        X(Dim),
        Y(Dim),
    }

    /// Papers have various coordinates and also get folded many times.
    pub struct Paper {
        pub coords: FnvHashSet<Coord>,
        pub folds: Vec<Fold>,
    }

    /// Obvious parsing code, but only looking at the last characters of the folding instructions.
    pub fn generator(input: &str) -> Paper {
        let mut lines = input.lines();

        Paper {
            coords: lines
                .by_ref()
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    let (x, y) = line.split_once(',').expect("Two parts to coords");

                    (
                        x.parse().expect("x-coords are integers"),
                        y.parse().expect("y-coords are integers"),
                    )
                })
                .collect(),
            folds: lines
                .map(|x| {
                    let fold = &x[11..];
                    let num = fold[2..].parse().expect("Need a coordinate to fold on");

                    match &fold.bytes().next().expect("Need a direction to fold on") {
                        b'x' => Fold::X(num),
                        b'y' => Fold::Y(num),
                        _ => unreachable!(),
                    }
                })
                .collect(),
        }
    }

    /// Run the same code as [`part2`] for only one interation.
    pub fn part1(paper: &Paper) -> usize {
        paper
            .coords
            .iter()
            .map(|&(x, y)| match paper.folds[0] {
                Fold::X(f) if x > f => (f - (x - f), y),
                Fold::Y(f) if y > f => (x, f - (y - f)),
                _ => (x, y),
            })
            .collect::<FnvHashSet<_>>()
            .len()
    }

    /// For each coordinate, iterate through the folds to determine where the coordinate will end
    /// up, then de-duplicate coordinates.
    ///
    /// NOTE: The output is human-readable, so we just print it the correct value rather than
    /// writing code to determine what it is.
    pub fn part2(paper: &Paper) -> &str {
        let folded = paper
            .coords
            .iter()
            .map(|&coord| {
                paper.folds.iter().fold(coord, |(x, y), &fold| match fold {
                    Fold::X(f) if x > f => (f - (x - f), y),
                    Fold::Y(f) if y > f => (x, f - (y - f)),
                    _ => (x, y),
                })
            })
            .collect::<FnvHashSet<_>>();

        // The output is easily interpretable by hand, but isn't interesting to parse via code.
        if cfg!(debug_assertions) {
            for y in 0..6 {
                for x in 0..=40 {
                    if folded.contains(&(x, y)) {
                        print!("#");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }

            "See output above"
        } else {
            "BCZRCEAB"
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

        #[test]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 17);
        }

        #[test]
        fn test_part2() {
            //let input = generator(EXAMPLE);
            //assert_eq!(part2(&input), 5);
        }

        #[test]
        fn test_answers() {
            //let input_file = "../data/2021/day13.txt";
            //let input_str =
            //   read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            //let input = generator(&input_str);
            //assert_eq!(part1(&input), 1448);
            //assert_eq!(part2(&input), 1471);
        }
    }
}

/// We map each pair of elements to its count, then update the counts according to the replacement
/// rules, allowing us to compute each step in O(rules) time.
///
/// To avoid using a set for our common accesses, we use a variant of perfect hashing to let us
/// track pairs via an array.
pub mod day14 {
    use arrayvec::ArrayVec;
    use indexmap::set::IndexSet;
    use itertools::Itertools;

    /// The template is 20 elements long.
    pub const MAX_TEMPLATE: usize = 20;

    /// There are 100 rules.
    pub const MAX_RULES: usize = 100;

    /// We represent pairs via hashes, which we'll operate a lot on.
    pub type Hash = u32;

    /// Rules map one pair to two new pairs via insertion.
    pub type Rule = (Hash, (Hash, Hash));

    /// Polymers contain a template and rules, and we also track the set of hashes to allow us to
    /// reverse our pairs (including in rules) to their original elements.
    pub struct Polymer {
        pub template: ArrayVec<u8, MAX_TEMPLATE>,
        pub rules: ArrayVec<Rule, MAX_RULES>,
        pub hashes: IndexSet<(u8, u8)>,
    }

    /// We only care about pairs of elements, and in particular only care about the pairs that we
    /// see in the pair rules.  As such, we can hash everything, allowing us to use a small array
    /// and indexing rather than a set.
    ///
    /// We then go a step further, using [`IndexSet`] as a stand-in for perfect hashing, allowing us to
    /// use a compressed array for bonus cache efficiency.
    pub fn generator(input: &str) -> Polymer {
        let mut lines = input.lines();
        let mut hashes = IndexSet::with_capacity(MAX_RULES);

        Polymer {
            template: lines
                .next()
                .expect("Must be a template")
                .bytes()
                .map(|b| b - b'A')
                .collect(),
            rules: lines
                .skip(1)
                .map(|x| x.as_bytes())
                .map(|x| {
                    let a = x[0] - b'A';
                    let b = x[1] - b'A';
                    let c = x[6] - b'A';

                    (
                        hashes.insert_full((a, b)).0 as Hash,
                        (
                            hashes.insert_full((a, c)).0 as Hash,
                            hashes.insert_full((c, b)).0 as Hash,
                        ),
                    )
                })
                .collect(),
            hashes,
        }
    }

    /// Run the polymerization process for the given amount of steps, then reverse the element
    /// identifiers to count the individual characters.
    ///
    /// Using const generics here doesn't appear to provide any speedup.
    pub fn polymerize(input: &Polymer, steps: usize) -> usize {
        let mut polymer = [0; MAX_RULES];

        input
            .template
            .iter()
            .tuple_windows()
            .filter_map(|(&a, &b)| input.hashes.get_full(&(a, b)).map(|(i, _)| i))
            .for_each(|h| polymer[h as usize] += 1);

        for _ in 0..steps {
            let mut new_polymer = [0; MAX_RULES];

            for &(hash, (replace_a, replace_b)) in input.rules.iter() {
                let count = polymer[hash as usize];

                new_polymer[replace_a as usize] += count;
                new_polymer[replace_b as usize] += count;
            }

            polymer = new_polymer;
        }

        // Convert back to the characters we need.
        let polymer = polymer
            .into_iter()
            .enumerate()
            .filter(|&(_, count)| count != 0)
            .map(|(i, count)| {
                (
                    *input
                        .hashes
                        .get_index(i)
                        .expect("Can't have stored to a non-existent hash"),
                    count,
                )
            });

        let mut counts = [0; 26];

        // Only take the second half to avoid double-counting.
        for ((_, b), count) in polymer {
            counts[b as usize] += count;
        }

        counts.iter().max().expect("Must be a largest")
            - counts
                .into_iter()
                .filter(|&x| x != 0)
                .min()
                .expect("Must be a smallest")
    }

    /// Directly calls [`polymerize`].
    pub fn part1(input: &Polymer) -> usize {
        polymerize(input, 10)
    }

    /// Directly calls [`polymerize`].
    pub fn part2(input: &Polymer) -> usize {
        polymerize(input, 40)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        #[test]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 1588);
        }

        #[test]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 2188189693529);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2021/day14.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 3831);
            assert_eq!(part2(&input), 5725739914282);
        }
    }
}

/// Use Djikstra's to traverse the grid, as there isn't a good heuristic for A*.
///
/// All real work is done in [`day15::a_star`].
///
/// The main difficulty was misreading the instructions and tiling incorrectly.
/// Rust also doesn't appear to have an efficient priority queue, so we emulate one with a
/// [`std::collections::BTreeMap`] and [`Vec`].
pub mod day15 {
    use arrayvec::ArrayVec;
    use std::collections::BTreeMap;

    /// The input map is 100x100.
    pub const INPUT_DIM: usize = 100;

    /// We're limited by memory here, so want the smallest datatypes we can use.
    pub type Coord = i16;

    /// We're limited by memory here, so want the smallest datatypes we can use.
    pub type Cost = i16;

    /// We knows the input will be in [0, 9].
    pub type Map = [[u8; INPUT_DIM]; INPUT_DIM];

    /// Parse directly to a 2D array.
    pub fn generator(input: &str) -> Map {
        input
            .lines()
            .map(|line| {
                line.trim_end()
                    .bytes()
                    .map(|x| x - b'0')
                    .collect::<ArrayVec<_, INPUT_DIM>>()
                    .as_slice()
                    .try_into()
                    .expect("Must be INPUT_DIM wide")
            })
            .collect::<ArrayVec<_, INPUT_DIM>>()
            .as_slice()
            .try_into()
            .expect("Must be INPUT_DIM high")
    }

    /// Implement A* with a heuristic of 0, also known as Djikstra's.
    ///
    /// We build our own approximation of a priority queue using a [`std::collections::BTreeMap`]
    /// mapping priorities to a list of coordinates at that priority.  This avoids tons of
    /// inefficient hashing/reheaping, since a BTree generally has friendly memory-use patterns.
    /// This is possibly only practical due to the restricted range of priorities we have.
    pub fn a_star<const DIM: usize>(input: &Map) -> Cost {
        let mut frontier = BTreeMap::<Cost, Vec<(Coord, Coord)>>::new();
        let mut best_score = [[Cost::MAX; DIM]; DIM];

        let start_node = (0, 0);
        let goal_node = (DIM as Coord - 1, DIM as Coord - 1);

        frontier.insert(0, vec![start_node]);
        best_score[start_node.1 as usize][start_node.0 as usize] = 0;

        while let Some(&current_score) = frontier.keys().next() {
            let priority_vec = frontier
                .remove(&current_score)
                .expect("Just found this key in the tree");

            // It's pretty rare to add a key that's lower than our current one, so we process keys
            // in batches for efficiency.
            for current in priority_vec.into_iter() {
                if best_score[current.1 as usize][current.0 as usize] != current_score {
                    // There's already a better occurrence of this in the heap, so ignore this.
                    continue;
                }

                if current == goal_node {
                    return current_score;
                }

                let neighbors = [(-1, 0), (0, -1), (1, 0), (0, 1)]
                    .into_iter()
                    .map(|(diff_x, diff_y)| (current.0 + diff_x, current.1 + diff_y))
                    .filter(|&(diff_x, diff_y)| {
                        diff_x >= 0 && diff_y >= 0 && diff_x < DIM as Coord && diff_y < DIM as Coord
                    })
                .map(|(x, y)| {
                    let cost = input[y as usize % INPUT_DIM][x as usize % INPUT_DIM] as Cost;

                    let cost = if DIM > INPUT_DIM {
                        let increase = y / INPUT_DIM as Cost + x / INPUT_DIM as Cost;
                        let wrapped_cost = cost + increase;

                        if wrapped_cost > 9 {
                            assert!(9 > 2 * (DIM / INPUT_DIM - 1), "Our dimension is small enough that we can emulate modulus with subtraction");
                            wrapped_cost - 9
                        } else {
                            wrapped_cost
                        }
                    } else {
                        cost
                    };

                    ((x, y), cost)
                });

                for (neighbor, neighbor_cost) in neighbors {
                    let tentative_score = current_score + neighbor_cost as Cost;
                    let current_best = best_score[neighbor.1 as usize][neighbor.0 as usize];

                    if tentative_score < current_best {
                        // We found a better path to neighbor.
                        best_score[neighbor.1 as usize][neighbor.0 as usize] = tentative_score;

                        // NOTE: Manhattan distance doesn't seem to be useful for some reason,
                        // so we fall back to essentially Djikstra's.
                        let distance_buf = frontier
                            .entry(tentative_score)
                            // Use a somewhat arbitrarily chosen capacity.
                            .or_insert_with(|| Vec::with_capacity(DIM / 2));
                        distance_buf.push(neighbor);
                    }
                }
            }
        }

        unreachable!()
    }

    /// Directly calls [`a_star`] on the input.
    pub fn part1(input: &Map) -> Cost {
        a_star::<INPUT_DIM>(input)
    }

    /// Directly calls [`a_star`] on the input.
    pub fn part2(input: &Map) -> Cost {
        a_star::<{ 5 * INPUT_DIM }>(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 40);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 315);
        }

        #[test]
        fn test_answers() {
            let input_file = "../data/2021/day15.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 609);
            assert_eq!(part2(&input), 2925);
        }
    }
}

/// Use a custom [`read N bits`](`day16::read`) primitive to implement a simple recursive parser.
pub mod day16 {
    /// Parse every two characters as a hex character.
    pub fn generator(input: &str) -> Vec<u8> {
        // Skip the NUL byte
        (0..input.len() - 1)
            .step_by(2)
            .map(|i| u8::from_str_radix(&input[i..i + 2], 16).expect("All characters are hex"))
            .collect()
    }

    /// Represents an index for a bit inside a byte array.
    pub type BitIndex = usize;

    /// Read `N` bits from the input starting at `start_bits`.
    ///
    /// We naively read 1 bit at a time, since it's pretty fast either way.  Hopefully `rustc`
    /// optimizes this.
    pub fn read<const N: usize>(input: &[u8], start_bits: BitIndex) -> usize {
        const BITS: usize = u8::BITS as usize;

        let mut result = 0;

        for i in 0..N {
            let index = (start_bits + i) / BITS;
            let offset = (start_bits + i) % BITS;
            let next_bit = (input[index] >> (BITS - 1 - offset)) & 0x1;

            result = result << 1 | next_bit as usize;
        }

        result
    }

    /// Recursively parse the input from the `start_bits` bit, returning a computed value as well
    /// as the next bit that should be read.
    ///
    /// If `SUM_VERSIONS` is `true`, the value returned will be the sum of all subpacket versions.
    /// Otherwise, the value returned will be the result of its expression.
    pub fn parse<const SUM_VERSIONS: bool>(
        input: &[u8],
        start_bits: BitIndex,
    ) -> (usize, BitIndex) {
        let mut bits = start_bits;

        let version = read::<3>(input, bits);
        let type_id = read::<3>(input, bits + 3);
        bits += 6;

        let value = match type_id {
            4 => {
                let mut value = 0;
                loop {
                    let continue_reading = read::<1>(input, bits);
                    value = value << 4 | read::<4>(input, bits + 1);
                    bits += 5;

                    if continue_reading == 0 {
                        break;
                    }
                }

                if SUM_VERSIONS {
                    version
                } else {
                    value
                }
            }
            _ => {
                // Operator packet
                let length_id = read::<1>(input, bits);
                bits += 1;

                // Helper type to make it easier to share the big while loop below.
                enum EndSubpackets {
                    Bits(usize),
                    Count(usize),
                }

                let subpacket_condition = if length_id == 0 {
                    // Next 15 bits represent total length in bits of sub-packets contained in this
                    // packet.
                    let end_subpacket_bits = read::<15>(input, bits) + 15 + bits;
                    bits += 15;

                    EndSubpackets::Bits(end_subpacket_bits)
                } else {
                    // Next 11 bits represent the number of sub-packets immediately contained by this
                    // packet.
                    let subpacket_count = read::<11>(input, bits);
                    bits += 11;

                    EndSubpackets::Count(subpacket_count)
                };

                let mut subpackets = 1;
                let mut subpacket_values = if SUM_VERSIONS {
                    version
                } else {
                    // Parse the first packet so we can initialize the subpacket values properly.
                    let (value, subpacket_bits) = parse::<SUM_VERSIONS>(input, bits);
                    bits = subpacket_bits;

                    value
                };

                while match subpacket_condition {
                    EndSubpackets::Bits(x) => bits < x,
                    EndSubpackets::Count(x) => subpackets < x,
                } {
                    let (value, subpacket_bits) = parse::<SUM_VERSIONS>(input, bits);
                    bits = subpacket_bits;
                    subpackets += 1;

                    if SUM_VERSIONS {
                        subpacket_values += value;
                    } else {
                        match type_id {
                            0 => subpacket_values += value,
                            1 => subpacket_values *= value,
                            2 => subpacket_values = std::cmp::min(subpacket_values, value),
                            3 => subpacket_values = std::cmp::max(subpacket_values, value),
                            5 => subpacket_values = if value < subpacket_values { 1 } else { 0 },
                            6 => subpacket_values = if value > subpacket_values { 1 } else { 0 },
                            7 => subpacket_values = if value == subpacket_values { 1 } else { 0 },
                            _ => unreachable!(),
                        }
                    }
                }

                subpacket_values
            }
        };

        (value, bits)
    }

    /// Directly calls [`parse`].
    pub fn part1(input: &[u8]) -> usize {
        parse::<true>(input, 0).0
    }

    /// Directly calls [`parse`].
    pub fn part2(input: &[u8]) -> usize {
        parse::<false>(input, 0).0
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "0 = 0000
1 = 0001
2 = 0010
3 = 0011
4 = 0100
5 = 0101
6 = 0110
7 = 0111
8 = 1000
9 = 1001
A = 1010
B = 1011
C = 1100
D = 1101
E = 1110
F = 1111";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 31);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            //assert_eq!(part2(&input), 5);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2021/day16.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 925);
            assert_eq!(part2(&input), 342997120375);
        }
    }
}

/// Leans on triangle numbers to reduce the need to brute-force.
pub mod day17 {
    /// Strip the garbage off the front, then split into two ranges and parse them.
    pub fn generator(input: &str) -> ((isize, isize), (isize, isize)) {
        let mut ranges = input["target area: ".len()..]
            .trim_end()
            .split(", ")
            .map(|range| {
                range[2..]
                    .split_once("..")
                    .map(|(l, r)| {
                        (
                            l.parse().expect("Ranges are composed of integers"),
                            r.parse().expect("Ranges are composed of integers"),
                        )
                    })
                    .expect("Ranges must be separated")
            });

        (
            ranges.next().expect("Must be a x range"),
            ranges.next().expect("Must be a y range"),
        )
    }

    /// Probe trajectory is a parabola, so it'll always pass back through 0 with `-initial`
    /// velocity.  Assume we're hitting the lowest part of our target in a single step from 0,
    /// then figure out how high it must've been.
    /// The trajectory will be `y + (y-1) + ... = \sum_1^y y`, or `y * (y+1) / 2`.
    pub fn part1(&(_, (by, _)): &((isize, isize), (isize, isize))) -> isize {
        (by * (by + 1)) / 2
    }

    /// Brute force the smallest set of possibilities we reasonably can.
    ///
    /// We know the minimum x-velocity that can reach the target to start with is when the
    /// projectile reaches the target with zero remaining x-velocity, so triangle numbers apply
    /// again.  The largest x-velocity is the right side of the target, since any faster would miss
    /// it in one step.
    ///
    /// `by` is the lowest y-velocity that can hit our target in one move, and we know `by.abs()` is the
    /// largest y-velocity.  We handle these halves in two cases:
    /// 1. Any negative velocities will drop fairly rapidly so wont't go through many
    ///    steps.
    /// 2. Positive velocities can go through substantially more steps, but must all go back
    ///    through zero, so they'll require at least `2y+1` steps. We can start the sampling at
    ///    that point, saving lots of effort.
    pub fn part2(&((lx, rx), (by, ty)): &((isize, isize), (isize, isize))) -> usize {
        assert!(ty < 0, "Assume targets are lower than us");

        let min_x = (1..)
            .find(|candidate_x| candidate_x * (candidate_x + 1) >= lx * 2)
            .expect("Must be a minimum x-velocity");

        (min_x..=rx)
            .flat_map(|xv| {
                (by..=0)
                    .filter(move |&yv| {
                        let (mut xv, mut yv) = (xv, yv);
                        let (mut x, mut y) = (xv, yv);

                        loop {
                            if y < by || x > rx {
                                break false;
                            } else if y <= ty && x >= lx {
                                // We made it.
                                break true;
                            }

                            xv = std::cmp::max(0, xv - 1);
                            yv -= 1;
                            x += xv;
                            y += yv;
                        }
                    })
                    .chain((1..by.abs()).filter(move |&yv| {
                        // We know our minimum steps are 2y+1, since that'll bring our
                        // y back to 0.
                        let min_steps = 2 * yv + 1;

                        let (mut y, mut yv) = (0, -yv);
                        let (mut x, mut xv) = if xv > min_steps {
                            // We're still stepping along
                            (
                                xv * min_steps - (min_steps * (min_steps - 1) / 2),
                                xv - (min_steps - 1),
                            )
                        } else {
                            // We'll be done stepping at this point.
                            (xv * (xv + 1) / 2, 0)
                        };

                        loop {
                            if y < by || x > rx {
                                break false;
                            } else if y <= ty && x >= lx {
                                // We made it.
                                break true;
                            }

                            xv = std::cmp::max(0, xv - 1);
                            yv -= 1;
                            x += xv;
                            y += yv;
                        }
                    }))
            })
            .count()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "target area: x=20..30, y=-10..-5";

        #[test]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 45);
        }

        #[test]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 112);
        }

        #[test]
        fn test_answers() {
            let input_file = "../data/2021/day17.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 7626);
            assert_eq!(part2(&input), 2032);
        }
    }
}

/// Represent snailfish numbers as a list of (value, depth), simplifying [day18::reduce_number] at
/// the cost of complexity in [day18::magnitude].
pub mod day18 {
    use itertools::Itertools;

    /// Depths can't be very large.
    pub type Depth = u8;

    /// Represent numbers as a value and the depth its at, making it easier to implement `explode`
    /// and `split`.
    ///
    /// We know this vector will have a max length of `2^MAX_DEPTH`, but it's more convenient to
    /// use an arbitrary length one to support addition.
    pub type Number = Vec<(u8, Depth)>;

    /// If anything nests more than this, we'll have to `explode` it.
    pub const MAX_DEPTH: Depth = 4;

    /// Trivial parsing by counting paren depth.
    pub fn generator(input: &str) -> Vec<Number> {
        input
            .lines()
            .map(|line| {
                let mut depth = 0;
                let mut number = Vec::with_capacity(line.len() / 2);

                for c in line.bytes() {
                    match c {
                        b'[' => depth += 1,
                        b']' => depth -= 1,
                        b',' => (),
                        num => number.push((num - b'0', depth)),
                    }
                }

                number
            })
            .collect()
    }

    /// Destructively reduce the given number.
    ///
    /// TODO: This isn't at all efficient, but this problem is fast enough that I haven't bothered
    /// optimizing.  My guess is rearranging elements of the vector is the bottleneck.
    pub fn reduce_number(number: &mut Number) {
        loop {
            if let Some(explode_start) = number.iter().position(|&(_, depth)| depth > MAX_DEPTH) {
                // There's a pair to explode.
                let (left, right) = (number[explode_start], number[explode_start + 1]);

                // Replace this tuple with a 0 at one lower depth.
                number[explode_start] = (0, left.1 - 1);
                number.remove(explode_start + 1);

                // Distribute this tuple to the left and right if present.
                if let Some(target) = number.get_mut(explode_start - 1) {
                    target.0 += left.0;
                }
                if let Some(target) = number.get_mut(explode_start + 1) {
                    target.0 += right.0;
                }
            } else if let Some(too_big) = number.iter().position(|&(num, _)| num > 9) {
                // There's a number that needs splitting.
                let (split, depth) = number[too_big];

                number[too_big] = (split / 2, depth + 1);
                number.insert(too_big + 1, ((split + 1) / 2, depth + 1));
            } else {
                // Neither `explode` nor `split` hit this time, so we must be done reducing.
                break;
            }
        }
    }

    /// Compute the magnitude by recursively computing the magnitude for each depth.
    ///
    /// For this problem, this is a bit overkill, since the highest magnitude numbers will always
    /// be full (`2^MAX_DEPTH` elements), meaning a recursive approach splitting the number in half
    /// each time would work equivalently well.  That felt a bit too hacky to me though.
    pub fn magnitude(number: &Number) -> usize {
        fn depth_mag(number: &Number, i: &mut usize, depth: u8) -> usize {
            // For a given depth, consume the left side first.
            let left = if number[*i].1 == depth {
                // We're at the right depth, so consume this element
                *i += 1;
                number[*i - 1].0 as usize
            } else {
                // Didn't find the expected depth at this character, so let's go deeper.
                depth_mag(number, i, depth + 1)
            };

            let right = if number[*i].1 == depth {
                // We're at the right depth, so consume this element
                *i += 1;
                number[*i - 1].0 as usize
            } else {
                // Didn't find the expected depth at this character, so let's go deeper.
                depth_mag(number, i, depth + 1)
            };

            3 * left + 2 * right
        }

        depth_mag(number, &mut 0, 1)
    }

    /// The obvious implementation, leaning on [reduce_number] and [magnitude].
    pub fn part1(numbers: &[Number]) -> usize {
        let mut cummulative = numbers[0].clone();
        reduce_number(&mut cummulative);

        for number in numbers.iter().skip(1) {
            cummulative.extend_from_slice(number);
            cummulative.iter_mut().for_each(|c| c.1 += 1);

            reduce_number(&mut cummulative);
        }

        magnitude(&cummulative)
    }

    /// Naively add and reduce each pair, taking the maximum resulting magnitude.
    ///
    /// We cheat by only looking at the largest numbers, since a longer number will score better
    /// than a smaller one.
    pub fn part2(numbers: &[Number]) -> usize {
        // Only look at a few of the largest numbers, since short numbers are
        // unlikely to have a large magnitude.
        const BIG_COUNT: usize = 10;

        let big_numbers: Vec<&Number> = numbers
            .iter()
            .sorted_by_key(|number| number.len())
            .rev()
            .take(BIG_COUNT)
            .collect();

        big_numbers
            .iter()
            .enumerate()
            .cartesian_product(big_numbers.iter().enumerate())
            .filter(|((x, _), (y, _))| x != y)
            .map(|((_, x), (_, y))| {
                let mut total = Vec::new();

                total.extend_from_slice(x);
                total.extend_from_slice(y);
                total.iter_mut().for_each(|c| c.1 += 1);

                reduce_number(&mut total);

                magnitude(&total)
            })
            .max()
            .expect("Must be a largest sum")
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "[1,2]
[[1,2],3]
[9,[8,7]]
[[1,9],[8,5]]
[[[[1,2],[3,4]],[[5,6],[7,8]]],9]
[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 4140);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 3993);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2021/day18.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 3359);
            assert_eq!(part2(&input), 4616);
        }
    }
}

/// Horribly annoying matrix code with a problem that seems a lot harder than it actually is.
///
/// The real work is done by [`day19::canonicalize_scanners`].
pub mod day19 {
    use arrayvec::ArrayVec;
    use fnv::{FnvHashMap, FnvHashSet};
    use itertools::Itertools;

    /// We're doing lots of math on this type, so we want a native size.
    pub type Dist = i32;

    /// A coordinate has three scalar distance components
    pub type Coord = (Dist, Dist, Dist);

    /// Scanners have beacons, and also track the distance (squared) between each pair of beacons.
    pub struct Scanner {
        beacons: Vec<Coord>,
        pairwise_distances: FnvHashMap<Dist, (Coord, Coord)>,
    }

    /// Parse scanners and beacons naively, but also generate pairwise distances between each
    /// beacon visible to the scanner, as this will be useful to reference throughout the problem.
    pub fn generator(input: &str) -> Vec<Scanner> {
        input
            .split("\n\n")
            .map(|scanner| {
                let beacons = scanner
                    .lines()
                    .skip(1)
                    .map(|line| {
                        line.split(',')
                            .map(|coord| coord.parse().expect("Must be integer coordinate"))
                            .next_tuple()
                            .expect("All positions have 3 dimensions")
                    })
                    .collect::<Vec<Coord>>();

                Scanner {
                    pairwise_distances: beacons
                        .iter()
                        .tuple_combinations()
                        .map(|(&a, &b)| {
                            let x = a.0 - b.0;
                            let y = a.1 - b.1;
                            let z = a.2 - b.2;

                            (x * x + y * y + z * z, (a, b))
                        })
                        .collect::<FnvHashMap<Dist, _>>(),
                    beacons: beacons,
                }
            })
            .collect()
    }

    /// Represents simple 3D rotation matrices
    pub type RotationMatrix = ((Dist, Dist, Dist), (Dist, Dist, Dist), (Dist, Dist, Dist));

    /// The set of 90-degree euclidean rotation matrices, which will be used to determine how
    /// our different coordinate systems compare to the canonical one.
    pub const ROTATIONS: &[RotationMatrix] = &[
        ((1, 0, 0), (0, 1, 0), (0, 0, 1)),
        ((1, 0, 0), (0, 0, -1), (0, 1, 0)),
        ((1, 0, 0), (0, -1, 0), (0, 0, -1)),
        ((1, 0, 0), (0, 0, 1), (0, -1, 0)),
        //
        ((0, -1, 0), (1, 0, 0), (0, 0, 1)),
        ((0, 0, 1), (1, 0, 0), (0, 1, 0)),
        ((0, 1, 0), (1, 0, 0), (0, 0, -1)),
        ((0, 0, -1), (1, 0, 0), (0, -1, 0)),
        //
        ((-1, 0, 0), (0, -1, 0), (0, 0, 1)),
        ((-1, 0, 0), (0, 0, -1), (0, -1, 0)),
        ((-1, 0, 0), (0, 1, 0), (0, 0, -1)),
        ((-1, 0, 0), (0, 0, 1), (0, 1, 0)),
        //
        ((0, 1, 0), (-1, 0, 0), (0, 0, 1)),
        ((0, 0, 1), (-1, 0, 0), (0, -1, 0)),
        ((0, -1, 0), (-1, 0, 0), (0, 0, -1)),
        ((0, 0, -1), (-1, 0, 0), (0, 1, 0)),
        //
        ((0, 0, -1), (0, 1, 0), (1, 0, 0)),
        ((0, 1, 0), (0, 0, 1), (1, 0, 0)),
        ((0, 0, 1), (0, -1, 0), (1, 0, 0)),
        ((0, -1, 0), (0, 0, -1), (1, 0, 0)),
        //
        ((0, 0, -1), (0, -1, 0), (-1, 0, 0)),
        ((0, -1, 0), (0, 0, 1), (-1, 0, 0)),
        ((0, 0, 1), (0, 1, 0), (-1, 0, 0)),
        ((0, 1, 0), (0, 0, -1), (-1, 0, 0)),
    ];

    /// Rotate (multiply) the coordinate via the given rotation matrix.
    pub fn rotate((x, y, z): Coord, (r1, r2, r3): &RotationMatrix) -> Coord {
        (
            r1.0 * x + r2.0 * y + r3.0 * z,
            r1.1 * x + r2.1 * y + r3.1 * z,
            r1.2 * x + r2.2 * y + r3.2 * z,
        )
    }

    /// Map each scanner onto a canonical coordinate system for further analysis.
    ///
    /// We first determine which scanners overlap based on whether they can see 12 of the same
    /// beacons, which can be done via the reasonable heuristic of having enough pairwise distances
    /// that overlap (in this case 12 choose 2).
    ///
    /// With the overlapping set of scanners, we do a graph traversal trying to canonicalize the
    /// beacons for each scanner.  Starting from the first scanner, which we treat as canonical, we
    /// know there are two beacons from a `canonical scanner` and `non-canonical scanner` that must
    /// be the same in the canonical coordinate system.  To determine which beacons these are, we
    /// rotate the pairs of non-canonical beacons that overlap until the distance between each pair
    /// matches, giving us a canonicalizing rotation and allowing us to determine the offset of the
    /// beacons from the canonical coordinate system.
    ///
    /// Applying this rotation and offset to all beacons in the scanner grows our canonical set and
    /// allows us to continue traversing the graph of scanners until they've all been
    /// canonicalized.
    pub fn canonicalize_scanners(
        scanners: &[Scanner],
    ) -> (Vec<FnvHashMap<Coord, Coord>>, Vec<Coord>) {
        let mut visible = FnvHashMap::default();

        for (a, scanner_a) in scanners.iter().enumerate() {
            for (b, scanner_b) in scanners.iter().enumerate().skip(a + 1) {
                let overlaps = scanner_a
                    .pairwise_distances
                    .keys()
                    .filter(|dist| scanner_b.pairwise_distances.contains_key(dist))
                    .count();

                if overlaps >= 66 {
                    (*visible.entry(a).or_insert(ArrayVec::<_, 5>::new())).push(b);
                    (*visible.entry(b).or_insert(ArrayVec::<_, 5>::new())).push(a);
                }
            }
        }

        // Treat scanner 0 as the base for our coordinate system.
        let mut feasible = visible
            .remove(&0)
            .expect("Must be visible nodes from 0")
            .into_iter()
            .map(|x| (0, x))
            .collect::<Vec<_>>();

        // For each scanner, map the canonical resolutions for their beacons.
        let mut canonicalization = vec![FnvHashMap::<Coord, Coord>::default(); scanners.len()];
        canonicalization[0].extend(scanners[0].beacons.iter().map(|x| (x, x)));

        let mut scanner_locations = vec![(0, 0, 0); scanners.len()];

        // Walk through each scanner that can see other scanners.
        // We assume this is a connected graph, otherwise it's not possible to put together a
        // cohesive picture.
        while let Some((parent, neighbor)) = feasible.pop() {
            if !canonicalization[neighbor].is_empty() {
                // We've already oriented this beacon.
                continue;
            }

            let (rotation, offset) = scanners[parent]
                .pairwise_distances
                .iter()
                .find_map(|(dist, (parent_a, parent_b))| {
                    scanners[neighbor].pairwise_distances.get(dist).map(
                        |&(neighbor_a, neighbor_b)| {
                            // Canonicalize the parents.
                            let parent_a = canonicalization[parent]
                                .get(parent_a)
                                .expect("Can't have a non-canonicalized parent");
                            let parent_b = canonicalization[parent]
                                .get(parent_b)
                                .expect("Can't have a non-canonicalized parent");

                            // We know parent_a, parent_b are two points in the canonical grid, and
                            // neighbor_a, neighbor_b are the same two points from a different
                            // perspective, so if we can rotate them to have equal offsets we know how
                            // our grid is shifted.
                            ROTATIONS
                                .iter()
                                .find_map(|rot| {
                                    let rot_a = rotate(neighbor_a, rot);
                                    let rot_b = rotate(neighbor_b, rot);

                                    if (rot_a.0 - parent_a.0 == rot_b.0 - parent_b.0)
                                        && (rot_a.1 - parent_a.1 == rot_b.1 - parent_b.1)
                                        && (rot_a.2 - parent_a.2 == rot_b.2 - parent_b.2)
                                    {
                                        Some((
                                            rot,
                                            (
                                                rot_a.0 - parent_a.0,
                                                rot_a.1 - parent_a.1,
                                                rot_a.2 - parent_a.2,
                                            ),
                                        ))
                                    } else if (rot_b.0 - parent_a.0 == rot_a.0 - parent_b.0)
                                        && (rot_b.1 - parent_a.1 == rot_a.1 - parent_b.1)
                                        && (rot_b.2 - parent_a.2 == rot_a.2 - parent_b.2)
                                    {
                                        Some((
                                            rot,
                                            (
                                                rot_b.0 - parent_a.0,
                                                rot_b.1 - parent_a.1,
                                                rot_b.2 - parent_a.2,
                                            ),
                                        ))
                                    } else {
                                        None
                                    }
                                })
                                .expect("Some rotation must work")
                        },
                    )
                })
                .expect("Must be a valid pairing between neighboring scanners");

            // Canonicalize all of our beacons.
            canonicalization[neighbor].extend(scanners[neighbor].beacons.iter().map(|beacon| {
                let rot = rotate(*beacon, &rotation);

                (
                    *beacon,
                    (rot.0 - offset.0, rot.1 - offset.1, rot.2 - offset.2),
                )
            }));

            // Track the canonical location for our scanner.
            scanner_locations[neighbor] = (-offset.0, -offset.1, -offset.2);

            // Now that we've oriented ourselves, we can orient our neighbors.
            visible
                .remove(&neighbor)
                .map(|neighbors| feasible.extend(neighbors.iter().map(|&n| (neighbor, n))));
        }

        (canonicalization, scanner_locations)
    }

    /// Canonicalize the scanners via [`canonicalize_scanners`], then count all of the unique
    /// beacons.
    pub fn part1(scanners: &[Scanner]) -> usize {
        let (canonicalization, _) = canonicalize_scanners(scanners);

        canonicalization
            .iter()
            .fold(FnvHashSet::<Coord>::default(), |mut total, scanner| {
                total.extend(scanner.values());
                total
            })
            .len()
    }

    /// Canonicalize the scanners via [`canonicalize_scanners`], then compare all combinations of
    /// scanners to determine the most separated pairs.
    pub fn part2(scanners: &[Scanner]) -> usize {
        let (_, scanner_locations) = canonicalize_scanners(scanners);

        scanner_locations
            .iter()
            .tuple_combinations()
            .map(|((x1, y1, z1), (x2, y2, z2))| {
                ((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()) as usize
            })
            .max()
            .expect("Must be multiple scanners")
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

        #[test]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 79);
        }

        #[test]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 3621);
        }

        #[test]
        fn test_answers() {
            let input_file = "../data/2021/day19.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 350);
            assert_eq!(part2(&input), 10895);
        }
    }
}

/// Conway's Game of Life, with a minor twist.
///
/// All data structures are naive, and [`day20::enhance`] is really the interesting piece of this
/// day.
pub mod day20 {
    use arrayvec::ArrayVec;

    /// Store our algorithm as an array of booleans to avoid various bit manipulation, since it's
    /// small enough that it won't add much cache pressure.
    type Algorithm = [bool; ALGO_BITS];

    /// Represent the image as a 2D array of booleans.
    pub type Image = Vec<[bool; START_DIM]>;

    /// Width and heigh of the image
    pub const START_DIM: usize = 100;

    /// Number of bits for the algorithm
    pub const ALGO_BITS: usize = 512;

    /// Fairly obvious parsing implementation.
    pub fn generator(input: &str) -> (Algorithm, Image) {
        let (algo, image) = input
            .split_once("\n\n")
            .expect("Must be two parts to the input");

        let algorithm = algo
            .trim_end()
            .bytes()
            .map(|b| b == b'#')
            .collect::<ArrayVec<_, ALGO_BITS>>()
            .as_slice()
            .try_into()
            .expect("Algorithm is always ALGO_BITS long");

        let image = image
            .lines()
            .map(|line| {
                line.trim_end()
                    .bytes()
                    .map(|b| b == b'#')
                    .collect::<ArrayVec<_, START_DIM>>()
                    .as_slice()
                    .try_into()
                    .expect("Canvas is always START_DIM wide.")
            })
            .collect();

        (algorithm, image)
    }

    /// Enhance the image over the given number of iterations by implementing a slightly modified
    /// Conway's Game of Life, where the abnormal thing we need to think about is whether our
    /// infinite background will flip each step.
    ///
    /// We do two interesting optimizations:
    /// 1. We generate a fixed size canvas that can fit all the iterations on it, to avoid needing
    ///    to do dynamic allocation.  To be (potentially) friendlier to the cache, we store
    ///    everything starting at `canvas[0][0]`, shifting the image every iteration rather than
    ///    changing our view.
    /// 2. We maintain a running index into the algorithm table for each row, allowing us to only
    ///    look at three elements of the image per pixel rather than all nine, and yielding an
    ///    substantial (~90%) speedup.
    pub fn enhance<const ITERATIONS: usize>(algorithm: &Algorithm, image: &Image) -> usize {
        const MAX_ITERATIONS: usize = 50;
        // TODO: We want to use ITERATIONS in this expression, but it's not currently supported by
        // stable Rust.
        let mut current = [[false; START_DIM + 2 * MAX_ITERATIONS]; START_DIM + 2 * MAX_ITERATIONS];

        for (y, row) in image.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                current[y][x] = value;
            }
        }

        // If `algo[0] && !algo[511]`, we'll constantly flip our infinite space from 0 to 1.
        // Let's keep track of what the empty space is, rather than trying to deal with that.
        let needs_toggle = algorithm[0] && !algorithm[ALGO_BITS - 1];

        for round in 1..=ITERATIONS {
            let mut new = [[false; START_DIM + 2 * MAX_ITERATIONS]; START_DIM + 2 * MAX_ITERATIONS];
            let toggle = (needs_toggle && round % 2 == 0) as usize;
            let prev_dim = START_DIM + 2 * (round - 1);

            for y in 0..START_DIM + 2 * round {
                // Compute a running index into `algorithm` for each neighboring row.  We'll modify
                // this for each column rather than recomputing it.
                // The first two elements of each row are out-of-bounds by definition and since we're
                // really starting a bit off the screen.
                let mut index = if toggle != 0 { 0b011011011 } else { 0 };

                // We're working with the coordinates from our current round, but the previous
                // round was both offset and slightly smaller.  Since we only expand by two
                // spaces per round, we know `y-2` won't be higher than the previous maximum.
                //
                // NOTE: Manually hoisted to help the optimizer out, since LLVM wasn't happy with
                // it inline.
                let neighbor_rows = [
                    current.get(y - 2),
                    (y - 1 < prev_dim).then(|| &current[y - 1]),
                    (y < prev_dim).then(|| &current[y]),
                ];

                for x in 0..START_DIM + 2 * round {
                    // Shift the algorithm index one column to the right.
                    let new_column = neighbor_rows.iter().fold(0, |acc, row| {
                        row.map_or(toggle, |row| {
                            if x < prev_dim {
                                row[x] as usize
                            } else {
                                toggle
                            }
                        }) | acc << 3
                    });
                    index = (index << 1) & 0b110110110 | new_column;

                    new[y][x] = algorithm[index];
                }
            }

            current = new;
        }

        current
            .into_iter()
            .map(|row| row.into_iter().filter(|&v| v).count())
            .sum()
    }

    /// [enhance] the image by two steps.
    pub fn part1((algo, image): &(Algorithm, Image)) -> usize {
        enhance::<2>(algo, image)
    }

    /// [enhance] the image by fifty steps.
    pub fn part2((algo, image): &(Algorithm, Image)) -> usize {
        enhance::<50>(algo, image)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str =
            "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 35);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 3351);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2021/day20.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 5489);
            assert_eq!(part2(&input), 19066);
        }
    }
}

/// A memoized recursive solution using a 4-d lookup table.
pub mod day21 {
    use arrayvec::ArrayVec;

    /// Represents which space is occupied
    pub type Space = usize;
    /// It's a two player game
    pub const PLAYERS: usize = 2;
    /// There are 10 spaces on the board
    pub const SPACES: usize = 10;

    /// There are less than 10 spaces, so we only need to look at the last byte on each line.
    pub fn generator(input: &str) -> [Space; PLAYERS] {
        input
            .lines()
            .map(|line| {
                let bytes = line.as_bytes();

                (bytes[bytes.len() - 1] - b'1') as Space
            })
            .collect::<ArrayVec<Space, PLAYERS>>()
            .as_slice()
            .try_into()
            .expect("Unexpected number of players")
    }

    /// Brute force solution with no optimization or cleverness.
    pub fn part1(starts: &[Space; PLAYERS]) -> usize {
        const TARGET: usize = 1000;
        const DIE_SIDES: usize = 100;

        let mut scores = [0; PLAYERS];
        let mut spaces = *starts;
        let mut turn = 0;

        for i in (1..).step_by(3) {
            spaces[turn] = (spaces[turn]
                + [i, i + 1, i + 2]
                    .iter()
                    .map(|x| x % DIE_SIDES)
                    .sum::<usize>())
                % SPACES;
            scores[turn] += spaces[turn] + 1;

            if scores[turn] >= TARGET {
                return (i + 2) * scores[(turn + 1) % 2];
            }

            turn = (turn + 1) % scores.len();
        }

        unreachable!()
    }

    /// Use memoization to compute all of the different states we can be be in,
    /// where the relevant state information is (score of each player, space of each player).
    /// From a given space, we know the number of ways we can move to other spaces, and can
    /// recursively count all of the games arising from those options.
    ///
    /// We're limited in performance by random accesses to the table, so it seems plausible that
    /// having a more cache efficient structure for set membership would pay off.
    pub fn part2(starts: &[Space; PLAYERS]) -> usize {
        const TARGET: usize = 21;

        // We're rolling three 3-sided dice.
        const ROLL_OPTIONS: [(usize, usize); 7] =
            [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

        let mut table = [[[[(0, 0); SPACES]; SPACES]; TARGET]; TARGET];

        // I gave up on supporting more than 2 players.
        fn play(
            current_space: usize,
            current_score: usize,
            other_space: usize,
            other_score: usize,
            table: &mut [[[[(usize, usize); SPACES]; SPACES]; TARGET]; TARGET],
        ) -> (usize, usize) {
            let prev = table[current_score][other_score][current_space][other_space];

            if prev != (0, 0) {
                // We've compute this before.
                return prev;
            }

            let mut result = (0, 0);

            for (roll_total, rolls) in ROLL_OPTIONS {
                let new_space = (current_space + roll_total) % SPACES;
                let new_score = current_score + new_space + 1;

                result = if new_score >= TARGET {
                    // We know what this'll be, so no need to do extra math.
                    (result.0 + rolls, result.1)
                } else {
                    let (p2_wins, p1_wins) =
                        play(other_space, other_score, new_space, new_score, table);

                    (result.0 + rolls * p1_wins, result.1 + rolls * p2_wins)
                };
            }

            table[current_score][other_score][current_space][other_space] = result;

            result
        }

        let (p1_wins, p2_wins) = play(starts[0], 0, starts[1], 0, &mut table);

        std::cmp::max(p1_wins, p2_wins)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "Player 1 starting position: 4
Player 2 starting position: 8";

        #[test]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 739785);
        }

        #[test]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 444356092776315);
        }

        #[test]
        fn test_answers() {
            let input_file = "../data/2021/day21.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 506466);
            assert_eq!(part2(&input), 632979211251440);
        }
    }
}

/// Track the set of boxes that are enabled, splitting them as necessary when they overlap.
pub mod day22 {
    use arrayvec::ArrayVec;

    /// We (surprisingly) see a ~10% performance boost when using `isize` rather than `i32`, which
    /// would be more memory efficient.
    pub type Coord = isize;

    /// Represents a box, with each coordinate being a `(start, end)` pair.
    #[derive(Copy, Clone)]
    pub struct Range<Coord> {
        x: (Coord, Coord),
        y: (Coord, Coord),
        z: (Coord, Coord),
    }
    /// The naive representation of the input commands.
    #[derive(Copy, Clone)]
    pub struct Command {
        enable: bool,
        range: Range<Coord>,
    }

    /// Given a pair of overlapping ranges, we can split into at most 7 smaller ranges, one of
    /// which is the overlap.
    pub const MAX_SPLITS: usize = 6;

    impl Range<Coord> {
        /// Returns the region that overlaps between the two ranges, if any.
        pub fn overlap(&self, other: Self) -> Option<Self> {
            ((self.x.0 <= other.x.1 && self.x.1 >= other.x.0)
                && (self.y.0 <= other.y.1 && self.y.1 >= other.y.0)
                && (self.z.0 <= other.z.1 && self.z.1 >= other.z.0))
                .then(|| Self {
                    x: (
                        std::cmp::max(self.x.0, other.x.0),
                        std::cmp::min(self.x.1, other.x.1),
                    ),
                    y: (
                        std::cmp::max(self.y.0, other.y.0),
                        std::cmp::min(self.y.1, other.y.1),
                    ),
                    z: (
                        std::cmp::max(self.z.0, other.z.0),
                        std::cmp::min(self.z.1, other.z.1),
                    ),
                })
        }

        /// Return the set of ranges left after removing `overlap`.
        pub fn remove(&self, overlap: Self) -> ArrayVec<Self, MAX_SPLITS> {
            let mut splits = ArrayVec::default();

            // Handle all the x overlaps
            if overlap.x.0 > self.x.0 {
                splits.push(Self {
                    x: (self.x.0, overlap.x.0 - 1),
                    y: (self.y.0, self.y.1),
                    z: (self.z.0, self.z.1),
                })
            }
            if overlap.x.1 < self.x.1 {
                splits.push(Self {
                    x: (overlap.x.1 + 1, self.x.1),
                    y: (self.y.0, self.y.1),
                    z: (self.z.0, self.z.1),
                })
            }
            // Handle all the Y overlaps
            if overlap.y.1 < self.y.1 {
                splits.push(Self {
                    x: (overlap.x.0, overlap.x.1),
                    y: (overlap.y.1 + 1, self.y.1),
                    z: (self.z.0, self.z.1),
                })
            }
            if overlap.y.0 > self.y.0 {
                splits.push(Self {
                    x: (overlap.x.0, overlap.x.1),
                    y: (self.y.0, overlap.y.0 - 1),
                    z: (self.z.0, self.z.1),
                })
            }
            // Handle all the z overlaps
            if overlap.z.0 > self.z.0 {
                splits.push(Self {
                    x: (overlap.x.0, overlap.x.1),
                    y: (overlap.y.0, overlap.y.1),
                    z: (self.z.0, overlap.z.0 - 1),
                })
            }
            if overlap.z.1 < self.z.1 {
                splits.push(Self {
                    x: (overlap.x.0, overlap.x.1),
                    y: (overlap.y.0, overlap.y.1),
                    z: (overlap.z.1 + 1, self.z.1),
                })
            }

            splits
        }

        pub fn count(&self) -> usize {
            (self.x.1 - self.x.0 + 1) as usize
                * (self.y.1 - self.y.0 + 1) as usize
                * (self.z.1 - self.z.0 + 1) as usize
        }
    }

    /// Naive parsing by splitting repeatedly.
    pub fn generator(input: &str) -> Vec<Command> {
        input
            .lines()
            .map(|line| {
                let (cmd, coords) = line.split_once(' ').expect("Must be a command and ranges");

                let mut coords = coords.split(',').map(|range| {
                    let (l, r) = range[2..].split_once("..").expect("Ranges have two parts");

                    (
                        l.parse().expect("Must be int"),
                        r.parse().expect("Must be int"),
                    )
                });

                Command {
                    enable: cmd.as_bytes()[1] == b'n',
                    range: Range {
                        x: coords.next().expect("Must be an x coord"),
                        y: coords.next().expect("Must be an y coord"),
                        z: coords.next().expect("Must be an z coord"),
                    },
                }
            })
            .collect()
    }

    /// Track all enabled regions of the grid, with the invariant that no region is tracked twice.
    /// Whenever a command overlaps with an existing enabled region, we disable the overlapping
    /// section, then add the command if it's an `on`.
    ///
    /// This doesn't use anything fancy to determine which regions overlap, as the n^2 variant is
    /// fast enough.
    pub fn counter(commands: &[Command]) -> usize {
        let mut lit = Vec::with_capacity(commands.len() / 2);

        for cmd in commands.iter() {
            let mut overlap_ids = Vec::new();
            let mut new_ranges = Vec::new();

            // Find any currently lit regions that overlap with us and remove the overlap from the
            // list.
            for (i, &currently_lit) in lit.iter().enumerate() {
                if let Some(overlap) = cmd.range.overlap(currently_lit) {
                    new_ranges.extend(currently_lit.remove(overlap));
                    overlap_ids.push(i);
                }
            }

            // Remove from the end first to avoid invalidating our indices.
            for id in overlap_ids.into_iter().rev() {
                lit.swap_remove(id);
            }
            lit.extend(new_ranges);

            if cmd.enable {
                lit.push(cmd.range);
            }
        }

        lit.into_iter().map(|range| range.count()).sum::<usize>()
    }

    /// Filter to only the initialization commands, then let [`counter`] do the real work.
    pub fn part1(commands: &[Command]) -> usize {
        let first_commands = commands
            .iter()
            .filter(|cmd| {
                let r = cmd.range;

                r.x.0 >= -50
                    && r.x.1 <= 50
                    && r.y.0 >= -50
                    && r.y.1 <= 50
                    && r.z.0 >= -50
                    && r.z.1 <= 50
            })
            .copied()
            .collect::<Vec<_>>();

        counter(&first_commands)
    }

    /// Let [`counter`] do the real work.
    pub fn part2(commands: &[Command]) -> usize {
        counter(commands)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 590784);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 2758514936282235);
        }

        #[test]
        fn test_answers() {
            let input_file = "../data/2021/day22.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 615869);
            assert_eq!(part2(&input), 1323862415207825);
        }
    }
}

/// Fairly tedious code to generate the different state transitions that are possible and then
/// search between those for an optimal solution.
///
/// The main interesting optimization here lies in the representation of [`day23::State`], which
/// requires only 16 bytes to represent the board.
pub mod day23 {
    use bit::BitIndex;
    use fnv::FnvHashMap;

    /// A type to represent the cost of some [`Amphipod`]s moving.
    pub type Cost = usize;

    ///////////////////////////////////////////////////////////////////////////
    // Amphipods
    ///////////////////////////////////////////////////////////////////////////

    /// Naive amphipod representation.
    #[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
    pub enum Amphipod {
        A,
        B,
        C,
        D,
    }

    impl Amphipod {
        /// The movement cost per step for the given amphipod.
        pub const fn cost(&self) -> Cost {
            match self {
                Amphipod::A => 1,
                Amphipod::B => 10,
                Amphipod::C => 100,
                Amphipod::D => 1000,
            }
        }

        /// Generate the corresponding Amphipod from an index.  This is useful for converting from
        /// integers.
        pub const fn from_index(index: usize) -> Self {
            match index {
                0 => Amphipod::A,
                1 => Amphipod::B,
                2 => Amphipod::C,
                3 => Amphipod::D,
                _ => unreachable!(),
            }
        }

        /// Return the target room for this amphipod (which also happens to be its index).
        pub const fn target(&self) -> usize {
            match self {
                Amphipod::A => 0,
                Amphipod::B => 1,
                Amphipod::C => 2,
                Amphipod::D => 3,
            }
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    // Rooms
    ///////////////////////////////////////////////////////////////////////////

    /// Rooms contain up to four different Amphipods, each of which can represent four different
    /// values, and we need to be able to arbitrarily examine and rearrange rooms.
    ///
    /// Of particular note is that we only need to remove amphipods from rooms during the game
    /// simulation, as there's no need to store amphipods that have reached their final location.
    ///
    /// To save space and optimize our memory allocations/lookup tables, we implement a tiny vector
    /// to store Amphipods, letting us represent all four rooms in 8 bytes.  Various bit
    /// manipulation is needed, and we only implement a few of the standard methods that we need.
    #[derive(Copy, Clone, Hash, PartialEq, Eq)]
    pub struct Room(pub u16);

    /// We want to be able to iterate over rooms to avoid significant refactoring of our code that
    /// initially represented [`Room`]s as [`arrayvec::ArrayVec`]s.
    ///
    /// This implements the naive implementation of iteration over [`Room`]s.
    pub struct RoomIterator<'a> {
        /// The room we're iterating over.
        pub room: &'a Room,

        /// The next index to yield from the iterator.
        pub i: usize,
    }

    impl Room {
        /// The number of bits in our underlying implementation.  Ideally this could be generic.
        pub const BITS: usize = u16::BITS as usize;
        /// The number of bits we use to store the array's current length.
        pub const LEN_BITS: usize = 3;
        /// The number of bits needed to store each item.
        pub const ITEM_BITS: usize = 2;

        /// New [`Room`]s are very simple.
        pub const fn new() -> Self {
            Self(0)
        }

        /// Returns the current number of [`Amphipod`]s in the room.
        pub fn len(&self) -> usize {
            self.0.bit_range(0..Room::LEN_BITS) as usize
        }

        /// A slight optimization of `self.len() == 0` that avoids unnecessary bit manipulation by
        /// directly comparing to 0.
        pub fn is_empty(&self) -> bool {
            self.0 == 0
        }

        /// Return the last amphipod in the room.
        ///
        /// NOTE: only valid for non-empty rooms.
        pub fn peek(&self) -> Amphipod {
            debug_assert!(!self.is_empty(), "Trying to peek an empty Room");

            let start = Room::LEN_BITS + (self.len() - 1) * Room::ITEM_BITS;
            Amphipod::from_index(self.0.bit_range(start..start + Room::ITEM_BITS) as usize)
        }

        /// Get the amphipod at the given index in the room.
        ///
        /// NOTE: only valid for non-empty rooms.
        pub fn get(&self, index: usize) -> Amphipod {
            debug_assert!(
                index < self.len(),
                "Trying to access too far into the room! {} >= {}",
                index,
                self.len()
            );
            let start = Room::LEN_BITS + index * Room::ITEM_BITS;
            Amphipod::from_index(self.0.bit_range(start..start + Room::ITEM_BITS) as usize)
        }

        /// Drop the last amphipod from the room - like [`Vec::pop`], but our use-case doesn't need
        /// its value.
        pub fn drop_last(&mut self) {
            let len = self.len();

            if len == 0 {
                return;
            }

            let start = Room::LEN_BITS + (len - 1) * Room::ITEM_BITS;

            self.0.set_bit_range(0..Room::LEN_BITS, (len - 1) as u16);
            self.0.set_bit_range(start..start + Room::ITEM_BITS, 0);
        }

        /// Remove the amphipod at the given index in the room.
        ///
        /// NOTE: only valid for non-empty rooms.
        pub fn remove(&mut self, index: usize) {
            let len = self.len();

            debug_assert!(
                index < len,
                "Trying to remove too far into the room! {} >= {}",
                index,
                len
            );

            let remainder = Room::LEN_BITS + (index + 1) * Room::ITEM_BITS;
            let target = Room::LEN_BITS + index * Room::ITEM_BITS;

            self.0.set_bit_range(0..Room::LEN_BITS, (len - 1) as u16);
            self.0.set_bit_range(
                target..Room::BITS,
                self.0.bit_range(remainder..Room::BITS as usize) as u16,
            );
        }

        /// Insert an amphipod at the given index in the room, shifting amphipods after it
        /// backwards.
        ///
        /// NOTE: `index` must be in `0..=self.len()`.
        pub fn insert(&mut self, index: usize, amphipod: Amphipod) {
            let len = self.len();

            debug_assert!(
                index <= len,
                "Trying to insert too far into the room! {} > {}",
                index,
                len
            );

            let target = Room::LEN_BITS + index * Room::ITEM_BITS;
            let remainder = Room::LEN_BITS + (index + 1) * Room::ITEM_BITS;

            self.0.set_bit_range(0..Room::LEN_BITS, (len + 1) as u16);
            self.0.set_bit_range(
                remainder..Room::BITS,
                self.0.bit_range(target..Room::BITS as usize) as u16,
            );
            self.0
                .set_bit_range(target..target + Room::ITEM_BITS, amphipod.target() as u16);
        }

        /// Returns an iterator over the elements of the room.
        pub fn iter(&self) -> RoomIterator {
            RoomIterator { i: 0, room: self }
        }
    }

    impl<'a> Iterator for RoomIterator<'a> {
        type Item = Amphipod;

        fn next(&mut self) -> Option<Self::Item> {
            if self.i < self.room.len() {
                self.i += 1;
                Some(self.room.get(self.i - 1))
            } else {
                None
            }
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    // State
    ///////////////////////////////////////////////////////////////////////////

    /// There are 7 live spaces in the hallway, since you can never block a door.
    pub const HALLWAY_SPACES: usize = 7;
    /// There are four rooms.
    pub const ROOM_COUNT: usize = 4;

    /// Represent the current state of the game in a fairly space-efficient layout.
    ///
    /// We're going to be hashing and cloning these frequently, so we want to optimize for space.
    /// We know we don't need to store most of the hallway, and we can represent the four rooms as
    /// [`Room`]s, which fit in u16s, giving us 16 bytes total once counting for padding.
    ///
    /// We could optimize further by compressing `hallway` similarly, but it won't easily yield
    /// results.  Rooms could technically fit in only 10 bits (2 for len, 2 * 4 slots), leaving 24
    /// bits to fit 7 elements of 3 bits each (2 per amphipod + 1 bit for [`Option`] tagging), but
    /// this would require us to represent `State` as a [`u64`] and do bit manipulation for
    /// everything.
    /// Since that representation would be 61 bits already, being any less efficient in
    /// representation would put us at >64 bits, which with padding will fill out to 16 bytes,
    /// making it not worth move effort here.
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct State {
        hallway: [Option<Amphipod>; HALLWAY_SPACES],
        rooms: [Room; ROOM_COUNT],
    }

    ///////////////////////////////////////////////////////////////////////////
    // Implementation
    ///////////////////////////////////////////////////////////////////////////

    /// Hacky parsing code that just grabs the specific bytes that are needed.
    pub fn generator(input: &str) -> State {
        State {
            hallway: Default::default(),
            rooms: {
                let mut rooms = [Room::new(); ROOM_COUNT];

                input.lines().skip(2).take(2).for_each(|line| {
                    let bytes = line.as_bytes();

                    for offset in [3, 5, 7, 9] {
                        rooms[(offset - 3) / 2]
                            .insert(0, Amphipod::from_index((bytes[offset] - b'A') as usize));
                    }
                });

                rooms
            },
        }
    }

    /// Generate transitions from one state to all other possible ones, then use
    /// [A*](https://en.wikipedia.org/wiki/A*_search_algorithm) to
    /// search for the cheapest one.
    ///
    /// For our A* heuristic, we simply assume amphipods can't interfere with each other and
    /// compute the total cost for them to move to the end state in that model.
    ///
    /// Additionally, we implement the priority queue via a [`std::collections::BTreeMap`]
    /// mapping from the heuristic score to a vector of [`State`]s with that score, since that
    /// appears to be substantially more efficient for our use-case than existing priority queue
    /// libraries.
    pub fn solve(mut initial_state: State) -> Cost {
        let mut initial_cost = 0;

        // Remove amphipods that are already parked in their correct location, and set the initial
        // cost to the cost needed to fill the rooms rather than just the first spot in the room.
        for i in 0..initial_state.rooms.len() {
            // Count the number of amphipods that are already at the end of their correct room, and
            // therefore won't need to move.
            let remove_count = initial_state.rooms[i]
                .iter()
                .take_while(|amphipod| amphipod.target() == i)
                .count();

            // Remove the already existing amphipods.
            for _ in 0..remove_count {
                initial_state.rooms[i].remove(0);
            }

            // Assume the rest of this algorithm will move amphipods to the first square of the
            // room, and count the cost needed to fill up the rest of the room.
            let room_cost = Amphipod::from_index(i).cost();
            for depth in 0..initial_state.rooms[i].len() {
                initial_cost += depth * room_cost;
            }

            // Assume the rest of the algorithm will move amphipods out of the first square of the
            // room, and count the cost needed to move to that square from their starting position.
            for (depth, starter) in initial_state.rooms[i].iter().enumerate() {
                initial_cost += (initial_state.rooms[i].len() - depth - 1) * starter.cost();
            }
        }

        let mut frontier = std::collections::BTreeMap::<Cost, Vec<State>>::new();
        let mut best_cost = FnvHashMap::<State, Cost>::default();
        let mut heuristic_cost = FnvHashMap::<State, Cost>::default();

        // Frontier maps best heuristic score -> state
        frontier.insert(initial_cost, vec![initial_state.clone()]);
        best_cost.insert(initial_state.clone(), initial_cost);
        heuristic_cost.insert(initial_state, initial_cost);

        while let Some(&cost) = frontier.keys().next() {
            let states = frontier.remove(&cost).expect("Just found this key");

            for state in states.into_iter() {
                if heuristic_cost[&state] < cost {
                    continue;
                }
                let cost = best_cost[&state];

                if state.hallway.iter().all(|space| space.is_none())
                    && state.rooms.iter().all(|room| room.is_empty())
                {
                    // We're done!
                    return cost;
                }

                let mut visit = |cost_addition, new_state: State| {
                    let existing_cost = *best_cost.get(&new_state).unwrap_or(&Cost::MAX);
                    let new_cost = cost + cost_addition;

                    if existing_cost <= new_cost {
                        // We already have a better way to get to this state.
                        return;
                    }

                    // For a heuristic, we'll figure out how much it costs to move all amphipods to
                    // their target room, assuming nothing gets in their way.
                    let heuristic = new_state
                        .hallway
                        .iter()
                        .enumerate()
                        .filter_map(|(i, spot)| {
                            spot.map(|amphipod| move_cost(&amphipod, i, amphipod.target()))
                        })
                        .sum::<usize>()
                        + new_state
                            .rooms
                            .iter()
                            .enumerate()
                            .map(|(i, room)| {
                                room.iter()
                                    .map(|amphipod| {
                                        // We'll never be able to move to our own room without
                                        // travelling in and out, so we don't do anything special
                                        // for the `target == i` case.
                                        (2 + 2 * ((amphipod.target() - i) as isize).abs()) as usize
                                            * amphipod.cost()
                                    })
                                    .sum::<usize>()
                            })
                            .sum::<usize>();

                    let distance_buf = frontier
                        .entry(new_cost + heuristic)
                        // In practice, 4 fits the vast majority of our vector sizes and avoids
                        // most reallocations.
                        .or_insert_with(|| Vec::with_capacity(4));
                    distance_buf.push(new_state.clone());

                    best_cost.insert(new_state.clone(), new_cost);
                    heuristic_cost.insert(new_state, new_cost + heuristic);
                };

                // How much does it cost for the given amphipod to move between a hallways spot and
                // a room.
                fn move_cost(amphipod: &Amphipod, hall_index: usize, room_index: usize) -> usize {
                    let hall_location = match hall_index {
                        0 => 0,
                        1 => 1,
                        2 => 3,
                        3 => 5,
                        4 => 7,
                        5 => 9,
                        6 => 10,
                        _ => unreachable!(),
                    };
                    let hall_distance =
                        ((2 + 2 * room_index - hall_location) as isize).abs() as usize;

                    (1 + hall_distance) * amphipod.cost()
                }

                // Try to move out of our rooms first.
                for (i, room) in state.rooms.iter().enumerate() {
                    if room.is_empty() {
                        continue;
                    }

                    // First element is at the end of the vector.
                    let moving = room.peek();

                    // We're moving to somewhere other than where we currently are (since we
                    // already threw out amphipods that are in the right spot already).
                    // Move into the available hallway spots.
                    let mut new_state = state.clone();
                    new_state.rooms[i].drop_last();

                    for spot in (0..=i + 1)
                        .rev()
                        .take_while(|&spot| state.hallway[spot].is_none())
                        .chain(
                            (i + 2..HALLWAY_SPACES)
                                .take_while(|&spot| state.hallway[spot].is_none()),
                        )
                    {
                        let mut new_state = new_state.clone();
                        new_state.hallway[spot] = Some(moving);

                        visit(move_cost(&moving, spot, i), new_state);
                    }
                }

                // If we're in the hallway and our target room is open, try to move into it.
                for (i, &moving) in state.hallway.iter().enumerate() {
                    // There's something in the hallway here.
                    if let Some(moving) = moving {
                        let target = moving.target();

                        if !state.rooms[target].is_empty() {
                            // Can't move to the target room, since it's currently occupied.
                            continue;
                        }

                        // Check if anything is blocking us from moving to our destination.
                        let fits = if i < target + 1 {
                            // We're approaching from the left.
                            (i + 1..=target + 1).all(|spot| state.hallway[spot].is_none())
                        } else if i > target + 2 {
                            // We're approaching from the right.
                            (target + 2..i).all(|spot| state.hallway[spot].is_none())
                        } else {
                            // We're right next to the target so definitely fit in.
                            true
                        };

                        if !fits {
                            continue;
                        }

                        let mut new_state = state.clone();
                        new_state.hallway[i] = None;

                        visit(move_cost(&moving, i, target), new_state);
                    }
                }
            }
        }

        unreachable!()
    }

    /// Directly calls [`solve`] on the input.
    pub fn part1(input: &State) -> Cost {
        solve(input.clone())
    }

    /// Same as [`part1`], except with a few lines added to the input.
    pub fn part2(input: &State) -> Cost {
        let mut modified = input.clone();

        modified.rooms[0].insert(1, Amphipod::D);
        modified.rooms[0].insert(2, Amphipod::D);
        modified.rooms[1].insert(1, Amphipod::B);
        modified.rooms[1].insert(2, Amphipod::C);
        modified.rooms[2].insert(1, Amphipod::A);
        modified.rooms[2].insert(2, Amphipod::B);
        modified.rooms[3].insert(1, Amphipod::C);
        modified.rooms[3].insert(2, Amphipod::A);

        solve(modified)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 12521);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            assert_eq!(part2(&input), 44169);
        }

        #[test]
        #[ignore]
        fn test_answers() {
            let input_file = "../data/2021/day23.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 14350);
            assert_eq!(part2(&input), 49742);
        }
    }
}

/// The instruction stream is [`day24::MODEL_DIGITS`] similar sequences (each differing by the arguments
/// to three instructions) implementing rudimentary push/pop with some simple operations on the
/// inputs.
///
/// The differing instructions are `div z [1|26]`, `add x N`, and `add y N`.
///
/// We have two real pathways to handle: push and pop.
///
/// In the push pathway, we don't care about `add x`, since the result always be out of range
/// of the succeeding input check, so the instruction stream is effectively doing `push(input +
/// add y)`.
///
/// In the pop pathway, `add x` serves as a useful check against the input, and if it matches
/// we skip the next push instruction entirely.  Since we only do one pop instruction per
/// instruction stream, we know that we'll only be successful if `input = pop() + add x`.
///
/// This means we can pair our push/pop instructions and find the max values that'll fit
/// into a single digit (where `push.input + push.arg == pop.input - pop.arg`) by first
/// maximizing for push.input (further left in the model number), then solving for pop.input.
pub mod day24 {
    use arrayvec::ArrayVec;
    use itertools::Itertools;

    /// Models are always 14 digit numbers.
    pub const MODEL_DIGITS: usize = 14;

    /// Instruction arguments can be negative.
    pub type Arg = i32;

    /// Each integer being parsed consists of 18 lines of very similar code.
    /// We'll cheat and only represent it with the pieces that are different.
    ///
    /// Part of this instruction stream is implementing push/pop via an integer and
    /// multiplication/division, and we only care about one piece of data in each case.
    #[derive(Debug, Copy, Clone)]
    pub enum StackOp {
        Push(Arg),
        Pop(Arg),
    }

    /// Gross parsing code that makes some strong assumptions about the instruction format in order
    /// to only look at the lines that matter.
    pub fn generator(input: &str) -> Vec<StackOp> {
        const INTEGER_INSTRUCTIONS: usize = 18;

        fn instruction_arg(instruction: Option<&str>) -> Arg {
            let ins = instruction
                .expect("Must be INTEGER_INSTRUCTIONS per integer")
                .trim_end();

            ins[ins
                .rfind(' ')
                .expect("Must be an argument to the instruction")
                + 1..]
                .parse()
                .expect("Argument must be an integer")
        }

        input
            .lines()
            .into_iter()
            .chunks(INTEGER_INSTRUCTIONS)
            .into_iter()
            .map(|mut ins| {
                if instruction_arg(ins.by_ref().nth(4)) == 1 {
                    StackOp::Push(instruction_arg(ins.nth(10)))
                } else {
                    StackOp::Pop(instruction_arg(ins.next()))
                }
            })
            .collect()
    }

    /// Directly interpret the stack machine.
    pub fn interpret_stack<const MAXIMIZE: bool>(ops: &[StackOp]) -> usize {
        let mut stack = ArrayVec::<_, MODEL_DIGITS>::new();
        let mut result = [0; MODEL_DIGITS];

        for (i, ins) in ops.iter().enumerate() {
            match ins {
                StackOp::Push(y_add) => stack.push((i, y_add)),
                StackOp::Pop(x_add) => {
                    let (push_i, y_add) = stack
                        .pop()
                        .expect("Must've been a push operation for the model to succeed");

                    // Can't push something that'll make push.y_add + pop.x_add take up more than
                    // one digit.
                    assert!(x_add + y_add <= 9, "Input needs to be negative");

                    let push_max = if MAXIMIZE {
                        std::cmp::min(9, 9 - (y_add + x_add))
                    } else {
                        // 1 is the minimum digit for some reason
                        std::cmp::max(1, 1 - (y_add + x_add))
                    };
                    let pop_max = push_max + y_add + x_add;

                    result[push_i] = push_max;
                    result[i] = pop_max;
                }
            }
        }

        result
            .into_iter()
            .fold(0, |acc, digit| 10 * acc + digit as usize)
    }

    /// Calls to [`interpret_stack`].
    pub fn part1(ops: &[StackOp]) -> usize {
        interpret_stack::<true>(ops)
    }

    /// Calls to [`interpret_stack`].
    pub fn part2(ops: &[StackOp]) -> usize {
        interpret_stack::<false>(ops)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 13579246899999);
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
            let input_file = "../data/2021/day24.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 94399898949959);
            assert_eq!(part2(&input), 21176121611511);
        }
    }
}

/// Another slight twist on Conway's game of life.
pub mod day25 {
    use arrayvec::ArrayVec;

    /// Cucumbers either have a direction or don't exist.
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum Cucumber {
        Vacant,
        East,
        South,
    }

    /// Width of the grid
    pub const WIDTH: usize = 139;
    /// Height of the grid
    pub const HEIGHT: usize = 137;

    /// Represent the set of cucumbers in a fixed 2D array, since we want the compiler to optimize
    /// copying and overwriting grids.
    pub type CucumberGrid = [[Cucumber; WIDTH]; HEIGHT];

    /// A naive parsing approach.
    pub fn generator(input: &str) -> CucumberGrid {
        input
            .lines()
            .map(|line| {
                line.trim_end()
                    .bytes()
                    .map(|b| match b {
                        b'v' => Cucumber::South,
                        b'>' => Cucumber::East,
                        _ => Cucumber::Vacant,
                    })
                    .collect::<ArrayVec<_, WIDTH>>()
                    .as_slice()
                    .try_into()
                    .expect("Grid is WIDTH wide")
            })
            .collect::<ArrayVec<_, HEIGHT>>()
            .as_slice()
            .try_into()
            .expect("Grid is HEIGHT high")
    }

    /// Run the migration code east then south, until no cucumbers were moved.
    /// While it may seem like we can repeatedly modify the initial grid, this runs into trouble
    /// when cucumbers wrap around, so it's easiest to modify a copy during each iteration.
    pub fn part1(input: &CucumberGrid) -> usize {
        let mut grid = *input;

        for steps in 1.. {
            let mut moved = grid;
            let mut shuffled = false;

            // Shuffle east
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    if grid[y][x] == Cucumber::East {
                        let east = (x + 1) % WIDTH;

                        if grid[y][east] == Cucumber::Vacant {
                            // Free spot for us, shuffle over.
                            moved[y][x] = Cucumber::Vacant;
                            moved[y][east] = Cucumber::East;
                            shuffled = true;
                        }
                    }
                }
            }

            // We've updated the grid for the east step, so need to use that information when we
            // move south.
            grid = moved;

            // Shuffle south
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    if grid[y][x] == Cucumber::South {
                        let south = (y + 1) % HEIGHT;

                        if grid[south][x] == Cucumber::Vacant {
                            // Free spot for us, shuffle over.
                            moved[y][x] = Cucumber::Vacant;
                            moved[south][x] = Cucumber::South;
                            shuffled = true;
                        }
                    }
                }
            }

            if !shuffled {
                // We didn't move at all this step.
                return steps;
            }

            grid = moved;
        }

        unreachable!()
    }

    /// There is no second part!
    pub fn part2(_: &CucumberGrid) -> &'static str {
        "merry christmas!"
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::read_to_string;

        const EXAMPLE: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

        #[test]
        #[ignore]
        fn test_part1() {
            let input = generator(EXAMPLE);
            assert_eq!(part1(&input), 58);
        }

        #[test]
        #[ignore]
        fn test_part2() {
            let input = generator(EXAMPLE);
            //assert_eq!(part2(&input), 5);
        }

        #[test]
        fn test_answers() {
            let input_file = "../data/2021/day25.txt";
            let input_str =
                read_to_string(&input_file).expect(&format!("Input file {} not exist", input_file));
            let input = generator(&input_str);
            assert_eq!(part1(&input), 424);
            //assert_eq!(part2(&input), 1471);
        }
    }
}

/// Benchmark results and information about the systems I benchmarked on.
///
/// Timing is done by calling [`std::time::Instant::now`] before and after each function runs, so
/// there's a little bit of extra overhead.
///
/// These results aren't scientific - each puzzle is run once, in order, on a varyingly loaded
/// desktop system.  However, they should be generally reproduceable.
pub mod benchmarks {
    /// My standard desktop and development system.
    pub struct I6700K {}

    /// An original M1 Macbook Pro.
    pub struct M1Mac {}

    /// A high-end desktop system based on an AMD 5950X.
    pub struct AMD5950X {}

    /// A placeholder trait to show information about benchmarked systems.
    pub trait SystemInfo {}

    /// A placeholder trait to show benchmark results for systems.
    pub trait BenchmarkResult {}

    /// Intel Core i7-6700K
    /// - 128KB L1D$
    /// - 1 MB L2$
    /// - DDR4-3200MHz CL16-16-16-36
    /// - Hyper-threading enabled
    /// - 4.00GHz base clock
    impl SystemInfo for I6700K {}

    /// AMD Ryzen 9 5950X 16-Core Processor
    /// - 512KB L1D$
    /// - 8 MB L2$
    /// - 64 MB L3$
    /// - DDR4-3600MHz CL16-19-19-39
    /// - Hyper-threading enabled
    /// - 3.4GHz base clock
    impl SystemInfo for AMD5950X {}

    /// Macbook Pro M1 2020
    /// - 128KB L1D$
    /// - 12 MB L2$
    /// - Unknown LPDDR4
    /// - No hyperthreading
    /// - 3.2GHz base clock
    impl SystemInfo for M1Mac {}

    /// ```text
    /// Day 1 (35.85µs)
    ///  · Generator (34.62µs)
    ///  · Part 1 (227.00ns) .............. 1475
    ///  · Part 2 (996.00ns) .............. 1515
    ///
    /// Day 2 (30.49µs)
    ///  · Generator (20.08µs)
    ///  · Part 1 (5.07µs) ................ 1604850
    ///  · Part 2 (5.35µs) ................ 1685186100
    ///
    /// Day 3 (49.95µs)
    ///  · Generator (21.12µs)
    ///  · Part 1 (4.99µs) ................ 2498354
    ///  · Part 2 (23.83µs) ............... 3277956
    ///
    /// Day 4 (171.21µs)
    ///  · Generator (46.96µs)
    ///  · Part 1 (29.52µs) ............... 23177
    ///  · Part 2 (94.74µs) ............... 6804
    ///
    /// Day 5 (1.44ms)
    ///  · Generator (41.57µs)
    ///  · Part 1 (854.33µs) .............. 7644
    ///  · Part 2 (543.85µs) .............. 18627
    ///
    /// Day 6 (3.39µs)
    ///  · Generator (3.28µs)
    ///  · Part 1 (31.00ns) ............... 394994
    ///  · Part 2 (81.00ns) ............... 1765974267455
    ///
    /// Day 7 (56.37µs)
    ///  · Generator (50.52µs)
    ///  · Part 1 (283.00ns) .............. 343441
    ///  · Part 2 (5.56µs) ................ 98925151
    ///
    /// Day 8 (126.20µs)
    ///  · Generator (116.04µs)
    ///  · Part 1 (1.99µs) ................ 495
    ///  · Part 2 (8.17µs) ................ 1055164
    ///
    /// Day 9 (369.12µs)
    ///  · Generator (10.55µs)
    ///  · Part 1 (79.46µs) ............... 577
    ///  · Part 2 (279.11µs) .............. 1069200
    ///
    /// Day 10 (145.22µs)
    ///  · Generator (5.31µs)
    ///  · Part 1 (69.80µs) ............... 290691
    ///  · Part 2 (70.12µs) ............... 2768166558
    ///
    /// Day 11 (322.08µs)
    ///  · Generator (761.00ns)
    ///  · Part 1 (58.02µs) ............... 1588
    ///  · Part 2 (263.31µs) .............. 517
    ///
    /// Day 12 (2.22ms)
    ///  · Generator (4.17µs)
    ///  · Part 1 (100.17µs) .............. 5457
    ///  · Part 2 (2.12ms) ................ 128506
    ///
    /// Day 13 (130.95µs)
    ///  · Generator (60.56µs)
    ///  · Part 1 (14.27µs) ............... 847
    ///  · Part 2 (56.12µs) ............... BCZRCEAB
    ///
    /// Day 14 (16.25µs)
    ///  · Generator (8.54µs)
    ///  · Part 1 (2.37µs) ................ 3230
    ///  · Part 2 (5.34µs) ................ 3542388214529
    ///
    /// Day 15 (12.13ms)
    ///  · Generator (8.18µs)
    ///  · Part 1 (431.07µs) .............. 537
    ///  · Part 2 (11.69ms) ............... 2881
    ///
    /// Day 16 (23.98µs)
    ///  · Generator (8.70µs)
    ///  · Part 1 (7.16µs) ................ 1007
    ///  · Part 2 (8.12µs) ................ 834151779165
    ///
    /// Day 17 (32.88µs)
    ///  · Generator (1.24µs)
    ///  · Part 1 (30.00ns) ............... 13041
    ///  · Part 2 (31.61µs) ............... 1031
    ///
    /// Day 18 (356.70µs)
    ///  · Generator (16.40µs)
    ///  · Part 1 (236.41µs) .............. 4480
    ///  · Part 2 (103.89µs) .............. 4676
    ///
    /// Day 19 (3.40ms)
    ///  · Generator (474.76µs)
    ///  · Part 1 (1.46ms) ................ 472
    ///  · Part 2 (1.46ms) ................ 12092
    ///
    /// Day 20 (2.70ms)
    ///  · Generator (10.29µs)
    ///  · Part 1 (48.71µs) ............... 4917
    ///  · Part 2 (2.64ms) ................ 16389
    ///
    /// Day 21 (538.75µs)
    ///  · Generator (204.00ns)
    ///  · Part 1 (1.10µs) ................ 513936
    ///  · Part 2 (537.45µs) .............. 105619718613031
    ///
    /// Day 22 (2.24ms)
    ///  · Generator (106.17µs)
    ///  · Part 1 (28.67µs) ............... 527915
    ///  · Part 2 (2.10ms) ................ 1218645427221987
    ///
    /// Day 23 (42.24ms)
    ///  · Generator (643.00ns)
    ///  · Part 1 (3.51ms) ................ 15338
    ///  · Part 2 (38.73ms) ............... 47064
    ///
    /// Day 24 (9.05µs)
    ///  · Generator (8.72µs)
    ///  · Part 1 (157.00ns) .............. 99919765949498
    ///  · Part 2 (172.00ns) .............. 24913111616151
    ///
    /// Day 25 (22.42ms)
    ///  · Generator (22.47µs)
    ///  · Part 1 (22.39ms) ............... 305
    ///  · Part 2 (29.00ns) ............... merry christmas!
    ///
    /// Overall runtime (91.73ms)
    /// ```
    impl BenchmarkResult for I6700K {}

    /// ```text
    /// Day 1 (22.78µs)
    ///  · Generator (21.62µs)
    ///  · Part 1 (280.00ns) .............. 1475
    ///  · Part 2 (880.00ns) .............. 1515
    ///
    /// Day 2 (21.56µs)
    ///  · Generator (14.27µs)
    ///  · Part 1 (3.65µs) ................ 1604850
    ///  · Part 2 (3.64µs) ................ 1685186100
    ///
    /// Day 3 (39.94µs)
    ///  · Generator (15.98µs)
    ///  · Part 1 (2.98µs) ................ 2498354
    ///  · Part 2 (20.98µs) ............... 3277956
    ///
    /// Day 4 (124.79µs)
    ///  · Generator (28.55µs)
    ///  · Part 1 (22.12µs) ............... 23177
    ///  · Part 2 (74.12µs) ............... 6804
    ///
    /// Day 5 (1.18ms)
    ///  · Generator (28.43µs)
    ///  · Part 1 (671.87µs) .............. 7644
    ///  · Part 2 (475.67µs) .............. 18627
    ///
    /// Day 6 (2.08µs)
    ///  · Generator (2.00µs)
    ///  · Part 1 (20.00ns) ............... 394994
    ///  · Part 2 (60.00ns) ............... 1765974267455
    ///
    /// Day 7 (36.47µs)
    ///  · Generator (32.81µs)
    ///  · Part 1 (200.00ns) .............. 343441
    ///  · Part 2 (3.46µs) ................ 98925151
    ///
    /// Day 8 (87.72µs)
    ///  · Generator (81.29µs)
    ///  · Part 1 (830.00ns) .............. 495
    ///  · Part 2 (5.60µs) ................ 1055164
    ///
    /// Day 9 (267.03µs)
    ///  · Generator (6.41µs)
    ///  · Part 1 (51.70µs) ............... 577
    ///  · Part 2 (208.92µs) .............. 1069200
    ///
    /// Day 10 (93.28µs)
    ///  · Generator (5.44µs)
    ///  · Part 1 (42.43µs) ............... 290691
    ///  · Part 2 (45.41µs) ............... 2768166558
    ///
    /// Day 11 (281.88µs)
    ///  · Generator (440.00ns)
    ///  · Part 1 (48.18µs) ............... 1588
    ///  · Part 2 (233.26µs) .............. 517
    ///
    /// Day 12 (1.72ms)
    ///  · Generator (2.78µs)
    ///  · Part 1 (74.48µs) ............... 5457
    ///  · Part 2 (1.65ms) ................ 128506
    ///
    /// Day 13 (102.18µs)
    ///  · Generator (47.02µs)
    ///  · Part 1 (12.09µs) ............... 847
    ///  · Part 2 (43.07µs) ............... BCZRCEAB
    ///
    /// Day 14 (11.25µs)
    ///  · Generator (6.24µs)
    ///  · Part 1 (1.55µs) ................ 3230
    ///  · Part 2 (3.46µs) ................ 3542388214529
    ///
    /// Day 15 (8.38ms)
    ///  · Generator (5.14µs)
    ///  · Part 1 (309.87µs) .............. 537
    ///  · Part 2 (8.07ms) ................ 2881
    ///
    /// Day 16 (14.29µs)
    ///  · Generator (4.75µs)
    ///  · Part 1 (4.38µs) ................ 1007
    ///  · Part 2 (5.16µs) ................ 834151779165
    ///
    /// Day 17 (18.23µs)
    ///  · Generator (770.00ns)
    ///  · Part 1 (30.00ns) ............... 13041
    ///  · Part 2 (17.43µs) ............... 1031
    ///
    /// Day 18 (246.76µs)
    ///  · Generator (9.80µs)
    ///  · Part 1 (162.87µs) .............. 4480
    ///  · Part 2 (74.09µs) ............... 4676
    ///
    /// Day 19 (2.71ms)
    ///  · Generator (428.78µs)
    ///  · Part 1 (1.16ms) ................ 472
    ///  · Part 2 (1.12ms) ................ 12092
    ///
    /// Day 20 (1.25ms)
    ///  · Generator (6.89µs)
    ///  · Part 1 (31.56µs) ............... 4917
    ///  · Part 2 (1.21ms) ................ 16389
    ///
    /// Day 21 (334.75µs)
    ///  · Generator (120.00ns)
    ///  · Part 1 (990.00ns) .............. 513936
    ///  · Part 2 (333.64µs) .............. 105619718613031
    ///
    /// Day 22 (1.36ms)
    ///  · Generator (78.25µs)
    ///  · Part 1 (18.96µs) ............... 527915
    ///  · Part 2 (1.26ms) ................ 1218645427221987
    ///
    /// Day 23 (31.25ms)
    ///  · Generator (290.00ns)
    ///  · Part 1 (2.79ms) ................ 15338
    ///  · Part 2 (28.45ms) ............... 47064
    ///
    /// Day 24 (5.67µs)
    ///  · Generator (5.45µs)
    ///  · Part 1 (110.00ns) .............. 99919765949498
    ///  · Part 2 (110.00ns) .............. 24913111616151
    ///
    /// Day 25 (13.90ms)
    ///  · Generator (16.77µs)
    ///  · Part 1 (13.88ms) ............... 305
    ///  · Part 2 (20.00ns) ............... merry christmas!
    ///
    /// Overall runtime (63.70ms)
    /// ```
    impl BenchmarkResult for AMD5950X {}

    ///```text
    /// Day 1 (29.37µs)
    ///  · Generator (28.00µs)
    ///  · Part 1 (166.00ns) .............. 1475
    ///  · Part 2 (1.21µs) ................ 1515
    ///
    /// Day 2 (28.00µs)
    ///  · Generator (19.46µs)
    ///  · Part 1 (4.25µs) ................ 1604850
    ///  · Part 2 (4.29µs) ................ 1685186100
    ///
    /// Day 3 (46.17µs)
    ///  · Generator (19.75µs)
    ///  · Part 1 (3.50µs) ................ 2498354
    ///  · Part 2 (22.92µs) ............... 3277956
    ///
    /// Day 4 (152.25µs)
    ///  · Generator (38.21µs)
    ///  · Part 1 (25.88µs) ............... 23177
    ///  · Part 2 (88.17µs) ............... 6804
    ///
    /// Day 5 (909.92µs)
    ///  · Generator (33.50µs)
    ///  · Part 1 (467.17µs) .............. 7644
    ///  · Part 2 (409.25µs) .............. 18627
    ///
    /// Day 6 (2.67µs)
    ///  · Generator (2.42µs)
    ///  · Part 1 (0.00ns) ................ 394994
    ///  · Part 2 (250.00ns) .............. 1765974267455
    ///
    /// Day 7 (51.25µs)
    ///  · Generator (46.13µs)
    ///  · Part 1 (291.00ns) .............. 343441
    ///  · Part 2 (4.83µs) ................ 98925151
    ///
    /// Day 8 (112.92µs)
    ///  · Generator (104.96µs)
    ///  · Part 1 (541.00ns) .............. 495
    ///  · Part 2 (7.42µs) ................ 1055164
    ///
    /// Day 9 (338.46µs)
    ///  · Generator (7.79µs)
    ///  · Part 1 (60.79µs) ............... 577
    ///  · Part 2 (269.88µs) .............. 1069200
    ///
    /// Day 10 (118.21µs)
    ///  · Generator (4.92µs)
    ///  · Part 1 (55.13µs) ............... 290691
    ///  · Part 2 (58.17µs) ............... 2768166558
    ///
    /// Day 11 (242.25µs)
    ///  · Generator (625.00ns)
    ///  · Part 1 (46.04µs) ............... 1588
    ///  · Part 2 (195.58µs) .............. 517
    ///
    /// Day 12 (1.78ms)
    ///  · Generator (3.33µs)
    ///  · Part 1 (83.21µs) ............... 5457
    ///  · Part 2 (1.69ms) ................ 128506
    ///
    /// Day 13 (84.50µs)
    ///  · Generator (53.63µs)
    ///  · Part 1 (9.67µs) ................ 847
    ///  · Part 2 (21.21µs) ............... BCZRCEAB
    ///
    /// Day 14 (15.21µs)
    ///  · Generator (8.58µs)
    ///  · Part 1 (2.08µs) ................ 3230
    ///  · Part 2 (4.54µs) ................ 3542388214529
    ///
    /// Day 15 (10.75ms)
    ///  · Generator (7.29µs)
    ///  · Part 1 (388.88µs) .............. 537
    ///  · Part 2 (10.35ms) ............... 2881
    ///
    /// Day 16 (30.21µs)
    ///  · Generator (6.21µs)
    ///  · Part 1 (5.17µs) ................ 1007
    ///  · Part 2 (18.83µs) ............... 834151779165
    ///
    /// Day 17 (28.42µs)
    ///  · Generator (1.46µs)
    ///  · Part 1 (0.00ns) ................ 13041
    ///  · Part 2 (26.96µs) ............... 1031
    ///
    /// Day 18 (357.12µs)
    ///  · Generator (17.04µs)
    ///  · Part 1 (234.54µs) .............. 4480
    ///  · Part 2 (105.54µs) .............. 4676
    ///
    /// Day 19 (2.96ms)
    ///  · Generator (348.13µs)
    ///  · Part 1 (1.34ms) ................ 472
    ///  · Part 2 (1.28ms) ................ 12092
    ///
    /// Day 20 (1.56ms)
    ///  · Generator (10.17µs)
    ///  · Part 1 (41.58µs) ............... 4917
    ///  · Part 2 (1.51ms) ................ 16389
    ///
    /// Day 21 (356.62µs)
    ///  · Generator (291.00ns)
    ///  · Part 1 (750.00ns) .............. 513936
    ///  · Part 2 (355.58µs) .............. 105619718613031
    ///
    /// Day 22 (2.02ms)
    ///  · Generator (81.29µs)
    ///  · Part 1 (23.00µs) ............... 527915
    ///  · Part 2 (1.92ms) ................ 1218645427221987
    ///
    /// Day 23 (34.67ms)
    ///  · Generator (708.00ns)
    ///  · Part 1 (3.43ms) ................ 15338
    ///  · Part 2 (31.25ms) ............... 47064
    ///
    /// Day 24 (7.00µs)
    ///  · Generator (6.75µs)
    ///  · Part 1 (83.00ns) ............... 99919765949498
    ///  · Part 2 (166.00ns) .............. 24913111616151
    ///
    /// Day 25 (19.47ms)
    ///  · Generator (18.00µs)
    ///  · Part 1 (19.45ms) ............... 305
    ///  · Part 2 (0.00ns) ................ merry christmas!
    ///
    /// Overall runtime (76.63ms)
    /// ```
    impl BenchmarkResult for M1Mac {}
}
