from __future__ import annotations

import argparse
import os.path
import re
import math
import collections
from collections import Counter
from typing import List, Tuple, Generator, Set, Dict, NamedTuple, Optional, Match
from bitarray.util import hex2ba, ba2int, int2ba
from bitarray import bitarray

from support import timing

INPUT_GITHUB = os.path.join("data", "2021", "day18_github.txt")
INPUT_GOOGLE = os.path.join("data", "2021", "day18_google.txt")
INPUT_REDDIT = os.path.join("data", "2021", "day18_reddit.txt")
INPUT_TWITTER = os.path.join("data", "2021", "day18_twitter.txt")
INPUT_S = """\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
"""


def parse(input: str) -> str:
    return ""


def solve(puzzle_input: str) -> Tuple[int, int]:
    solution1 = part1(puzzle_input)
    solution2 = part2(puzzle_input)
    return solution1, solution2


SnailFishNumber = str
PAIR_RE = re.compile(r'\[(\d+),(\d+)\]')
DOUBLENUM_RE = re.compile(r'\d\d+')
NUM_RE = re.compile(r'\d+')
LEFT_NUM_RE = re.compile(r'\d+(?!.*\d)')
"""
\\d Digit (1 or more
?! Negative look ahead
regexr.com/6br8v
"""


def addition(left: SnailFishNumber, right: SnailFishNumber) -> SnailFishNumber:
    return f"[{left},{right}]"


def find_level(input: str, idx: int) -> int:
    num_left_bracket = input.count("[", 0, idx)
    num_right_bracket = input.count("]", 0, idx)
    return num_left_bracket - num_right_bracket


def explode(input: str, pair: Match[str]) -> str:
    def left(match: Match[str]) -> str:
        return str(int(match[0]) + int(pair[1]))

    def right(match: Match[str]) -> str:
        return str(int(match[0]) + int(pair[2]))

    start = LEFT_NUM_RE.sub(left, input[:pair.start()], count=1)
    end = NUM_RE.sub(right, input[pair.end():], count=1)
    input = f'{start}0{end}'

    return input


def split(input: str, pair: Match[str]) -> str:

    def cb(match: Match[str]) -> str:
        return f"[{math.floor(int(match[0])/2)},{math.ceil(int(match[0])/2)}]"

    input = DOUBLENUM_RE.sub(cb, input, count=1)
    return input


def reduce(input: SnailFishNumber) -> SnailFishNumber:
    exploded_happend: bool = True
    while exploded_happend:
        exploded_happend = False
        pairs = PAIR_RE.finditer(input)
        for pair in pairs:
            num_lvls = find_level(input, pair.start())
            if num_lvls >= 4:
                exploded_happend = True
                input = explode(input, pair)
                break
        if exploded_happend:
            continue

        pairs = DOUBLENUM_RE.finditer(input)
        for pair in pairs:
            input = split(input, pair)
            exploded_happend = True
            break

    return input


def magnatude(input: str) -> int:
    while "[" in input:
        pairs = PAIR_RE.finditer(input)
        for pair in pairs:
            input = input[:pair.start()] + \
                f"{3*int(pair.groups()[0]) + 2*int(pair.groups()[1])}" + \
                input[pair.end():]
            break
    return int(input)


def part1(input: str) -> int:
    prev_result: Optional[str] = None
    for i, line in enumerate(input.splitlines()):
        if i == 0:
            # cannot add just 1 number
            continue
        if prev_result is not None:
            prev_result = addition(prev_result, line)
        else:
            prev_result = addition(input.splitlines()[i-1], line)

        prev_result = reduce(prev_result)

    if prev_result is None:
        return 0
    return magnatude(prev_result)


def part2(input: str) -> int:
    largest_mag: int = 0
    for i, x in enumerate(input.splitlines()):
        for j, y in enumerate(input.splitlines()):
            if i == j:
                continue
            result = addition(x, y)
            result = reduce(result)
            mag = magnatude(result)
            if mag > largest_mag:
                largest_mag = mag

    return largest_mag


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("data_file", nargs="?", default=INPUT_GITHUB)
    args = parser.parse_args()

    with open(args.data_file) as f, timing():
        solutions = solve(f.read())
        print("\n".join(str(solution) for solution in solutions))

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
