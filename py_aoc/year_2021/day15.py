from __future__ import annotations

import argparse
import os.path
import collections
from collections import Counter
from typing import List, Tuple, Generator, Set, Dict
import heapq

from support import timing

INPUT_GITHUB = os.path.join("data", "2021", "day15_github.txt")
INPUT_GOOGLE = os.path.join("data", "2021", "day15_google.txt")
INPUT_REDDIT = os.path.join("data", "2021", "day15_reddit.txt")
INPUT_TWITTER = os.path.join("data", "2021", "day15_twitter.txt")
INPUT_S = """\
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
"""

PolymerTemplate = str
Point = Tuple[int, int]


def adjacent(point: Point) -> Generator[tuple[int, int], None, None]:
    y = point[0]
    x = point[1]
    yield y, x + 1
    yield y, x - 1
    yield y - 1, x
    yield y + 1, x


def parse(input: str):
    map = []
    for index, line in enumerate(input.splitlines()):
        map.append(line)
    return map


def solve(puzzle_input: str):
    solution1 = part1(puzzle_input)
    solution2 = part2(puzzle_input)
    return solution1, solution2


def dijkstra_algorithm(coords: Dict[Point, int]) -> int:

    max_x, max_y = max(coords)

    shortest_path: Dict[Point, int] = {}
    priority_queue: List[Tuple[int, Point]] = [(0, (0, 0))]
    visited = set()

    while priority_queue:
        weight, node = heapq.heappop(priority_queue)

        if node not in visited:
            visited.add(node)
            if node == (max_x, max_y):
                return weight

            for point in adjacent(node):
                if point not in coords:
                    continue

                if point in visited:
                    continue
                prev = shortest_path[point] if point in shortest_path else None
                next = weight + coords[point]
                if prev is None or next < prev:
                    shortest_path[point] = next
                    heapq.heappush(priority_queue, (next, point))

    raise AssertionError('DID NOT FIND A PATH')


def part1(input: str) -> int:
    coords: Dict[Point, int] = {}
    lines = input.splitlines()

    for y, line in enumerate(lines):
        for x, c in enumerate(line):
            coords[(y, x)] = int(c)

    return dijkstra_algorithm(coords)


def part2(input: str):
    coords: Dict[Point, int] = {}
    lines = input.splitlines()

    for my in range(5):
        for mx in range(5):

            for y, line in enumerate(lines):
                for x, c in enumerate(line):
                    val = (int(c) + my + mx)
                    if val > 9:
                        val = val % 10 + 1
                    xx = x + (mx * len(line))
                    yy = y + (my * len(lines))
                    coords[(yy, xx)] = val

    return dijkstra_algorithm(coords)


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
