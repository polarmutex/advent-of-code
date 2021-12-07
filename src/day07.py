from __future__ import annotations

import numpy
import argparse
import os.path
from typing import List

from support import timing

HorizPos = int
CrabSubs = List[HorizPos]

INPUT_TXT = os.path.join(os.path.dirname(__file__),
                         "aoc_data", "day07_github.txt")
INPUT_S = """\
16,1,2,0,4,2,7,1,2,14
"""


def parse(input: str) -> CrabSubs:
    crabs: CrabSubs = [int(num) for num in input.split(",")]
    return crabs


def solve(puzzle_input: str):
    solution1 = part1(puzzle_input)
    solution2 = part2(puzzle_input)
    return solution1, solution2


def part1(input: str):
    data = parse(input)
    median_val = numpy.median(data)
    return sum([abs(num-median_val) for num in data])


def calculate_fuel(list: List[int], mean: int):
    def series_sum(n): return n*(n+1)/2
    return int(sum([series_sum(abs(mean-i)) for i in list]))


def part2(input: str):
    data = parse(input)
    fuel = [calculate_fuel(data, i) for i in range(max(data))]
    return min(fuel)


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
