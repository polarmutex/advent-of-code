from __future__ import annotations

import argparse
import os.path
import collections
from collections import Counter
from typing import List, Tuple, Generator, Set, Dict, NamedTuple
from bitarray.util import hex2ba, ba2int, int2ba
from bitarray import bitarray

from support import timing

INPUT_GITHUB = os.path.join("data", "2021", "day17_github.txt")
INPUT_GOOGLE = os.path.join("data", "2021", "day17_google.txt")
INPUT_REDDIT = os.path.join("data", "2021", "day17_reddit.txt")
INPUT_TWITTER = os.path.join("data", "2021", "day17_twitter.txt")
INPUT_S = """\
target area: x=20..30, y=-10..-5
"""

PolymerTemplate = str
Point = Tuple[int, int]
Target = Tuple[Tuple[int, int], Tuple[int, int]]
Coord = Tuple[int, int]


def parse(input: str) -> Target:
    x_str, y_str = input.replace("target area: ", "").split(", ")
    x = [int(num) for num in x_str.replace("x=", "").split("..")]
    y = [int(num) for num in y_str.replace("y=", "").split("..")]
    return (int(x[0]), int(x[1])), (int(y[0]), int(y[1]))


def solve(puzzle_input: str) -> Tuple[int, int]:
    solution1 = part1(puzzle_input)
    solution2 = part2(puzzle_input)
    return solution1, solution2


def find_x_vel_ranges(target: Tuple[int, int]) -> Tuple[int, int]:
    min_steps: int = 0
    sum: int = 0
    while sum < target[0]:
        min_steps += 1
        sum += min_steps
    lowest_x_vel: int = min_steps
    highest_x_vel: int = target[1]
    return (lowest_x_vel, highest_x_vel)


def find_y_vel_ranges(target: Tuple[int, int]) -> Tuple[int, int]:
    lowest_y_vel: int = target[0]
    highest_y_vel: int = abs(target[0]) - 1
    return (lowest_y_vel, highest_y_vel)


def is_successful(vel: Tuple[int, int], target: Target) -> Tuple[bool, int]:
    cur: Tuple[int, int] = (0, 0)
    max_y: int = 0
    while True:
        cur = (cur[0] + vel[0], cur[1] + vel[1])
        print(f"cur x: {cur[0]} y: {cur[1]}")
        if cur[1] > max_y:
            max_y = cur[1]

        if (target[0][0] <= cur[0] <= target[0][1]) and (target[1][0] <= cur[1] <= target[1][1]):
            return True, max_y
        elif cur[0] > target[0][1]:
            return False, max_y
        elif cur[1] < target[1][0]:
            return False, max_y

        vel = (vel[0] - 1 if vel[0] > 0 else 0, vel[1] - 1)

    return False, max_y


def part1(input: str) -> int:
    x_tgt, y_tgt = parse(input)
    x_vel_range = find_x_vel_ranges(x_tgt)
    y_vel_range = find_y_vel_ranges(y_tgt)

    max_y: int = 0
    for x in range(x_vel_range[0], x_vel_range[1] + 1):
        for y in range(y_vel_range[0], y_vel_range[1] + 1):
            success, max_hght = is_successful((x, y), (x_tgt, y_tgt))
            if max_hght > max_y:
                max_y = max_hght
            print("")
    return max_y


def part2(input: str) -> int:
    x_tgt, y_tgt = parse(input)
    x_vel_range = find_x_vel_ranges(x_tgt)
    y_vel_range = find_y_vel_ranges(y_tgt)

    hit_target = set()
    for x in range(x_vel_range[0], x_vel_range[1] + 1):
        for y in range(y_vel_range[0], y_vel_range[1] + 1):
            success, _ = is_successful((x, y), (x_tgt, y_tgt))
            if success:
                hit_target.add((x, y))
    return len(hit_target)


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
