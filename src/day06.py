from __future__ import annotations

import argparse
import os.path
from collections import defaultdict
from typing import Dict, List, Tuple

from support import timing

FishTimer = int
Fish = List[FishTimer]
NumFish = int

INPUT_TXT = os.path.join(os.path.dirname(__file__),
                         "aoc_data", "day06_github.txt")
INPUT_S = """\
3,4,3,1,2
"""


def parse(input: str) -> Fish:
    fish: Fish = [int(num) for num in input.split(",")]
    return fish


def compute(input: Fish, days_to_run: int) -> int:
    total_fish: int = 0

    fish_dict: Dict[FishTimer, NumFish] = defaultdict(lambda: 0)

    # setup inital count
    for timer in input:
        fish_dict[timer] += 1

    for _ in range(days_to_run):
        next_fish_dict: Dict[FishTimer, NumFish] = defaultdict(lambda: 0)
        for timer, number in fish_dict.items():
            if timer == 0:
                next_fish_dict[8] += number
                next_fish_dict[6] += number
            else:
                next_fish_dict[timer - 1] += number
        fish_dict = next_fish_dict

    for num_fish in fish_dict.values():
        total_fish += num_fish

    return total_fish


def solve(puzzle_input: str):
    solution1 = part1(puzzle_input)
    solution2 = part2(puzzle_input)
    return solution1, solution2


def part1(input: str):
    data = parse(input)
    return compute(data, 80)


def part2(input: str):
    data = parse(input)
    return compute(data, 256)


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
