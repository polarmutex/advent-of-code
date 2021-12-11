from __future__ import annotations

import argparse
import os.path
import collections
from typing import List, Tuple, Generator, Set

from support import timing

INPUT_GITHUB = os.path.join("data", "2021", "day11_github.txt")
INPUT_GOOGLE = os.path.join("data", "2021", "day11_google.txt")
INPUT_REDDIT = os.path.join("data", "2021", "day11_reddit.txt")
INPUT_TWITTER = os.path.join("data", "2021", "day11_twitter.txt")
INPUT_S = """\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"""

Map = List[str]


def parse(input: str) -> Map:
    map: Map = []
    for index, line in enumerate(input.splitlines()):
        map.append(line)
    return map


def solve(puzzle_input: str):
    solution1 = part1(puzzle_input, 100)
    solution2 = part2(puzzle_input)
    return solution1, solution2


def adjacent(y: int, x: int) -> Generator[Tuple[int, int], None, None]:
    yield y+1, x+1
    yield y+1, x
    yield y+1, x-1
    yield y, x+1
    yield y, x-1
    yield y-1, x+1
    yield y-1, x
    yield y-1, x-1


def part1(input: str) -> int:

    coords = collections.defaultdict(lambda: 0)

    lines = input.splitlines()
    for y, line in enumerate(lines):
        for x, c in enumerate(line):
            coords[(y, x)] = int(c)

    flashes: int = 0
    for _ in range(100):
        to_process: collections.deque[tuple[int, int]] = collections.deque()
        # First, the energy level of each octopus increases by 1.
        for pt, n in tuple(coords.items()):
            coords[pt] += 1
            if coords[pt] > 9:
                to_process.append(pt)

        # any octopus with an energy level greater than 9 flashes.
        # This increases the energy level of all adjacent octopuses by 1,
        while to_process:
            pt = to_process.pop()
            if coords[pt] == 0:
                # already flashed
                continue
            coords[pt] = 0
            flashes += 1
            for adj_pt in adjacent(*pt):
                if adj_pt in coords and coords[adj_pt] != 0:
                    coords[adj_pt] += 1
                    if coords[adj_pt] > 9:
                        to_process.append(adj_pt)

    return flashes


def part2(input: str):
    coords = collections.defaultdict(lambda: 0)

    lines = input.splitlines()
    for y, line in enumerate(lines):
        for x, c in enumerate(line):
            coords[(y, x)] = int(c)

    step: int = 0
    while(True):
        step += 1
        flashes: int = 0
        to_process: collections.deque[tuple[int, int]] = collections.deque()
        # First, the energy level of each octopus increases by 1.
        for pt, n in tuple(coords.items()):
            coords[pt] += 1
            if coords[pt] > 9:
                to_process.append(pt)

        # any octopus with an energy level greater than 9 flashes.
        # This increases the energy level of all adjacent octopuses by 1,
        while to_process:
            pt = to_process.pop()
            if coords[pt] == 0:
                # already flashed
                continue
            coords[pt] = 0
            flashes += 1
            for adj_pt in adjacent(*pt):
                if adj_pt in coords and coords[adj_pt] != 0:
                    coords[adj_pt] += 1
                    if coords[adj_pt] > 9:
                        to_process.append(adj_pt)
        if flashes == len(coords):
            break

    return step


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
