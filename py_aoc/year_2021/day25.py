from __future__ import annotations

import argparse
import os.path
import collections
from collections import Counter
from typing import List, Tuple, Generator, Set, Dict, NamedTuple
import heapq
from copy import deepcopy
import z3

from support import timing

INPUT_GITHUB = os.path.join("data", "2021", "day24_github.txt")
INPUT_GOOGLE = os.path.join("data", "2021", "day24_google.txt")
INPUT_REDDIT = os.path.join("data", "2021", "day24_reddit.txt")
INPUT_TWITTER = os.path.join("data", "2021", "day24_twitter.txt")
INPUT_S = """\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
"""

Point = Tuple[int, int]
Map = Dict[Point, str]


def parse(input: str) -> Map:
    coords: Dict[Tuple[int, int], str] = {}
    for y, line in enumerate(input.splitlines()):
        for x, char in enumerate(line):
            coords[(x, y)] = char
    return coords


def solve(puzzle_input: str) -> Tuple[int, int]:
    solution1 = part1(puzzle_input)
    solution2 = part2(puzzle_input)
    return solution1, solution2


def print_(map: Map, step: int) -> None:
    max_x, max_y = max(map)
    print(f"------ {step} --------")
    for y in range(max_y+1):
        line = ""
        for x in range(max_x+1):
            line += map[(x, y)]
        print(line)
    print("----------------------")


def east_gen(x: int, y: int, maxx: int) -> Point:
    if x == maxx:
        return 0, y
    else:
        return x+1, y


def south_gen(x: int, y: int, maxy: int) -> Point:
    if y == maxy:
        return x, 0
    else:
        return x, y+1


def part1(input: str) -> int:
    map = parse(input)

    max_x, max_y = max(map)
    print(f"maxx: {max_x} maxy: {max_y}")

    steps: int = 0
    print_(map, steps)

    while True:
        seacucumber_moved: bool = False
        steps += 1
        next_map = deepcopy(map)
        for coord, val in map.items():
            if val == '>':
                check_pt = east_gen(*coord, max_x)
                if map[check_pt] == '.':
                    next_map[check_pt] = '>'
                    next_map[coord] = '.'
                    seacucumber_moved = True

        map = next_map
        next_map = deepcopy(map)

        for coord, val in map.items():
            if val == 'v':
                check_pt = south_gen(*coord, max_y)
                if map[check_pt] == '.':
                    next_map[check_pt] = 'v'
                    next_map[coord] = '.'
                    seacucumber_moved = True

        if seacucumber_moved:
            map = next_map
        else:
            print("NO SEACUCUMBERS MOVED")
            break
        print_(map, steps)
    return steps


def part2(input: str) -> int:
    return 0


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
