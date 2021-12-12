from __future__ import annotations

import argparse
import os.path
import collections
from typing import List, Tuple, Generator, Set, Dict

from support import timing

INPUT_GITHUB = os.path.join("data", "2021", "day12_github.txt")
INPUT_GOOGLE = os.path.join("data", "2021", "day12_google.txt")
INPUT_REDDIT = os.path.join("data", "2021", "day12_reddit.txt")
INPUT_TWITTER = os.path.join("data", "2021", "day12_twitter.txt")
INPUT_S_SMALL = """\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
"""
INPUT_S_LARGE = """\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
"""

Map = List[str]


def parse(input: str) -> Map:
    map: Map = []
    for index, line in enumerate(input.splitlines()):
        map.append(line)
    return map


def solve(puzzle_input: str):
    solution1 = part1(puzzle_input)
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


def find_paths_1(caves) -> Set:
    paths = set()

    to_process: List[Tuple[str, ...]] = [('start',)]
    while to_process:
        path = to_process.pop()

        # if the last node is the end, add to final set
        if path[-1] == 'end':
            paths.add(path)
            continue

        for next_node in caves[path[-1]]:
            if next_node.isupper() or next_node not in path:
                to_process.append((*path, next_node))

    return paths


def find_paths_2(caves) -> Set:
    paths = set()

    to_process: List[Tuple[Tuple[str, ...], bool]] = [(('start',), False)]
    while to_process:
        path, second_small_cave_visit = to_process.pop()

        # if the last node is the end, add to final set
        if path[-1] == 'end':
            paths.add(path)
            continue

        for next_node in caves[path[-1]]:
            if next_node == 'start':
                continue
            if next_node.isupper() or next_node not in path:
                to_process.append(
                    ((*path, next_node), second_small_cave_visit))
            elif second_small_cave_visit is False and path.count(next_node) == 1:
                to_process.append(((*path, next_node), True))

    return paths


def part1(input: str) -> int:

    caves: Dict = collections.defaultdict(set)

    lines = input.splitlines()
    for line in lines:
        src, dest = line.split('-')
        caves[src].add(dest)
        caves[dest].add(src)

    paths = find_paths_1(caves)

    return len(paths)


def part2(input: str):
    caves: Dict = collections.defaultdict(set)

    lines = input.splitlines()
    for line in lines:
        src, dest = line.split('-')
        caves[src].add(dest)
        caves[dest].add(src)

    paths = find_paths_2(caves)

    return len(paths)


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
