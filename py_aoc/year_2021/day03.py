from __future__ import annotations

import argparse
import os.path
from typing import List

from bitarray import bitarray
from bitarray.util import ba2int

from support import timing

INPUT_GITHUB = os.path.join("data", "2021", "day03_github.txt")
INPUT_GOOGLE = os.path.join("data", "2021", "day03_google.txt")
INPUT_REDDIT = os.path.join("data", "2021", "day03_reddit.txt")
INPUT_TWITTER = os.path.join("data", "2021", "day03_twitter.txt")

INPUT_S = """\
00100
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
01010
"""


def part1(s: str) -> int:
    lines = s.splitlines()
    total = len(lines)
    line_total: int = 0
    count_1: List[int] = []
    for line_index, line in enumerate(lines):
        if line_index == 0:
            line_total = len(line)

        for index, char in enumerate(line):
            if line_index == 0:
                count_1.append(0)
            if char == "1":
                count_1[index] += 1

    gamma: bitarray = bitarray(line_total)
    epsilon: bitarray = bitarray(line_total)
    for index, count in enumerate(count_1):
        if count > total / 2:
            gamma[index] = 1
            epsilon[index] = 0
        else:
            gamma[index] = 0
            epsilon[index] = 1

    return ba2int(gamma) * ba2int(epsilon)


def part2(s: str) -> int:
    lines = s.splitlines()
    line_total: int = len(lines[0])

    oxygen_rating = lines
    for index in range(line_total):
        ba = bitarray(len(oxygen_rating))
        for line_index, line in enumerate(oxygen_rating):
            ba[line_index] = int(line[index])

        result = []
        if ba.count(1) >= len(oxygen_rating) / 2:
            for o_line in oxygen_rating:
                if o_line[index] == "1":
                    result.append(o_line)
        else:
            for o_line in oxygen_rating:
                if o_line[index] == "0":
                    result.append(o_line)
        oxygen_rating = result

        if len(oxygen_rating) == 1:
            break
    print(oxygen_rating)

    co2_rating = lines
    for index in range(line_total):
        ba = bitarray(len(co2_rating))
        for line_index, line in enumerate(co2_rating):
            ba[line_index] = int(line[index])

        result = []
        if ba.count(1) < len(co2_rating) / 2:
            for c_line in co2_rating:
                if c_line[index] == "1":
                    result.append(c_line)
        else:
            for c_line in co2_rating:
                if c_line[index] == "0":
                    result.append(c_line)
        co2_rating = result

        if len(co2_rating) == 1:
            break
    print(co2_rating)

    return ba2int(bitarray(oxygen_rating[0])) * ba2int(bitarray(co2_rating[0]))


def solve(puzzle_input: str):
    """Solve the puzzle for the given input"""
    data = puzzle_input
    solution1 = part1(data)
    solution2 = part2(data)
    return solution1, solution2


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
