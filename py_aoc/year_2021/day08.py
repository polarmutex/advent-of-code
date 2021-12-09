from __future__ import annotations

import numpy
import argparse
import os.path
from typing import List, Tuple, Dict, Optional

from support import timing

SignalPattern = str
NumberOutput = str
Entry = Tuple[List[SignalPattern], List[NumberOutput]]

INPUT_TXT = os.path.join("data", "2021", "day08_github.txt")
INPUT_S = """\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |fgae cfgab fg bagce
"""


def parse(input: str) -> List[Entry]:
    entries: List[Entry] = []
    for line in input.splitlines():
        temp: List[str] = line.split("|")
        signal_patterns: List[SignalPattern] = temp[0].split()
        number_output: List[NumberOutput] = temp[1].split()
        entries.append((signal_patterns, number_output))
    return entries


def solve(puzzle_input: str):
    solution1 = part1(puzzle_input)
    solution2 = part2(puzzle_input)
    return solution1, solution2


def part1(input: str):
    data: List[Entry] = parse(input)
    unique_digits: int = 0
    for entry in data:
        for digit in entry[1]:
            if (len(digit) == 2) or (len(digit) == 3) or (len(digit) == 4) or (len(digit) == 7):
                unique_digits += 1
    return unique_digits


def sort_string(string: str) -> str:
    return ''.join(sorted(string))


def find_1(input: List[str]):
    for item in input:
        if len(item) == 2:
            return item


def find_235(input: List[str], chars: List[str]):
    assert(len(chars) == 2)
    char1 = chars[0]
    char2 = chars[1]
    for item in input:
        if len(item) == 5:
            if char1 in item and char2 in item:
                return item


def find_4(input: List[str]):
    for item in input:
        if len(item) == 4:
            return item


def find_069(input: List[str], chars: List[str]):
    assert(len(chars) == 2)
    char1 = chars[0]
    char2 = chars[1]
    for item in input:
        if len(item) == 6:
            if char1 not in item or char2 not in item:
                return item


def find_7(input: List[str]):
    for item in input:
        if len(item) == 3:
            return item


def find_8(input: List[str]):
    for item in input:
        if len(item) == 7:
            return item


"""
 aaaa
b    c
b    c
 dddd
e    f
e    f
 gggg

0 - 6 - abc efg
1 - 2 -   c  f
2 - 5 - a cde g
3 - 5 - a cd fg
4 - 4 -  bcd f
5 - 5 - ab d fg
6 - 6 - ab defg
7 - 3 - a c  f
8 - 7 - abcdefg
9 - 6 - abcd fg
"""


def part2(input: str):
    data = parse(input)

    sum: int = 0
    for entry in data:
        sig_patterns = []
        for sig in entry[0]:
            sig_patterns.append(sort_string(sig))

        number_map: Dict[int, Optional[str]] = {
            0: None,
            2: None,
            1: None,
            3: None,
            4: None,
            5: None,
            6: None,
            7: None,
            8: None,
            9: None,
        }
        signal_map: Dict[str, List[str]] = {
            "a": [],
            "b": [],
            "c": [],
            "d": [],
            "e": [],
            "f": [],
            "g": [],
        }

        number_map[1] = find_1(sig_patterns)
        if number_map[1] is not None:
            sig_patterns.remove(number_map[1])

        number_map[4] = find_4(sig_patterns)
        if number_map[4] is not None:
            sig_patterns.remove(number_map[4])

        number_map[7] = find_7(sig_patterns)
        if number_map[7] is not None:
            sig_patterns.remove(number_map[7])

        number_map[8] = find_8(sig_patterns)
        if number_map[8] is not None:
            sig_patterns.remove(number_map[8])

        # find "a" signal
        if number_map[7] is not None and number_map[1] is not None:
            for char in number_map[7]:
                if char not in number_map[1]:
                    signal_map["a"].append(char)

        # "c" and "f" only make up one
        if number_map[1] is not None:
            for char in number_map[1]:
                signal_map["c"].append(char)
                signal_map["f"].append(char)

        # taking away one signals leaves us "b" and "d" of four
        if number_map[4] is not None:
            for char in number_map[4]:
                if char not in signal_map["c"]:
                    signal_map["b"].append(char)
                    signal_map["d"].append(char)

        # taking away one, four, seven signals leaves us "e" and "g" of eight
        if number_map[8] is not None:
            for char in number_map[8]:
                if char not in signal_map["c"] and char not in signal_map["b"] and char not in signal_map["a"]:
                    signal_map["e"].append(char)
                    signal_map["g"].append(char)

        # only 3 has c and f signal of 5 len strings
        number_map[3] = find_235(sig_patterns, signal_map["c"])
        if number_map[3] is not None:
            sig_patterns.remove(number_map[3])

        # only 5 has b and d signal of 5 len strings
        number_map[5] = find_235(sig_patterns, signal_map["b"])
        if number_map[5] is not None:
            sig_patterns.remove(number_map[5])

        # only 2 has e and g signal of 5 len strings
        number_map[2] = find_235(sig_patterns, signal_map["e"])
        if number_map[2] is not None:
            sig_patterns.remove(number_map[2])

        # only 6 does not have c and f signal of 6 len strings
        number_map[6] = find_069(sig_patterns, signal_map["c"])
        if number_map[6] is not None:
            sig_patterns.remove(number_map[6])

        # only 0 does not have b and d signal of 6 len strings
        number_map[0] = find_069(sig_patterns, signal_map["b"])
        if number_map[0] is not None:
            sig_patterns.remove(number_map[0])

        # only 9 does not have e and g signal of 6 len strings
        number_map[9] = find_069(sig_patterns, signal_map["e"])
        if number_map[9] is not None:
            sig_patterns.remove(number_map[9])

        # all number strings should be found
        assert(len(sig_patterns) == 0)

        # print(signal_map)
        # print(number_map)

        number: str = ""
        print(entry[1])
        for num in entry[1]:
            sort_num: str = sort_string(num)
            if sort_num == number_map[0]:
                number += "0"
            elif sort_num == number_map[1]:
                number += "1"
            elif sort_num == number_map[2]:
                number += "2"
            elif sort_num == number_map[3]:
                number += "3"
            elif sort_num == number_map[4]:
                number += "4"
            elif sort_num == number_map[5]:
                number += "5"
            elif sort_num == number_map[6]:
                number += "6"
            elif sort_num == number_map[7]:
                number += "7"
            elif sort_num == number_map[8]:
                number += "8"
            elif sort_num == number_map[9]:
                number += "9"
            else:
                number += "E"

        sum += int(number)

    return sum


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("data_file", nargs="?", default=INPUT_TXT)
    args = parser.parse_args()

    with open(args.data_file) as f, timing():
        solutions = solve(f.read())
        print("\n".join(str(solution) for solution in solutions))

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
