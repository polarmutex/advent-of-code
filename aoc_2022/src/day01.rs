use crate::prelude::*;

day!(1, parse => part1, part2);

#[derive(Debug, Clone)]
struct Elf {
    food: Vec<u32>,
}

//fn input_parser() -> impl Parser<char, Vec<Vec<u32>>, Error = Simple<char>> {
//    let elf_line = c::text::int(10)
//        .map(|s: String| s.parse().unwrap())
//        .separated_by(c::text::newline());
//    (c::text::int(10)).separated_by(just("\n\n"))
//}

fn parse(input: &str) -> ParseResult<Vec<Elf>> {
    //Ok(input_parser().parse(input).unwrap())
    let elf_groups: Vec<&str> = input.split("\n\n").collect();
    let mut elfs: Vec<Elf> = vec![];
    for elf in elf_groups {
        let elf_temp: Vec<&str> = elf.split('\n').collect();
        let mut elf_food: Vec<u32> = vec![];
        for i in elf_temp {
            if !i.is_empty() {
                elf_food.push(i.parse().unwrap());
            }
        }
        elfs.push(Elf { food: elf_food });
    }
    Ok(elfs)
}

fn part1(input: &[Elf]) -> u32 {
    let mut largest = 0;
    for elf in input {
        let mut sum = 0;
        for &food in &elf.food {
            sum += food
        }
        if sum > largest {
            largest = sum
        }
    }
    largest
}

fn part2(input: &[Elf]) -> u32 {
    let mut sums: Vec<u32> = vec![];
    for elf in input {
        let mut sum = 0;
        for &food in &elf.food {
            sum += food;
        }
        sums.push(sum);
    }
    sums.sort();
    sums.pop().unwrap() + sums.pop().unwrap() + sums.pop().unwrap()
}

tests! {
    const EXAMPLE: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    simple_tests!(parse, part1, part1_example_test, EXAMPLE => 24000);
    input_tests!(2022, 1, parse, part1, part1_input_test, 68802);
    simple_tests!(parse, part2, part2_example_test, EXAMPLE => 45000); // 24000 + 11000 + 45000
    input_tests!(2022, 1, parse, part2, part2_input_test, 205370);
}
